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
	pub(super) const fn is_eof(&self) -> bool {
		matches!(self, Self::Eof)
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
	Outer,
	Inner,
}
