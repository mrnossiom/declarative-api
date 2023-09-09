// When macros 2.0 we'll be a thing move macros from top-level to here.

#[cfg(test)]
mod tests {
	use crate::{ident, sp, sym};
	use crate::{BytePos, Ident, Span, Symbol};

	#[test]
	fn make_sym() {
		assert_eq!(sym!("var"), Symbol::intern("var"));
	}

	#[test]
	fn make_ident() {
		assert_eq!(
			ident!("var", 0, 3),
			Ident::new(
				Symbol::intern("var"),
				Span {
					start: BytePos(0),
					end: BytePos(3),
				}
			)
		);
	}

	#[test]
	fn make_span() {
		assert_eq!(
			sp!(0, 1),
			Span {
				start: BytePos(0),
				end: BytePos(1),
			}
		);
	}
}
