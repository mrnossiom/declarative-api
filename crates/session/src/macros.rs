// When macros 2.0 we'll be a thing move macros from top-level to here.

#[cfg(test)]
mod tests {
	use crate::{BytePos, Ident, Span, Symbol};
	use crate::{ident, sp, sym};

	#[test]
	fn make_sym() {
		assert_eq!(sym!("var"), Symbol::intern("var"));
	}

	#[test]
	fn make_ident() {
		assert_eq!(
			ident!("var", 0, 3),
			Ident::new(Symbol::intern("var"), sp!(0, 3))
		);
	}

	#[test]
	fn make_span() {
		assert_eq!(sp!(0, 1), Span::from_bounds(BytePos(0), BytePos(1)));
	}
}
