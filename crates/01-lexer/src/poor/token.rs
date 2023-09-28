use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
	pub kind: TokenKind,
	pub length: u32,
}

impl Token {
	pub(crate) const fn new(kind: TokenKind, length: u32) -> Self {
		Self { kind, length }
	}
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.kind)?;

		if !self.kind.is_single_char() {
			write!(f, " ({})", self.length)?;
		};

		Ok(())
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
	// IDEA: maybe multiline
	LineComment(Option<DocStyle>),

	/// Any whitespace character.
	Whitespace,

	/// An identifier
	Ident,

	/// Like the above, but containing invalid unicode codepoints.
	InvalidIdent,

	/// See [LiteralKind] for more details.
	Literal(LiteralKind),

	// One-char tokens:
	/// ";"
	Semi,
	/// ","
	Comma,
	/// "."
	Dot,

	/// `(`
	OpenParenthesis,
	/// `)`
	CloseParenthesis,
	/// `{`
	OpenBrace,
	/// `}`
	CloseBrace,
	/// `[`
	OpenBracket,
	/// `]`
	CloseBracket,

	/// "@"
	At,
	/// "#"
	Pound,
	/// "~"
	Tilde,
	/// "?"
	Question,
	/// ":"
	Colon,
	/// "$"
	Dollar,
	/// "="
	Eq,
	/// "!"
	Bang,

	/// "<"
	Lt,
	/// ">"
	Gt,
	/// "-"
	Minus,
	/// "&"
	And,
	/// "|"
	Or,
	/// "+"
	Plus,
	/// "*"
	Star,
	/// "/"
	Slash,
	/// "^"
	Caret,
	/// "%"
	Percent,

	/// Unknown token, not expected by the lexer, e.g. "№"
	Unknown,

	/// End of input.
	Eof,
}

impl TokenKind {
	#[must_use]
	pub const fn is_whitespace(&self) -> bool {
		matches!(self, Self::Whitespace)
	}

	pub(crate) const fn is_eof(&self) -> bool {
		matches!(self, Self::Eof)
	}

	pub(crate) const fn is_single_char(&self) -> bool {
		!matches!(
			self,
			Self::LineComment(_)
				| Self::Whitespace
				| Self::Ident | Self::InvalidIdent
				| Self::Literal(_)
				| Self::Unknown
		)
	}
}

impl fmt::Display for TokenKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::LineComment(None) => write!(f, "a line comment"),
			Self::LineComment(Some(DocStyle::Inner)) => write!(f, "an inner line comment"),
			Self::LineComment(Some(DocStyle::Outer)) => write!(f, "an outer line comment"),
			Self::Whitespace => write!(f, "a whitespace"),

			Self::Ident => write!(f, "an ident"),
			Self::InvalidIdent => write!(f, "an invalid ident"),
			Self::Literal(LiteralKind::Number) => write!(f, "a literal number"),
			Self::Literal(LiteralKind::Str { terminated: true }) => write!(f, "a string literal"),
			Self::Literal(LiteralKind::Str { terminated: false }) => {
				write!(f, "a non-terminated string literal")
			}

			Self::Semi => write!(f, "a semi `;`"),
			Self::Comma => write!(f, "a comma `,`"),
			Self::Dot => write!(f, "a dot `.`"),

			Self::OpenParenthesis => write!(f, "an opening paren `(`"),
			Self::CloseParenthesis => write!(f, "a closing paren `)`"),
			Self::OpenBrace => write!(f, "an opening brace `{{`"),
			Self::CloseBrace => write!(f, "a closing brace `}}`"),
			Self::OpenBracket => write!(f, "an opening bracket `[`"),
			Self::CloseBracket => write!(f, "a closing bracket `]`"),

			Self::At => write!(f, "an at sign `@`"),
			Self::Pound => write!(f, "a pound sign `#`"),
			Self::Tilde => write!(f, "a tilde `~`"),
			Self::Question => write!(f, "a question mark `?`"),
			Self::Colon => write!(f, "a colon `:`"),
			Self::Dollar => write!(f, "a dollar sign `$`"),
			Self::Eq => write!(f, "an equal sign `=`"),
			Self::Bang => write!(f, "a bang sign `!`"),

			Self::Lt => write!(f, ">"),
			Self::Gt => write!(f, "<"),
			Self::Minus => write!(f, "-"),
			Self::And => write!(f, "&"),
			Self::Or => write!(f, "|"),
			Self::Plus => write!(f, "+"),
			Self::Star => write!(f, "*"),
			Self::Slash => write!(f, "/"),
			Self::Caret => write!(f, "^"),
			Self::Percent => write!(f, "%"),

			Self::Unknown => write!(f, "an unknown token"),

			Self::Eof => write!(f, "an end of file token"),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralKind {
	/// "12_u8", "0o100", "0b120i99", "1f32".
	Number,
	/// ""abc"", ""abc"
	Str { terminated: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocStyle {
	// `##!` they document inside of scopes
	Inner,
	// `##` they document the item they embrace
	Outer,
}
