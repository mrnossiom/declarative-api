pub type PResult<T> = Result<T, PError>;

pub struct PError {
	message: String,
}

impl PError {
	fn report(&self) {
		eprintln!("Error: {}", self.message);
	}
}
