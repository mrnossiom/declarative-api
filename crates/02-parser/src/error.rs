// TODO
#![allow(unused_variables)]

use ast::types::AttrStyle;
use lexer::rich::TokenKind;
use macros::IntoDiagnostic;
use session::{Diagnostic, Span, Symbol};

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
	pub token: Span,

	pub parsed: TokenKind,
	pub expected: TokenKind,
}

#[derive(Debug, IntoDiagnostic)]
#[message("we expected a {expected} but found {parsed}")]
pub struct UnexpectedTokenInsteadOfKeyword {
	#[label("expected {expected}")]
	pub token: Span,

	pub parsed: TokenKind,
	pub expected: Symbol,
}
