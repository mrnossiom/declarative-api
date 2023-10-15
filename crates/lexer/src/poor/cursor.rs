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
	pub fn from_source(source: &'a str) -> Cursor<'a> {
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
	///
	/// # Panics
	/// When used in a file that is over 4GiB
	pub(super) fn pos_within_token(&self) -> u32 {
		u32::try_from(self.len_remaining - self.chars.as_str().len())
			.expect("loaded sources can't go over 4 GiB")
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

	pub(super) fn eat_decimal_digits(&mut self) -> bool {
		let mut has_digits = false;
		loop {
			match self.first() {
				'_' => {
					self.bump();
				}
				'0'..='9' => {
					has_digits = true;
					self.bump();
				}
				_ => break,
			}
		}
		has_digits
	}
}

pub struct Iter<'a>(Cursor<'a>);

impl<'a> Iterator for Iter<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		let token = self.0.next_token();

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
	use crate::tests::{ATTR, EXAMPLE, URLS};

	macro_rules! assert_tokenize {
		($variant:literal, $src:ident) => {
			paste::paste! {
				#[test]
				fn [<tokenize_ $variant>]() {
					let vec = super::Cursor::from_source($src)
						.into_iter()
						.collect::<Vec<_>>();
					insta::assert_debug_snapshot!(vec);
				}
			}
		};
	}

	assert_tokenize!("example", EXAMPLE);
	assert_tokenize!("attribute", ATTR);
	assert_tokenize!("urls", URLS);
}
