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

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.kind,)?;

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

impl Display for TokenKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::LineComment(None) => write!(f, "line comment"),
			Self::LineComment(Some(DocStyle::Inner)) => write!(f, "inner line comment"),
			Self::LineComment(Some(DocStyle::Outer)) => write!(f, "outer line comment"),
			Self::Whitespace => write!(f, "whitespace"),

			Self::Ident => write!(f, "ident"),
			Self::InvalidIdent => write!(f, "invalid ident"),
			Self::Literal(LiteralKind::Number) => write!(f, "number literal"),
			Self::Literal(LiteralKind::Str { terminated: true }) => write!(f, "string literal"),
			Self::Literal(LiteralKind::Str { terminated: false }) => {
				write!(f, "non-terminated string literal")
			}

			Self::Semi => write!(f, ";"),
			Self::Comma => write!(f, ","),
			Self::Dot => write!(f, "."),

			Self::OpenParenthesis => write!(f, "("),
			Self::CloseParenthesis => write!(f, ")"),
			Self::OpenBrace => write!(f, "{{"),
			Self::CloseBrace => write!(f, "}}"),
			Self::OpenBracket => write!(f, "["),
			Self::CloseBracket => write!(f, "]"),

			Self::At => write!(f, "@"),
			Self::Pound => write!(f, "#"),
			Self::Tilde => write!(f, "~"),
			Self::Question => write!(f, "?"),
			Self::Colon => write!(f, ":"),
			Self::Dollar => write!(f, "$"),
			Self::Eq => write!(f, "="),
			Self::Bang => write!(f, "!"),

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

			Self::Unknown => write!(f, "unknown"),

			Self::Eof => write!(f, "<EOF>"),
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
pub enum AttrStyle {
	Outer,
	Inner,
	Inline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocStyle {
	// `##!` they document inside of scopes
	Inner,
	// `##` they document the item they embrace
	Outer,
}
