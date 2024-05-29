use crate::{
	poor,
	rich::{errors, Delimiter, DocStyle, LiteralKind, OpKind, Token, TokenKind},
};
use dapic_session::{BytePos, ParseSession, SourceFile, Span, Symbol};
use tracing::instrument;

/// Transforms [`poor::Token`]s that are only relevant when reading the source file at the
/// same time into [`rich::Token`](crate::rich::Token)s that are self-explanatory. The latter doesn't include
/// tokens that don't add information to the generator such as whitespace or comments.
pub struct Enricher<'gctx> {
	session: &'gctx ParseSession<'gctx>,
	source: &'gctx str,
	cursor: poor::Cursor<'gctx>,
	start_pos: BytePos,
	pos: BytePos,
}

impl<'a> Enricher<'a> {
	/// Creates a new [`Enricher`] for the given source, creating in the way
	/// the underlying [`poor::Cursor`] to get tokens to enrich.
	#[must_use]
	pub fn from_source(session: &'a ParseSession<'a>, source: &'a SourceFile) -> Self {
		Self {
			session,
			source: &source.source,
			cursor: poor::Cursor::from_source(&source.source),
			start_pos: source.offset,
			pos: source.offset,
		}
	}

	/// Fetches the next token from the underlying poor lexer and add information
	#[instrument(level = "DEBUG", skip(self))]
	pub fn next_token(&mut self) -> (Token, bool) {
		let mut has_whitespace_before = false;

		loop {
			let token = self.cursor.next_token();
			let start = self.pos;
			self.pos = self.pos + BytePos(token.length);

			let span = Span::from_bounds(start, self.pos);

			let kind = match token.kind {
				poor::TokenKind::Ident => {
					let ident = self.str_from(start);
					TokenKind::Ident(Symbol::intern(ident))
				}
				poor::TokenKind::LineComment(style) => {
					// Skip non-doc comments
					let Some(style) = style else {
						has_whitespace_before = true;
						continue;
					};

					let len = match style {
						DocStyle::Inner => 3, // `##!`
						DocStyle::Outer => 2, // `##`
					};

					let content = self.str_from(start + BytePos(len));

					let (style, sym) = Self::cook_doc_line_comment(content, style);

					TokenKind::DocComment(style, sym)
				}
				poor::TokenKind::Literal(kind) => {
					let (kind, sym) = self.cook_literal(start, self.pos, kind);

					TokenKind::Literal(kind, sym)
				}

				poor::TokenKind::Whitespace => {
					has_whitespace_before = true;
					continue;
				}

				poor::TokenKind::InvalidIdent => {
					self.session.diag.emit(errors::InvalidIdent { span });

					// TODO: enhance recovery by emitting a wrong identifier or helping the user to change it

					has_whitespace_before = true;
					continue;
				}

				poor::TokenKind::Unknown => {
					self.session.diag.emit(errors::Unknown { span });

					has_whitespace_before = true;
					continue;
				}

				poor::TokenKind::Semi => TokenKind::Semi,
				poor::TokenKind::Comma => TokenKind::Comma,
				poor::TokenKind::Dot => TokenKind::Dot,

				poor::TokenKind::OpenParenthesis => TokenKind::OpenDelim(Delimiter::Parenthesis),
				poor::TokenKind::CloseParenthesis => TokenKind::CloseDelim(Delimiter::Parenthesis),
				poor::TokenKind::OpenBrace => TokenKind::OpenDelim(Delimiter::Brace),
				poor::TokenKind::CloseBrace => TokenKind::CloseDelim(Delimiter::Brace),
				poor::TokenKind::OpenBracket => TokenKind::OpenDelim(Delimiter::Bracket),
				poor::TokenKind::CloseBracket => TokenKind::CloseDelim(Delimiter::Bracket),

				poor::TokenKind::At => TokenKind::At,
				poor::TokenKind::Pound => TokenKind::Pound,
				poor::TokenKind::Tilde => TokenKind::Tilde,
				poor::TokenKind::Question => TokenKind::Question,
				poor::TokenKind::Colon => TokenKind::Colon,
				poor::TokenKind::Dollar => TokenKind::Dollar,
				poor::TokenKind::Eq => TokenKind::Eq,
				poor::TokenKind::Bang => TokenKind::Bang,

				poor::TokenKind::Lt => TokenKind::Op(OpKind::Lt),
				poor::TokenKind::Gt => TokenKind::Op(OpKind::Gt),
				poor::TokenKind::Minus => TokenKind::Op(OpKind::Minus),
				poor::TokenKind::And => TokenKind::Op(OpKind::And),
				poor::TokenKind::Or => TokenKind::Op(OpKind::Or),
				poor::TokenKind::Plus => TokenKind::Op(OpKind::Plus),
				poor::TokenKind::Star => TokenKind::Op(OpKind::Star),
				poor::TokenKind::Slash => TokenKind::Op(OpKind::Slash),
				poor::TokenKind::Caret => TokenKind::Op(OpKind::Caret),
				poor::TokenKind::Percent => TokenKind::Op(OpKind::Percent),

				poor::TokenKind::Eof => TokenKind::Eof,
			};

			return (Token::new(kind, span), has_whitespace_before);
		}
	}

