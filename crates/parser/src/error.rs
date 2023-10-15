use ast::types::AttrStyle;
use lexer::rich::{Token, TokenKind};
use macros::IntoDiagnostic;
use session::{Diagnostic, Ident, Span, Symbol};

pub type PResult<T> = Result<T, Diagnostic>;

#[derive(Debug, IntoDiagnostic)]
#[message("we expected an {style} attribute but found a {parsed_style} attribute")]
pub struct WrongAttrStyle {
	#[label("expected {style}")]
	pub attr: Span,

	pub style: AttrStyle,
	pub parsed_style: AttrStyle,
}

#[derive(Debug, IntoDiagnostic)]
#[message("we expected a {expected} but found {parsed}")]
pub struct UnexpectedToken {
	#[label("expected {expected}")]
	pub parsed: Token,

	pub expected: TokenKind,
}

#[derive(Debug, IntoDiagnostic)]
#[message("we expected a {expected} but found {parsed}")]
pub struct UnexpectedTokenInsteadOfKeyword {
	#[label("expected {expected} keyword")]
	pub parsed: Token,

	pub expected: Symbol,
}

#[derive(Debug, IntoDiagnostic)]
#[severity(Warning)]
#[message("we expected an HTTP verb from the spec but found {found}")]
pub struct InvalidVerb {
	#[label("this is supposed to be a valid verb")]
	pub found: Ident,
}

#[derive(Debug, IntoDiagnostic)]
#[severity(Error)]
#[message("we expected a type")]
pub struct ExpectedType {
	#[label("expected a type here")]
	pub span: Span,
}
