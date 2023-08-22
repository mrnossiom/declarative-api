use super::{
	cursor::Cursor,
	token::{
		DocStyle,
		LiteralKind::*,
		Token,
		TokenKind::{self, *},
	},
};
use unicode_xid::UnicodeXID;

impl<'a> Cursor<'a> {
	pub(crate) fn advance_token(&mut self) -> Token {
		let Some(first_char) = self.bump() else {
			return Token::new(Eof, 0);
		};

		let token_kind = match first_char {
			'#' => match self.first() {
				'#' | ' ' => self.line_comment(),
				_ => Pound,
			},

			// Whitespace sequence.
			c if is_whitespace(c) => self.whitespace(),

			// Identifier (this should be checked after other variant that can
			// start as identifier).
			c if is_id_start(c) => self.ident(),

			// Numeric literal.
			c @ '0'..='9' => self.number(c),

			// One-symbol tokens.
			';' => Semi,
			',' => Comma,
			'.' => Dot,
			'(' => OpenParenthesis,
			'{' => OpenBrace,
			'[' => OpenBracket,
			')' => CloseParenthesis,
			'}' => CloseBrace,
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
				Literal(Str { terminated })
			}

			_ => Unknown,
		};

		let res = Token::new(token_kind, self.pos_within_token());
		self.reset_pos_within_token();
		res
	}

	fn line_comment(&mut self) -> TokenKind {
		debug_assert!(self.prev() == '#' && (self.first() == '#' || self.first() == ' '));
		self.bump();

		let style = match self.first() {
			'!' => Some(DocStyle::Inner),
			' ' => Some(DocStyle::Outer),
			_ => None,
		};

		self.eat_while(|c| c != '\n');
		LineComment(style)
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
			UnicodeXID::is_xid_continue(c)
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

	fn number(&mut self, _c: char) -> TokenKind {
		self.eat_decimal_digits();

		Literal(Number)
	}
}

/// True if `c` is considered a whitespace
const fn is_whitespace(c: char) -> bool {
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
fn is_id_start(c: char) -> bool {
	// This is XID_Start OR '_' (which formally is not a XID_Start).
	c == '_' || UnicodeXID::is_xid_start(c)
}

/// True if `c` is valid as a non-first character of an identifier.
fn is_id_continue(c: char) -> bool {
	UnicodeXID::is_xid_continue(c)
}

/// The passed string is lexically an identifier.
fn _is_ident(string: &str) -> bool {
	let mut chars = string.chars();
	chars.next().map_or(false, |start| {
		is_id_start(start) && chars.all(is_id_continue)
	})
}
