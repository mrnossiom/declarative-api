pub type PResult<T> = Result<T, PError>;

// #[derive(thiserror::Error)]
#[derive(Debug)]
pub struct PError {
	message: String,
}

impl PError {
	pub(crate) const fn new(message: String) -> Self {
		Self { message }
	}

	fn report(&self) {
		eprintln!("Error: {}", self.message);
	}
}
