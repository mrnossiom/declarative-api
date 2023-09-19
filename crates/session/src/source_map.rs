use ariadne::{Cache, Source};
use parking_lot::RwLock;
use std::{
	cell::RefCell,
	collections::{hash_map::DefaultHasher, HashMap},
	fmt, fs,
	hash::{Hash, Hasher},
	io, mem,
	path::{Path, PathBuf},
	rc::Rc,
	sync::atomic::{AtomicU32, Ordering},
};

use self::analyse::{analyze_source_file, MultiByteChar, NonNarrowChar};
pub use self::{
	monotonic::{FileIdx, FilesVec},
	pos::{BytePos, CharPos},
};

thread_local! {
	static SOURCE_MAP: RefCell<Option<Rc<SourceMap>>> = RefCell::<Option<Rc<SourceMap>>>::default();
}

#[inline]
pub fn with_source_map<R, F>(f: F) -> Option<R>
where
	F: FnOnce(&Rc<SourceMap>) -> R,
{
	SOURCE_MAP.with(|sm| sm.borrow().as_ref().map(f))
}

pub fn add_source_map_context<T, F: FnOnce() -> T>(source_map: Rc<SourceMap>, f: F) -> T {
	SOURCE_MAP.with(|sm| *sm.borrow_mut() = Some(source_map));
	let value = f();
	SOURCE_MAP.with(|sm| sm.borrow_mut().take());
	value
}

#[derive(Debug, Default)]
pub struct SourceMap {
	/// The address space below this value is currently used by the files in the source map.
	/// Using `u32`s in [`Span`] means that we cannot load more than 4GiB of sources
	used_space: AtomicU32,

	pub files: RwLock<SourceMapFiles>,
}

impl SourceMap {
	/// # Errors
	/// Returns an error if the file cannot be read.
	pub fn load_file(&self, path: &Path) -> io::Result<Rc<SourceFile>> {
		let filename = FileName::new_real(path.to_owned());
		let source = fs::read_to_string(path)?;

		Ok(self.new_source_file(filename, source))
	}

	#[must_use]
	pub fn load_anon(&self, source: String) -> Rc<SourceFile> {
		let filename = FileName::new_anon(&source);

		self.new_source_file(filename, source)
	}

	#[must_use]
	fn new_source_file(&self, filename: FileName, source: String) -> Rc<SourceFile> {
		self.try_new_source_file(filename, source)
			.expect("SourceMap can only contain up to 4GiB of sources, this limit seems to have been exceeded")
	}

	fn try_new_source_file(
		&self,
		filename: FileName,
		source: String,
	) -> Result<Rc<SourceFile>, OffsetOverflowError> {
		let file_id = SourceFileId::new(&filename);

		let source_file = if let Some(sf) = self.source_file_by_id(&file_id) {
			sf
		} else {
			let start_pos = self.allocate_space(source.len())?;

			// TODO: find a way to function without this clone
			let diagnostic_source = ariadne::Source::from(source.clone());
			let source_file = Rc::new(SourceFile::new(filename, source, start_pos));

			// Check we haven't altered in any way the file
			debug_assert_eq!(SourceFileId::from_source_file(&source_file), file_id);

			let mut files = self.files.write();

			files.sources.push(source_file.clone());
			files.file_id_to_source.insert(file_id, source_file.clone());

			// Specific to ariadne
			files.diagnostic_sources.push(diagnostic_source);

			source_file
		};

		Ok(source_file)
	}

	fn allocate_space(&self, size: usize) -> Result<BytePos, OffsetOverflowError> {
		let size = u32::try_from(size).map_err(|_| OffsetOverflowError)?;

		loop {
			let current = self.used_space.load(Ordering::Relaxed);
			let next = current
				.checked_add(size)
				// Add one so there is some space between files. This lets us distinguish
				// positions in the `SourceMap`, even in the presence of zero-length files.
				.and_then(|next| next.checked_add(1))
				.ok_or(OffsetOverflowError)?;

			if self
				.used_space
				.compare_exchange(current, next, Ordering::Relaxed, Ordering::Relaxed)
				.is_ok()
			{
				break Ok(BytePos(current));
			}
		}
	}

	fn source_file_by_id(&self, id: &SourceFileId) -> Option<Rc<SourceFile>> {
		self.files.read().file_id_to_source.get(id).cloned()
	}
}

impl SourceMap {
	pub fn lookup_source_file(&self, pos: BytePos) -> Rc<SourceFile> {
		let index = self.lookup_source_file_index(pos);
		self.files.read().sources[index].clone()
	}

