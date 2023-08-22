pub type PResult<T> = Result<T, PError>;

// #[derive(thiserror::Error)]
#[derive(Debug)]
pub struct PError {
	message: String,
}

impl PError {
	fn report(&self) {
		eprintln!("Error: {}", self.message);
	}
}
