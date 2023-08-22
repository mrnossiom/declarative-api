use crate::{
	poor,
	rich::{AttrStyle, Delimiter, LiteralKind, OpKind, Token, TokenKind},
	span::Span,
	symbols::Symbol,
};

/// Transforms [`poor::Token`]s that are only relevant when reading the source file at the
/// same time into [`rich::Token`]s that are self-explanatory. The latter doesn't include
/// tokens that don't add information to the generator such as whitespace or comments.
pub struct Enricher<'a> {
	source: &'a str,
	cursor: poor::Cursor<'a>,
	pos: u32,
}

impl<'a> Enricher<'a> {
	/// Creates a new [`Enricher`] for the given source, creating in the way
	/// the underlying [`poor::Cursor`] to get tokens to enrich.
	#[must_use]
	pub fn from_source(source: &'a str) -> Self {
		Self {
			source,
			cursor: poor::Cursor::new(source),
			pos: 0,
		}
	}

	/// Fetches the next token from the underlying poor lexer and add information
	///
	/// # Panics
	/// TODO: invalid characters or ident are not implemented yet
	pub fn next_token(&mut self) -> (Token, bool) {
		let mut has_whitespace_before = false;

		loop {
			let token = self.cursor.advance_token();
			let start = self.pos;
			self.pos += token.length;

			let kind = match token.kind {
				poor::TokenKind::Ident => {
					let ident = self.str_from(start).to_owned();
					let sym = symbol_dark_magic_to_remove(ident);

					TokenKind::Ident(sym)
				}
				poor::TokenKind::LineComment(style) => {
					// Skip non-doc comments
					let Some(style) = style else {
						has_whitespace_before = true;
						continue;
					};

					let len = match style {
						// `##`
						poor::DocStyle::Inner => 2,
						// `##!`
						poor::DocStyle::Outer => 3,
					};

					let content = self.str_from(start + len);

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

				poor::TokenKind::InvalidIdent | poor::TokenKind::Unknown => todo!(),

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

			let span = Span::from_bounds(start, self.pos);
			return (Token::new(kind, span), has_whitespace_before);
		}
	}

	fn cook_doc_line_comment(content: &str, style: poor::DocStyle) -> (AttrStyle, Symbol) {
		let sym = symbol_dark_magic_to_remove(content.to_owned());

		let style = match style {
			poor::DocStyle::Inner => AttrStyle::Inner,
			poor::DocStyle::Outer => AttrStyle::Outer,
		};

		(style, sym)
	}

	fn cook_literal(
		&mut self,
		start: u32,
		end: u32,
		kind: poor::LiteralKind,
	) -> (LiteralKind, Symbol) {
		match kind {
			poor::LiteralKind::Str { terminated } => {
				if !terminated {
					// error somehow
				}

				let content = self.str_from_to(start + 1, end - 1);
				let sym = symbol_dark_magic_to_remove(content.to_owned());

				(LiteralKind::Str, sym)
			}
			poor::LiteralKind::Number => {
				let content = self.str_from(start);
				let sym = symbol_dark_magic_to_remove(content.to_owned());

				(LiteralKind::Number, sym)
			}
		}
	}

	/// Slice of the source text from `start` up to but excluding `self.pos`,
	/// meaning the slice does not include the character `self.ch`.
	fn str_from(&self, start: u32) -> &'a str {
		self.str_from_to(start, self.pos)
	}

	/// Slice of the source text spanning from `start` up to but excluding `end`.
	fn str_from_to(&self, start: u32, end: u32) -> &'a str {
		&self.source[(start as usize)..(end as usize)]
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
	use crate::{
		symbols::attrs,
		tests::{ATTR, EXAMPLE, URLS},
	};

	macro_rules! sym {
		($lit:literal) => {
			Symbol::new_static($lit)
		};
	}

	macro_rules! tokens {
		($expr:ident, $(($ty:expr, [$lo:literal, $hi:literal])),+) => {
			let mut tokens = Enricher::from_source($expr).into_iter();

			$(
				assert_eq!(
					tokens.next(),
					Some(Token::new($ty, Span::from_bounds($lo, $hi)))
				);
			)+
			assert_eq!(tokens.next(), None);
		};
	}

	#[test]
	fn can_enrich_example_file() {
		let rich_tokens = Enricher::from_source(EXAMPLE)
			.into_iter()
			.collect::<Vec<_>>();

		println!("{rich_tokens:#?}");
	}

	#[test]
	fn parse_attr() {
		use crate::rich::TokenKind::*;
		use crate::span::Span;

		tokens!(
			ATTR,
			(At, [0, 1]),
			(Ident(attrs::Format), [1, 7]),
			(Colon, [7, 8]),
			(Ident(Symbol::new_static("date")), [9, 13])
		);
	}

	#[test]
	fn parse_array_like() {
		use crate::rich::{Delimiter::*, LiteralKind::*, TokenKind::*};
		use crate::span::Span;

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

fn symbol_dark_magic_to_remove(ident: String) -> Symbol {
	// FIXME(milo): oula, need to remove this a fast as possible
	// maybe using a symbol interner like what Rust does
	let ident = Box::leak(Box::new(ident));

	Symbol::new_static(ident)
}
