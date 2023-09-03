use miette::{MietteError, SourceCode, SourceSpan, SpanContents};
use parking_lot::RwLock;
use std::{
	collections::{hash_map::DefaultHasher, HashMap},
	fmt, fs,
	hash::{Hash, Hasher},
	io,
	ops::{Add, AddAssign, Deref, Sub},
	path::{Path, PathBuf},
	rc::Rc,
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
	pub fn load_file(&self, path: &Path) -> io::Result<Rc<SourceFile>> {
		let filename = FileName::new_real(path.to_owned());
		let source = fs::read_to_string(path)?;

		Ok(self.new_source_file(filename, source))
	}

	#[must_use]
	pub fn add_source(&self, source: String) -> Rc<SourceFile> {
		let filename = FileName::new_anon(&source);

		self.new_source_file(filename, source)
	}

	#[must_use]
	fn new_source_file(&self, filename: FileName, source: String) -> Rc<SourceFile> {
		self.try_new_source_file(filename, source).unwrap()
	}

	fn try_new_source_file(
		&self,
		filename: FileName,
		source: String,
	) -> Result<Rc<SourceFile>, OffsetOverflowError> {
		let file_id = SourceFileId::new(&filename);

		let source_file = match self.source_file_by_id(&file_id) {
			Some(sf) => sf,
			None => {
				let start_pos = self.allocate_space(source.len())?;

				let source_file = Rc::new(SourceFile::new(filename, source, start_pos));

				debug_assert_eq!(SourceFileId::from_source_file(&source_file), file_id);

				let mut files = self.files.write();

				files.sources.push(source_file.clone());
				files.file_id_to_source.insert(file_id, source_file.clone());

				source_file
			}
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
	pub fn lookup_source_file_and_relative_pos(&self, pos: BytePos) -> (Rc<SourceFile>, BytePos) {
		let index = self.lookup_source_file_index(pos);
		let sf = self.files.read().sources[index].clone();
		let offset = pos - sf.start_pos;
		(sf, offset)
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

// TODO: move elsewhere
#[derive(Debug, Default)]
struct OffsetOverflowError;

#[derive(Debug, Default)]
pub struct SourceMapFiles {
	pub sources: MonotonicVec<Rc<SourceFile>>,
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
		let end_pos = start_pos + source.len();

		Self {
			name,
			// FIXME(mrnossiom): Could be Rc but needs to be Arc for SourceCode impl
			source: Arc::new(source),
			source_hash,
			start_pos,
			end_pos,
		}
	}
}

impl SourceCode for &SourceFile {
	fn read_span<'a>(
		&'a self,
		span: &SourceSpan,
		context_lines_before: usize,
		context_lines_after: usize,
	) -> Result<Box<dyn SpanContents<'a> + 'a>, MietteError> {
		let source = self
			.source
			.read_span(span, context_lines_before, context_lines_after)?;

		Ok(source)
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BytePos(pub u32);

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
		self.0 += rhs.0
	}
}

impl Sub for BytePos {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self(self.0 - rhs.0)
	}
}

impl Add<usize> for BytePos {
	type Output = Self;

	fn add(self, rhs: usize) -> Self::Output {
		Self(self.0 + rhs as u32)
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceFileId(u64);

impl SourceFileId {
	// TODO: string should be filename
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
	fn new_real(path: PathBuf) -> Self {
		Self::Real(path)
	}

	fn new_anon(source: &str) -> Self {
		let mut hasher = DefaultHasher::new();
		source.hash(&mut hasher);
		Self::Anon(hasher.finish())
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
		self.0.push(value)
	}
}

impl<T> Deref for MonotonicVec<T> {
	type Target = Vec<T>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
