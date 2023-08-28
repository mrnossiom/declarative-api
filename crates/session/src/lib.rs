/// Represents the data associated with a compilation session.
#[derive(Debug, Default)]
pub struct Session {
	parse: ParseSession,
}

/// Info about a parsing session.
#[derive(Debug, Default)]
pub struct ParseSession {}
