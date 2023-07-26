mod cursor;

use cursor::Cursor;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
	pub kind: TokenKind,
	pub length: u32,
}

impl Token {
	fn new(kind: TokenKind, length: u32) -> Self {
		Self { kind, length }
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
	LineComment,

	/// Any whitespace character.
	Whitespace,

	/// An identifier
	Ident,

	/// Like the above, but containing invalid unicode codepoints.
	InvalidIdent,

	/// Examples: `12u8`, `1.0e-40`, `b"123"`. Note that `_` is an invalid
	/// suffix, but may be present here on string and float literals. Users of
	/// this type will need to check for and reject that case.
	///
	/// See [LiteralKind] for more details.
	Literal {
		kind: LiteralKind,
		suffix_start: u32,
	},

	// One-char tokens:
	/// ";"
	Semi,
	/// ","
	Comma,
	/// "."
	Dot,
	/// "("
	OpenParen,
	/// ")"
	CloseParen,
	/// "{"
	OpenBrace,
	/// "}"
	CloseBrace,
	/// "["
	OpenBracket,
	/// "]"
	CloseBracket,
	/// "@"
	At,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralKind {
	/// "12_u8", "0o100", "0b120i99", "1f32".
	Number,
	/// ""abc"", ""abc"
	Str { terminated: bool },
}

use LiteralKind::*;
use TokenKind::*;

impl<'a> Cursor<'a> {
	fn advance_token(&mut self) -> Token {
		let Some(first_char) = self.bump() else {
			return Token::new(TokenKind::Eof, 0);
		};

		let token_kind = match first_char {
			'#' => self.line_comment(),

			// Whitespace sequence.
			c if is_whitespace(c) => self.whitespace(),

			// Identifier (this should be checked after other variant that can
			// start as identifier).
			c if is_id_start(c) => self.ident(),

			// One-symbol tokens.
			';' => Semi,
			',' => Comma,
			'.' => Dot,
			'(' => OpenParen,
			')' => CloseParen,
			'{' => OpenBrace,
			'}' => CloseBrace,
			'[' => OpenBracket,
			']' => CloseBracket,
			'@' => At,
			'~' => Tilde,
			'?' => Question,
			':' => Colon,
			'$' => Dollar,
			'=' => Eq,
			'!' => Bang,
			'<' => Lt,
			'>' => Gt,
			'-' => Minus,
			'&' => And,
			'|' => Or,
			'+' => Plus,
			'*' => Star,
			'^' => Caret,
			'%' => Percent,

			// String literal.
			'"' => {
				let terminated = self.double_quoted_string();
				let suffix_start = self.pos_within_token();

				let kind = Str { terminated };
				Literal { kind, suffix_start }
			}

			_ => Unknown,
		};

		let res = Token::new(token_kind, self.pos_within_token());
		self.reset_pos_within_token();
		res
	}

	fn line_comment(&mut self) -> TokenKind {
		self.eat_while(|c| c != '\n');
		LineComment
	}

	fn whitespace(&mut self) -> TokenKind {
		debug_assert!(is_whitespace(self.prev()));
		self.eat_while(is_whitespace);
		Whitespace
	}

	fn ident(&mut self) -> TokenKind {
		debug_assert!(is_id_start(self.prev()));
		// Start is already eaten, eat the rest of identifier.
		self.eat_while(is_id_continue);
		// Known prefixes must have been handled earlier. So if
		// we see a prefix here, it is definitely an unknown prefix.
		match self.first() {
			c if !c.is_ascii() && unic_emoji_char::is_emoji(c) => self.fake_ident(),
			_ => Ident,
		}
	}

	fn fake_ident(&mut self) -> TokenKind {
		// Start is already eaten, eat the rest of identifier.
		self.eat_while(|c| {
			unicode_xid::UnicodeXID::is_xid_continue(c)
				|| (!c.is_ascii() && unic_emoji_char::is_emoji(c))
				|| c == '\u{200d}'
		});

		InvalidIdent
	}

	/// Eats double-quoted string and returns true
	/// if string is terminated.
	fn double_quoted_string(&mut self) -> bool {
		debug_assert!(self.prev() == '"');
		while let Some(c) = self.bump() {
			match c {
				'"' => {
					return true;
				}
				'\\' if self.first() == '\\' || self.first() == '"' => {
					// Bump again to skip escaped character.
					self.bump();
				}
				_ => (),
			}
		}
		// End of file reached.
		false
	}
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + '_ {
	let mut cursor = Cursor::new(input);
	std::iter::from_fn(move || {
		let token = cursor.advance_token();
		if token.kind != TokenKind::Eof {
			Some(token)
		} else {
			None
		}
	})
}

/// True if `c` is considered a whitespace according to Rust language definition.
/// See [Rust language reference](https://doc.rust-lang.org/reference/whitespace.html)
/// for definitions of these classes.
pub fn is_whitespace(c: char) -> bool {
	// This is Pattern_White_Space.
	//
	// Note that this set is stable (ie, it doesn't change with different
	// Unicode versions), so it's ok to just hard-code the values.

	matches!(
		c,
		// Usual ASCII suspects
		'\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
	)
}

/// True if `c` is valid as a first character of an identifier.
/// See [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html) for
/// a formal definition of valid identifier name.
pub fn is_id_start(c: char) -> bool {
	// This is XID_Start OR '_' (which formally is not a XID_Start).
	c == '_' || unicode_xid::UnicodeXID::is_xid_start(c)
}

/// True if `c` is valid as a non-first character of an identifier.
/// See [Rust language reference](https://doc.rust-lang.org/reference/identifiers.html) for
/// a formal definition of valid identifier name.
pub fn is_id_continue(c: char) -> bool {
	unicode_xid::UnicodeXID::is_xid_continue(c)
}

/// The passed string is lexically an identifier.
pub fn is_ident(string: &str) -> bool {
	let mut chars = string.chars();
	if let Some(start) = chars.next() {
		is_id_start(start) && chars.all(is_id_continue)
	} else {
		false
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_example_dapi_file() {
		let content = include_str!("../../../tests/main.dapi");
		let tokens: Vec<_> = tokenize(content).collect();
		dbg!(tokens);
	}

	#[test]
	fn parse_attr_style() {
		let content = "@format: date";
		let mut tokens = tokenize(content);

		macro_rules! next_token {
			($tk:expr) => {
				assert_eq!(tokens.next(), Some($tk));
			};
		}

		next_token!(Token::new(At, 1));
		next_token!(Token::new(Ident, 6));
		next_token!(Token::new(Colon, 1));
		next_token!(Token::new(Whitespace, 1));
		next_token!(Token::new(Ident, 4));
		assert_eq!(tokens.next(), None);
	}
}
