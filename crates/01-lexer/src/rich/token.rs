use derive_more::AsRef;
use session::{Ident, Span, Symbol};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, AsRef)]
pub struct Token {
	pub kind: TokenKind,
	#[as_ref]
	pub span: Span,
}

impl From<Token> for Span {
	fn from(value: Token) -> Self {
		value.span
	}
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if f.alternate() {
			write!(f, "{} ({})", self.kind, self.span)
		} else {
			write!(f, "{}", self.kind)
		}
	}
}

impl Token {
	pub const DUMMY: Self = Self {
		kind: TokenKind::Eof,
		span: Span::DUMMY,
	};

	pub(crate) const fn new(kind: TokenKind, span: Span) -> Self {
		Self { kind, span }
	}

	#[must_use]
	pub const fn ident(&self) -> Option<Ident> {
		match &self.kind {
			TokenKind::Ident(sym) => Some(Ident::new(*sym, self.span)),
			_ => None,
		}
	}

	#[must_use]
	pub fn is_keyword(&self, kw: Symbol) -> bool {
		self.ident().map_or(false, |id| id.symbol == kw)
	}

	#[must_use]
	pub const fn is_open_delim(&self) -> bool {
		matches!(&self.kind, TokenKind::OpenDelim(_))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
	// IDEA: maybe multiline
	DocComment(DocStyle, Symbol),

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

impl fmt::Display for TokenKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// the output should match with the following sentence
		// found {token}

		match self {
			Self::DocComment(style, sym) => write!(
				f,
				"an {} doc comment `{sym}`",
				match style {
					DocStyle::Outer => "outer",
					DocStyle::Inner => "inner",
				}
			),

			Self::Ident(sym) => write!(f, "an ident `{sym}`"),
			Self::InvalidIdent => write!(f, "an invalid ident"),
			Self::Literal(LiteralKind::Bool, _sym) => {
				unreachable!("bool literal doesn't exist in lexer")
			}
			Self::Literal(LiteralKind::Number, sym) => write!(f, "a literal `{sym}`"),
			Self::Literal(LiteralKind::Str, sym) => write!(f, r#"a literal `"{sym}"`"#),

			Self::Semi => write!(f, "a semi `;`"),
			Self::Comma => write!(f, "a comma `,`"),
			Self::Dot => write!(f, "a dot `.`"),
			Self::OpenDelim(Delimiter::Parenthesis) => write!(f, "an opening paren `(`"),
			Self::CloseDelim(Delimiter::Parenthesis) => write!(f, "a closing paren `)`"),
			Self::OpenDelim(Delimiter::Brace) => write!(f, "an opening brace `{{`"),
			Self::CloseDelim(Delimiter::Brace) => write!(f, "a closing brace `}}`"),
			Self::OpenDelim(Delimiter::Bracket) => write!(f, "an opening bracket `[`"),
			Self::CloseDelim(Delimiter::Bracket) => write!(f, "a closing bracket `]`"),

			Self::At => write!(f, "an at sign `@`"),
			Self::Pound => write!(f, "a pound sign `#`"),
			Self::Tilde => write!(f, "a tilde `~`"),
			Self::Question => write!(f, "a question mark `?`"),
			Self::Colon => write!(f, "a colon `:`"),
			Self::Dollar => write!(f, "a dollar sign `$`"),
			Self::Eq => write!(f, "an equal sign `=`"),
			Self::Bang => write!(f, "a bang sign `!`"),

			Self::Op(op) => write!(f, "an operator `{op}`"),

			Self::Unknown => write!(f, "an unknown token"),

			Self::Eof => write!(f, "an end of file token"),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralKind {
	/// AST only, represent  `true` or `false`
	Bool,

	/// ""abc"", ""abc"
	Str,
	/// "12_u8", "0o100", "0b120i99", "1f32".
	Number,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Delimiter {
	/// `( ... )`
	Parenthesis,
	/// `{ ... }`
	Brace,
	/// `[ ... ]`
	Bracket,
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

impl fmt::Display for OpKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
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
		}
	}
}

/// An attribute of these forms
///
/// `@name(<tokens>)`
/// `@name{<tokens>}`
/// `@name[<tokens>]`
/// `@name: <token>`
///
/// or an outer doc comment
///
/// `## doc comment`
///
/// with an optional `!` after the `@` or the `##` to signify inner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocStyle {
	// `##!` they document inside of scopes
	Inner,
	// `##` they document the item they embrace
	Outer,
}
