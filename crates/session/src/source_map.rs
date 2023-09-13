use ariadne::{Cache, Source};
use parking_lot::RwLock;
use std::{
	cell::RefCell,
	collections::{hash_map::DefaultHasher, HashMap},
	fmt, fs,
	hash::{Hash, Hasher},
	io, mem,
	ops::{Add, AddAssign, Deref, DerefMut, Sub},
	path::{Path, PathBuf},
	rc::Rc,
	sync::atomic::{AtomicU32, Ordering},
};

thread_local! {
	static SOURCE_MAP: RefCell<Option<Rc<SourceMap>>> = RefCell::<Option<Rc<SourceMap>>>::default();
}

// TODO: lame name, find another
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
			let a_src = ariadne::Source::from(source.clone());
			let source_file = Rc::new(SourceFile::new(filename, source, start_pos));

			debug_assert_eq!(SourceFileId::from_source_file(&source_file), file_id);

			let mut files = self.files.write();

			files.sources.push(source_file.clone());
			files.file_id_to_source.insert(file_id, source_file.clone());

			// Specific to ariadne
			files.a_sources.push(a_src);

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
		let source_file = &self.0.files.read().a_sources[id];

		// TODO: check safety
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
	pub a_sources: monotonic::FilesVec<ariadne::Source>,
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

	/// The start position of this source in the `SourceMap`.
	pub start_pos: BytePos,
	/// The end position of this source in the `SourceMap`.
	pub end_pos: BytePos,
}

impl SourceFile {
	fn new(name: FileName, source: String, start_pos: BytePos) -> Self {
		let source_hash = SourceFileHash::new(&source);
		let end_pos = start_pos + BytePos(source.len() as u32);

		Self {
			name,
			source: Rc::new(source),
			source_hash,
			start_pos,
			end_pos,
		}
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BytePos(pub u32);

impl BytePos {
	#[must_use]
	pub const fn as_usize(self) -> usize {
		self.0 as usize
	}
}

impl fmt::Display for BytePos {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&self.0, f)
	}
}

impl Add for BytePos {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 + rhs.0)
	}
}

impl AddAssign for BytePos {
	fn add_assign(&mut self, rhs: Self) {
		self.0 += rhs.0;
	}
}

impl Sub for BytePos {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self(self.0 - rhs.0)
	}
}

impl Deref for BytePos {
	type Target = u32;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl DerefMut for BytePos {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
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

pub use monotonic::{FileIdx, FilesVec};

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
