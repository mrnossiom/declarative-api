use crate::poor::Token;
use std::str::Chars;

pub struct Cursor<'a> {
	len_remaining: usize,
	chars: Chars<'a>,

	#[cfg(debug_assertions)]
	prev: char,
}

pub(crate) const EOF_CHAR: char = '\0';

impl<'a> Cursor<'a> {
	#[must_use]
	pub fn new(source: &'a str) -> Cursor<'a> {
		Cursor {
			len_remaining: source.len(),
			chars: source.chars(),
			#[cfg(debug_assertions)]
			prev: EOF_CHAR,
		}
	}

	/// Returns the last eaten symbol.
	#[cfg(debug_assertions)]
	pub(super) const fn prev(&self) -> char {
		self.prev
	}

	/// Peeks the next symbol from the input stream without consuming it.
	/// If requested position doesn't exist, `EOF_CHAR` is returned.
	/// However, getting `EOF_CHAR` doesn't always mean actual end of file,
	/// it should be checked with `is_eof` method.
	pub(super) fn first(&self) -> char {
		self.chars.clone().next().unwrap_or(EOF_CHAR)
	}

	/// Peeks the second symbol from the input stream without consuming it.
	pub(super) fn _second(&self) -> char {
		let mut iter = self.chars.clone();
		iter.next();
		iter.next().unwrap_or(EOF_CHAR)
	}

	/// Checks if there is nothing more to consume.
	pub(super) fn is_eof(&self) -> bool {
		self.chars.as_str().is_empty()
	}

	/// Returns amount of already consumed symbols.
	pub(super) fn pos_within_token(&self) -> u32 {
		(self.len_remaining - self.chars.as_str().len()) as u32
	}

	/// Resets the number of bytes consumed to 0.
	pub(super) fn reset_pos_within_token(&mut self) {
		self.len_remaining = self.chars.as_str().len();
	}

	/// Moves to the next character.
	pub(super) fn bump(&mut self) -> Option<char> {
		let c = self.chars.next()?;

		#[cfg(debug_assertions)]
		{
			self.prev = c;
		}

		Some(c)
	}

	/// Eats symbols while predicate returns true or until the end of file is reached.
	pub(super) fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
		while predicate(self.first()) && !self.is_eof() {
			self.bump();
		}
	}
}

pub struct Iter<'a>(Cursor<'a>);

impl<'a> Iterator for Iter<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		let token = self.0.advance_token();

		if token.kind.is_eof() {
			None
		} else {
			Some(token)
		}
	}
}

impl<'a> IntoIterator for Cursor<'a> {
	type Item = Token;
	type IntoIter = Iter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		Iter(self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::tests::{ATTR, EXAMPLE};

	#[test]
	fn can_tokenize_example_file() {
		let _poor_tokens = Cursor::new(EXAMPLE).into_iter().collect::<Vec<_>>();
	}

	#[test]
	fn parse_attr_style() {
		use crate::poor::TokenKind::*;

		let mut tokens = Cursor::new(ATTR).into_iter();

		macro_rules! next_token {
			($ty:ident, $len:literal) => {
				assert_eq!(tokens.next(), Some(Token::new($ty, $len)));
			};
			(@end) => {
				assert_eq!(tokens.next(), None);
			};
		}

		next_token!(At, 1);
		next_token!(Ident, 6);
		next_token!(Colon, 1);
		next_token!(Whitespace, 1);
		next_token!(Ident, 4);
		next_token!(@end);
	}
}
