use session::{Ident, Span, Symbol};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
	pub kind: TokenKind,
	pub span: Span,
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
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} ({})", self.kind, self.span)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
	// IDEA: maybe multiline
	DocComment(AttrStyle, Symbol),

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
		match self {
			Self::DocComment(style, sym) => write!(
				f,
				r#"{} doc comment ("{sym}")"#,
				match style {
					AttrStyle::OuterOrInline => "outer",
					AttrStyle::Inner => "inner",
				}
			),

			Self::Ident(sym) => write!(f, r#"ident "{sym}""#),
			Self::InvalidIdent => write!(f, "invalid ident"),
			Self::Literal(LiteralKind::Bool, _sym) => {
				unreachable!("bool literal doesn't exist in lexer")
			}
			Self::Literal(LiteralKind::Number, sym) => write!(f, "lit {sym}"),
			Self::Literal(LiteralKind::Str, sym) => write!(f, r#"lit "{sym}""#),

			Self::Semi => write!(f, ";"),
			Self::Comma => write!(f, ","),
			Self::Dot => write!(f, "."),
			Self::OpenDelim(Delimiter::Parenthesis) => write!(f, "("),
			Self::CloseDelim(Delimiter::Parenthesis) => write!(f, ")"),
			Self::OpenDelim(Delimiter::Brace) => write!(f, "{{"),
			Self::CloseDelim(Delimiter::Brace) => write!(f, "}}"),
			Self::OpenDelim(Delimiter::Bracket) => write!(f, "["),
			Self::CloseDelim(Delimiter::Bracket) => write!(f, "]"),
			Self::OpenDelim(Delimiter::Invisible) => write!(f, "Ø..."),
			Self::CloseDelim(Delimiter::Invisible) => write!(f, "...Ø"),

			Self::At => write!(f, "@"),
			Self::Pound => write!(f, "#"),
			Self::Tilde => write!(f, "~"),
			Self::Question => write!(f, "?"),
			Self::Colon => write!(f, ":"),
			Self::Dollar => write!(f, "$"),
			Self::Eq => write!(f, "="),
			Self::Bang => write!(f, "!"),

			Self::Op(op) => write!(f, "{op}"),

			Self::Unknown => write!(f, "unknown"),

			Self::Eof => write!(f, "<EOF>"),
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
pub enum AttrStyle {
	/// Without a `!` bang
	OuterOrInline,

	/// With a `!` bang
	Inner,
}