	pub fn lookup_source_file_index(&self, pos: BytePos) -> FileIdx {
		let idx = self
			.files
			.read()
			.sources
			.inner()
			.binary_search_by_key(&pos, |sf| sf.start_pos)
			.unwrap_or_else(|p| p - 1);

		FileIdx::new(idx)
	}

	pub(crate) fn to_cache_hack(&self) -> impl Cache<FileIdx> + '_ {
		SourceMapCacheHack(self)
	}
}

struct SourceMapCacheHack<'a>(&'a SourceMap);

impl<'a> Cache<FileIdx> for SourceMapCacheHack<'a> {
	fn fetch(&mut self, id: &FileIdx) -> Result<&ariadne::Source, Box<dyn fmt::Debug + '_>> {
		let source_file = &self.0.files.read().diagnostic_sources[id];

		// TODO: check safety
		// SAFETY: ?
		let source_file = unsafe { mem::transmute::<&Source, &Source>(source_file) };

		Ok(source_file)
	}

	fn display<'b>(&self, id: &'b FileIdx) -> Option<Box<dyn fmt::Display + 'b>> {
		let name = self.0.files.read().sources[id].name.clone();
		Some(Box::new(name))
	}
}

// TODO: move elsewhere
#[derive(Debug, Default)]
struct OffsetOverflowError;

#[derive(Debug, Default)]
pub struct SourceMapFiles {
	pub sources: monotonic::FilesVec<Rc<SourceFile>>,
	pub diagnostic_sources: monotonic::FilesVec<ariadne::Source>,
	file_id_to_source: HashMap<SourceFileId, Rc<SourceFile>>,
}

/// A single source in the [`SourceMap`].
#[derive(Debug)]
pub struct SourceFile {
	/// The name of the file that the source came from. Source that doesn't
	/// originate from files has names between angle brackets by convention
	/// (e.g., `<anon>`).
	pub name: FileName,
	/// The complete source code.
	pub source: Rc<String>,
	/// The source code's hash.
	pub source_hash: SourceFileHash,

	pub lines: Vec<BytePos>,
	pub multi_bytes_chars: Vec<MultiByteChar>,
	pub non_narrow_chars: Vec<NonNarrowChar>,

	/// The start position of this source in the `SourceMap`.
	pub start_pos: BytePos,
	/// The end position of this source in the `SourceMap`.
	pub end_pos: BytePos,
}

impl SourceFile {
	fn new(name: FileName, source: String, start_pos: BytePos) -> Self {
		let source_hash = SourceFileHash::new(&source);
		let end_pos = start_pos
			+ BytePos(u32::try_from(source.len()).expect("source must be less than 4 GiB"));

		let (lines, multi_bytes_chars, non_narrow_chars) = analyze_source_file(&source, start_pos);

		Self {
			name,
			source: Rc::new(source),
			source_hash,

			lines,
			multi_bytes_chars,
			non_narrow_chars,

			start_pos,
			end_pos,
		}
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceFileId(u64);

impl SourceFileId {
	// TODO: string should be filename
	#[must_use]
	pub fn new(path: &FileName) -> Self {
		let mut hasher = DefaultHasher::new();
		path.hash(&mut hasher);
		Self(hasher.finish())
	}

	fn from_source_file(source_file: &SourceFile) -> Self {
		Self::new(&source_file.name)
	}
}

#[derive(Debug)]
pub struct SourceFileHash(u64);

impl SourceFileHash {
	fn new(source: &str) -> Self {
		let mut hasher = DefaultHasher::new();
		source.hash(&mut hasher);
		Self(hasher.finish())
	}
}

#[derive(Debug, Clone, Hash)]
pub enum FileName {
	/// Real file path
	Real(PathBuf),
	/// Anonymous source for tests or internal use, stores a hash
	Anon(u64),
}

impl FileName {
	const fn new_real(path: PathBuf) -> Self {
		Self::Real(path)
	}

	fn new_anon(source: &str) -> Self {
		let mut hasher = DefaultHasher::new();
		source.hash(&mut hasher);
		Self::Anon(hasher.finish())
	}
}

impl fmt::Display for FileName {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match &self {
			Self::Real(path) => write!(f, "{}", path.display()),
			Self::Anon(hash) => write!(f, "<anon:{hash}>"),
		}
	}
}

mod monotonic {
	use negative_impl::negative_impl;
	use std::ops::Index;

	#[derive(Debug, Clone, PartialEq, Eq)]
	pub struct FileIdx(usize);

	// `FileIdx` refers to thread local indexes
	#[negative_impl]
	impl !Send for FileIdx {}

