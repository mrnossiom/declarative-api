use crate::{span::Span, symbols::Symbol};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
	pub kind: TokenKind,
	pub span: Span,
}

impl Token {
	pub(crate) const fn new(kind: TokenKind, span: Span) -> Self {
		Self { kind, span }
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
	// IDEA: maybe multiline
	Comment(AttrStyle, Symbol),

	/// An identifier
	Ident(Symbol),

	/// Like the above, but containing invalid unicode codepoints.
	InvalidIdent,

	/// See [LiteralKind] for more details.
	Literal(LiteralKind, Symbol),

	// One-char tokens:
	/// ";"
	Semi,
	/// ","
	Comma,
	/// "."
	Dot,

	/// An opening delimiter (e.g., `{`).
	OpenDelim(Delimiter),
	/// A closing delimiter (e.g., `}`).
	CloseDelim(Delimiter),

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

	Op(OpKind),

	/// Unknown token, not expected by the lexer, e.g. "№"
	Unknown,

	/// End of input.
	Eof,
}

impl TokenKind {
	#[must_use]
	pub const fn is_eof(&self) -> bool {
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Delimiter {
	/// `( ... )`
	Parenthesis,
	/// `{ ... }`
	Brace,
	/// `[ ... ]`
	Bracket,
	/// `Ø ... Ø`
	/// An invisible delimiter, that may, for example, appear around tokens coming from a
	/// "macro variable" `$var`. It is important to preserve operator priorities in cases like
	/// `$var * 3` where `$var` is `1 + 2`.
	/// Invisible delimiters might not survive roundtrip of a token stream through a string.
	Invisible,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum OpKind {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttrStyle {
	Outer,
	Inner,
	Inline,
}