	#[instrument(level = "DEBUG")]
	fn cook_doc_line_comment(content: &str, style: DocStyle) -> (DocStyle, Symbol) {
		(style, Symbol::intern(content))
	}

	#[instrument(level = "DEBUG", skip(self))]
	fn cook_literal(
		&mut self,
		start: BytePos,
		end: BytePos,
		kind: poor::LiteralKind,
	) -> (LiteralKind, Symbol) {
		match kind {
			poor::LiteralKind::Str { terminated } => {
				if !terminated {
					// error somehow
				}

				let content = self.str_from_to(start + BytePos(1), end - BytePos(1));
				(LiteralKind::Str, Symbol::intern(content))
			}
			poor::LiteralKind::Number => {
				let content = self.str_from(start);
				(LiteralKind::Number, Symbol::intern(content))
			}
		}
	}

	#[inline]
	fn src_index(&self, pos: BytePos) -> usize {
		(pos - self.start_pos).0 as usize
	}

	/// Slice of the source text from `start` up to but excluding `self.pos`,
	/// meaning the slice does not include the character `self.ch`.
	#[instrument(level = "DEBUG", skip(self))]
	fn str_from(&self, start: BytePos) -> &'a str {
		self.str_from_to(start, self.pos)
	}

	/// Slice of the source text spanning from `start` up to but excluding `end`.
	#[instrument(level = "DEBUG", skip(self))]
	fn str_from_to(&self, start: BytePos, end: BytePos) -> &'a str {
		&self.source[self.src_index(start)..self.src_index(end)]
	}
}

pub struct Iter<'a>(Enricher<'a>);

impl<'a> Iterator for Iter<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		let (token, _) = self.0.next_token();

		if token.kind.is_eof() {
			None
		} else {
			Some(token)
		}
	}
}

impl<'a> IntoIterator for Enricher<'a> {
	type Item = Token;
	type IntoIter = Iter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		Iter(self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::{ATTR, EXAMPLE, URLS};
	use dapic_session::{sym, symbols::attrs, Session};

	macro_rules! tokens {
		($expr:ident, $(($ty:expr, [$lo:literal, $hi:literal])),+) => {
			let sess = Session::default();
			let source = sess.source_map.load_anon($expr.into());
			let psess = sess.parse_sess();
			let mut tokens = Enricher::from_source(&psess, &source).into_iter();

			$(
				assert_eq!(
					tokens.next(),
					Some(Token::new($ty, dapic_session::sp!($lo, $hi)))
				);
			)+
			assert_eq!(tokens.next(), None);
		};
	}

	#[test]
	fn can_enrich_example_file() {
		let sess = Session::default();
		let source = sess.source_map.load_anon(EXAMPLE.into());

		let rich_tokens = Enricher::from_source(
			&{
				let this = &sess;
				ParseSession {
					diag: &this.diagnostics,
				}
			},
			&source,
		)
		.into_iter()
		.collect::<Vec<_>>();

		println!("{rich_tokens:#?}");
	}

	#[test]
	fn parse_attr() {
		use crate::rich::TokenKind::*;

		tokens!(
			ATTR,
			(At, [0, 1]),
			(Ident(attrs::format), [1, 7]),
			(Colon, [7, 8]),
			(Ident(Symbol::intern("date")), [9, 13])
		);
	}

	#[test]
	fn parse_array_like() {
		use crate::rich::{Delimiter::*, LiteralKind::*, TokenKind::*};

		tokens![
			URLS,
			(Ident(sym!("urls")), [0, 4]),
			(OpenDelim(Bracket), [5, 6]),
			(
				Literal(Str, sym!("https://paradigm.lighton.ai/api/v1")),
				[8, 44]
			),
			(
				Literal(Str, sym!("https://paradigm-preprod.lighton.ai/api/v1")),
				[46, 90]
			),
			(
				Literal(Str, sym!("https://paradigm-dev.lighton.ai/api/v1")),
				[92, 132]
			),
			(CloseDelim(Bracket), [133, 134])
		];
	}
}
