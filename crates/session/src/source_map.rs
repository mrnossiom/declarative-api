use crate::Span;
use miette::{MietteError, MietteSpanContents, SourceCode, SourceSpan, SpanContents};
use parking_lot::RwLock;
use std::{
	collections::{hash_map::DefaultHasher, HashMap, VecDeque},
	fmt, fs,
	hash::{Hash, Hasher},
	io,
	mem::transmute,
	ops::{Add, AddAssign, Deref, DerefMut, Sub},
	path::{Path, PathBuf},
	sync::{
		atomic::{AtomicU32, Ordering},
		Arc,
	},
};

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
	pub fn load_file(&self, path: &Path) -> io::Result<Arc<SourceFile>> {
		let filename = FileName::new_real(path.to_owned());
		let source = fs::read_to_string(path)?;

		Ok(self.new_source_file(filename, source))
	}

	#[must_use]
	pub fn load_anon(&self, source: String) -> Arc<SourceFile> {
		let filename = FileName::new_anon(&source);

		self.new_source_file(filename, source)
	}

	#[must_use]
	fn new_source_file(&self, filename: FileName, source: String) -> Arc<SourceFile> {
		self.try_new_source_file(filename, source)
			.expect("SourceMap can only contain up to 4GiB of sources, this limit seems to have been exceeded")
	}

	fn try_new_source_file(
		&self,
		filename: FileName,
		source: String,
	) -> Result<Arc<SourceFile>, OffsetOverflowError> {
		let file_id = SourceFileId::new(&filename);

		let source_file = if let Some(sf) = self.source_file_by_id(&file_id) {
			sf
		} else {
			let start_pos = self.allocate_space(source.len())?;

			let source_file = Arc::new(SourceFile::new(filename, source, start_pos));

			debug_assert_eq!(SourceFileId::from_source_file(&source_file), file_id);

			let mut files = self.files.write();

			files.sources.push(source_file.clone());
			files.file_id_to_source.insert(file_id, source_file.clone());

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

	fn source_file_by_id(&self, id: &SourceFileId) -> Option<Arc<SourceFile>> {
		self.files.read().file_id_to_source.get(id).cloned()
	}
}

impl SourceMap {
	pub fn lookup_source_file(&self, pos: BytePos) -> Arc<SourceFile> {
		let index = self.lookup_source_file_index(pos);
		self.files.read().sources[index].clone()
	}

	fn lookup_source_file_index(&self, pos: BytePos) -> usize {
		self.files
			.read()
			.sources
			.0
			.binary_search_by_key(&pos, |sf| sf.start_pos)
			.unwrap_or_else(|p| p - 1)
	}
}

impl SourceCode for SourceMap {
	fn read_span<'a>(
		&'a self,
		span: &SourceSpan,
		context_lines_before: usize,
		context_lines_after: usize,
	) -> Result<Box<dyn SpanContents<'a> + 'a>, MietteError> {
		let span = Span::from(span);

		let sf = self.lookup_source_file(span.start);

		let span = sf.read_span(span, context_lines_before, context_lines_after)?;

		// TODO: check safety
		let span = unsafe { transmute::<MietteSpanContents<'_>, MietteSpanContents<'a>>(span) };

		Ok(Box::new(span))
	}
}

// TODO: move elsewhere
#[derive(Debug, Default)]
struct OffsetOverflowError;

#[derive(Debug, Default)]
pub struct SourceMapFiles {
	// Could be Rc but needs to be Arc for SourceCode impl
	pub sources: MonotonicVec<Arc<SourceFile>>,
	file_id_to_source: HashMap<SourceFileId, Arc<SourceFile>>,
}

/// A single source in the [`SourceMap`].
#[derive(Debug)]
pub struct SourceFile {
	/// The name of the file that the source came from. Source that doesn't
	/// originate from files has names between angle brackets by convention
	/// (e.g., `<anon>`).
	pub name: FileName,
	/// The complete source code.
	pub source: Arc<String>,
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
			// FIXME(mrnossiom): Could be Rc but needs to be Arc for SourceCode impl
			source: Arc::new(source),
			source_hash,
			start_pos,
			end_pos,
		}
	}

	fn read_span(
		&self,
		global_span: Span,
		context_lines_before: usize,
		context_lines_after: usize,
	) -> Result<MietteSpanContents, MietteError> {
		let input = self.source.as_bytes();
		let span = global_span.relative_to(self);

		let mut offset: usize = 0;
		let mut line_count: usize = 0;

		let mut start_line: usize = 0;
		let mut start_column: usize = 0;

		let mut current_line_start: usize = 0;
		let mut before_lines_starts: VecDeque<usize> = VecDeque::new();

		let mut end_lines: usize = 0;

		let mut post_span = false;
		let mut post_span_got_newline = false;

		let mut iter = input.iter().copied().peekable();

		while let Some(char) = iter.next() {
			if matches!(char, b'\r' | b'\n') {
				// On newline increment
				line_count += 1;

				// Offset by one byte if we're on a CRLF line ending.
				if char == b'\r' && iter.next_if_eq(&b'\n').is_some() {
					offset += 1;
				}

				// If before the start of the span.
				if offset < span.start.as_usize() {
					// Reset the column start.
					start_column = 0;

					// Register a newline that is before the span.
					before_lines_starts.push_back(current_line_start);

					// If we've collected more lines than we need, pop the first
					if before_lines_starts.len() > context_lines_before {
						before_lines_starts.pop_front();

						// Track the numbers of lines skipped to show the first line of the source read
						start_line += 1;
					}
				} else if offset >= (span.start + span.len()).as_usize().saturating_sub(1) {
					// We're after the end of the span, but haven't necessarily
					// started collecting end lines yet (we might still be
					// collecting context lines).
					if post_span {
						start_column = 0;
						if post_span_got_newline {
							end_lines += 1;
						} else {
							post_span_got_newline = true;
						}
						if end_lines >= context_lines_after {
							offset += 1;
							break;
						}
					}
				}

				current_line_start = offset + 1;
			} else if offset < span.start.as_usize() {
				start_column += 1;
			}

			if offset >= (span.start + span.len()).as_usize().saturating_sub(1) {
				post_span = true;

				if end_lines >= context_lines_after {
					offset += 1;
					break;
				}
			}

			offset += 1;
		}

		if (span.start + span.len()).as_usize().saturating_sub(1) > offset {
			return Err(MietteError::OutOfBounds);
		}

		let starting_offset = before_lines_starts.front().copied().unwrap_or_else(|| {
			if context_lines_before == 0 {
				span.start.as_usize()
			} else {
				0
			}
		});

		Ok(MietteSpanContents::new_named(
			self.name.to_string(),
			&input[starting_offset..offset],
			(0, 3).into(),
			start_line,
			if context_lines_before == 0 {
				start_column
			} else {
				0
			},
			line_count,
		))
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

#[derive(Debug, Hash)]
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

#[derive(Debug)]
pub struct MonotonicVec<T>(Vec<T>);

impl<T> Default for MonotonicVec<T> {
	fn default() -> Self {
		Self(Vec::default())
	}
}

impl<T> MonotonicVec<T> {
	fn push(&mut self, value: T) {
		self.0.push(value);
	}
}

impl<T> Deref for MonotonicVec<T> {
	type Target = Vec<T>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