	impl FileIdx {
		#[must_use]
		pub const fn new(idx: usize) -> Self {
			Self(idx)
		}
	}

	#[derive(Debug)]
	pub struct FilesVec<T>(Vec<T>);

	impl<T> Default for FilesVec<T> {
		fn default() -> Self {
			Self(Vec::default())
		}
	}

	impl<T> FilesVec<T> {
		pub(super) fn push(&mut self, value: T) {
			self.0.push(value);
		}

		pub(super) const fn inner(&self) -> &Vec<T> {
			&self.0
		}
	}

	impl<T> Index<FileIdx> for FilesVec<T> {
		type Output = T;

		fn index(&self, index: FileIdx) -> &Self::Output {
			&self.0[index.0]
		}
	}

	impl<T> Index<&FileIdx> for FilesVec<T> {
		type Output = T;

		fn index(&self, index: &FileIdx) -> &Self::Output {
			&self.0[index.0]
		}
	}
}

mod pos {
	use core::{
		fmt,
		ops::{Add, Sub},
	};

	/// Implements binary operators "&T op U", "T op &U", "&T op &U"
	/// based on "T op U" where T and U are expected to be `Copy`able
	macro_rules! forward_ref_bin_op {
		(impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
			impl<'a> $imp<$u> for &'a $t {
				type Output = <$t as $imp<$u>>::Output;

				#[inline]
				fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
					$imp::$method(*self, other)
				}
			}

			impl<'a> $imp<&'a $u> for $t {
				type Output = <$t as $imp<$u>>::Output;

				#[inline]
				fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
					$imp::$method(self, *other)
				}
			}

			impl<'a, 'b> $imp<&'a $u> for &'b $t {
				type Output = <$t as $imp<$u>>::Output;

				#[inline]
				fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
					$imp::$method(*self, *other)
				}
			}
		};
	}

	macro_rules! impl_pos {
		(
			$(
				$(#[$attr:meta])*
				$vis:vis struct $ident:ident($inner_vis:vis $inner_ty:ty);
			)*
		) => {
			$(
				$(#[$attr])*
				$vis struct $ident($inner_vis $inner_ty);

				impl $ident {
					#[must_use]
					#[inline(always)]
					pub const fn from_usize(n: usize) -> $ident {
						$ident(n as $inner_ty)
					}

					#[must_use]
					#[inline(always)]
					pub const fn to_usize(self) -> usize {
						self.0 as usize
					}

					#[must_use]
					#[inline(always)]
					pub const fn from_u32(n: u32) -> $ident {
						$ident(n as $inner_ty)
					}

					#[must_use]
					#[inline(always)]
					pub const fn to_u32(self) -> u32 {
						self.0 as u32
					}
				}

				impl fmt::Display for $ident {
					fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
						self.0.fmt(f)
					}
				}

				impl core::ops::Add for $ident {
					type Output = $ident;

					#[inline(always)]
					fn add(self, rhs: $ident) -> $ident {
						$ident(self.0 + rhs.0)
					}
				}

				forward_ref_bin_op! { impl Add, add for $ident, $ident }

				impl core::ops::Sub for $ident {
					type Output = $ident;

					#[inline(always)]
					fn sub(self, rhs: $ident) -> $ident {
						$ident(self.0 - rhs.0)
					}
				}

				forward_ref_bin_op! { impl Sub, sub for $ident, $ident }
			)*
		};
	}

	impl_pos! {
		/// A byte offset.
		///
		/// This is used in the
		/// This is kept small because an AST contains a lot of them.
		/// They also the limit the amount of sources that can be imported (≈ 4GiB). Find more information on [`SourceMap::allocate_space`]
		#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
		pub struct BytePos(pub u32);

		/// A character offset.
		///
		/// Because of multibyte UTF-8 characters, a byte offset
		/// is not equivalent to a character offset. The [`SourceMap`] will convert [`BytePos`]
		/// values to `CharPos` values as necessary.
		///
		/// It's a `usize` because it's easier to use with string slices
		#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
		pub struct CharPos(pub usize);
	}
}

mod analyse {
	use crate::BytePos;
	use unicode_width::UnicodeWidthChar;

	/// Identifies an offset of a multi-byte character in a `SourceFile`.
	#[derive(Copy, Clone, Eq, PartialEq, Debug)]
	pub struct MultiByteChar {
		/// The absolute offset of the character in the `SourceMap`.
		pub pos: BytePos,
		/// The number of bytes, `>= 2`.
		pub bytes: u8,
	}

	/// Identifies an offset of a non-narrow character in a `SourceFile`.
	#[derive(Copy, Clone, Eq, PartialEq, Debug)]
	pub enum NonNarrowChar {
		/// Represents a zero-width character.
		ZeroWidth(BytePos),
		/// Represents a wide (full-width) character.
		Wide(BytePos),
		/// Represents a tab character, represented visually with a width of 4 characters.
		Tab(BytePos),
	}

	impl NonNarrowChar {
		fn new(pos: BytePos, width: usize) -> Self {
			match width {
				0 => Self::ZeroWidth(pos),
				2 => Self::Wide(pos),
				4 => Self::Tab(pos),
				_ => panic!("width {width} given for non-narrow character"),
			}
		}

		/// Returns the absolute offset of the character in the `SourceMap`.
		pub const fn pos(self) -> BytePos {
			match self {
				Self::ZeroWidth(p) | Self::Wide(p) | Self::Tab(p) => p,
			}
		}

		/// Returns the width of the character, 0 (zero-width) or 2 (wide).
		pub const fn width(self) -> usize {
			match self {
				Self::ZeroWidth(_) => 0,
				Self::Wide(_) => 2,
				Self::Tab(_) => 4,
			}
		}
	}

	/// Finds all newlines, multi-byte characters, and non-narrow characters in a
	/// [`SourceFile`].
	///
	/// This function will use an SSE2 enhanced implementation if hardware support
	/// is detected at runtime.
	pub fn analyze_source_file(
		src: &str,
		source_file_start_pos: BytePos,
	) -> (Vec<BytePos>, Vec<MultiByteChar>, Vec<NonNarrowChar>) {
		let mut lines = vec![source_file_start_pos];
		let mut multi_byte_chars = vec![];
		let mut non_narrow_chars = vec![];

		analyze_source_file_(
			src,
			source_file_start_pos,
			&mut lines,
			&mut multi_byte_chars,
			&mut non_narrow_chars,
		);

		// The code above optimistically registers a new line *after* each \n
		// it encounters. If that point is already outside the source_file, remove
		// it again.
		if let Some(&last_line_start) = lines.last() {
			let source_file_end = source_file_start_pos + BytePos::from_usize(src.len());
			assert!(source_file_end >= last_line_start);
			if last_line_start == source_file_end {
				lines.pop();
			}
		}

		(lines, multi_byte_chars, non_narrow_chars)
	}

	// `scan_len` determines the number of bytes in `src` to scan. Note that the
	// function can read past `scan_len` if a multi-byte character start within the
	// range but extends past it. The overflow is returned by the function.
	fn analyze_source_file_(
		src: &str,
		output_offset: BytePos,
		lines: &mut Vec<BytePos>,
		multi_byte_chars: &mut Vec<MultiByteChar>,
		non_narrow_chars: &mut Vec<NonNarrowChar>,
	) -> usize {
		let mut i = 0;
		let src_bytes = src.as_bytes();

		while i < src.len() {
			let byte = unsafe {
				// SAFETY: We verified that i < src.len()
				*src_bytes.get_unchecked(i)
			};

			// How much to advance in order to get to the next UTF-8 char in the
			// string.
			let mut char_len = 1;

			if byte < 32 {
				// This is an ASCII control character, it could be one of the cases
				// that are interesting to us.

				let pos = BytePos::from_usize(i) + output_offset;

				match byte {
					b'\n' => lines.push(pos + BytePos(1)),
					b'\t' => non_narrow_chars.push(NonNarrowChar::Tab(pos)),
					_ => non_narrow_chars.push(NonNarrowChar::ZeroWidth(pos)),
				}
			} else if byte >= 127 {
				// slow path: This is either ASCII control character "DEL"
				// or the beginning of a multibyte char. Just decode to `char`.
				let c = src[i..]
					.chars()
					.next()
					.expect("the loop ensures that there is at least one char");

				// Always return a number in 1..=4
				char_len = c.len_utf8();

				let pos = BytePos::from_usize(i) + output_offset;

				if char_len > 1 {
					let mbc = MultiByteChar {
						pos,
						// `len_utf8` returns a number between 1 and 4 inclusive
						#[allow(clippy::cast_possible_truncation)]
						bytes: char_len as u8,
					};
					multi_byte_chars.push(mbc);
				}

				// Assume control characters are zero width.
				let char_width = UnicodeWidthChar::width(c).unwrap_or(0);

				if char_width != 1 {
					non_narrow_chars.push(NonNarrowChar::new(pos, char_width));
				}
			}

			i += char_len;
		}

		i - src.len()
	}
}
