use ast::types::AttrStyle;
use lexer::rich::TokenKind;
use session::{Diagnostic, DiagnosticsHandler, IntoDiagnostic, Span, Symbol};
use thiserror::Error;

pub type PResult<'a, T> = Result<T, Diagnostic>;

#[derive(Debug, Error, miette::Diagnostic)]
#[error("we expected an {style} attribute but found a {parsed_style} attribute")]
#[diagnostic(code(dapi::wrong_attr_style))]
pub struct WrongAttrStyle {
	#[label = "expected {style}"]
	pub attr: Span,

	pub style: AttrStyle,
	pub parsed_style: AttrStyle,
}

impl<'a> IntoDiagnostic<'a> for WrongAttrStyle {
	fn into_diag(self) -> Diagnostic {
		DiagnosticsHandler::builder(self)
	}
}

#[derive(Debug, Error, miette::Diagnostic)]
#[error("we expected a {expected} but found {parsed}")]
#[diagnostic(code(dapi::internal::unexpected_token))]
pub struct UnexpectedToken {
	#[label]
	pub token: Span,

	pub parsed: TokenKind,
	pub expected: TokenKind,
}

impl<'a> IntoDiagnostic<'a> for UnexpectedToken {
	fn into_diag(self) -> Diagnostic {
		DiagnosticsHandler::builder(self)
	}
}

#[derive(Debug, Error, miette::Diagnostic)]
#[error("we expected kw {expected} but found {parsed}")]
#[diagnostic(code(dapi::internal::unexpected_token_instead_of_kw))]
pub struct UnexpectedTokenInsteadOfKeyword {
	#[label]
	pub token: Span,

	pub parsed: TokenKind,
	pub expected: Symbol,
}

impl<'a> IntoDiagnostic<'a> for UnexpectedTokenInsteadOfKeyword {
	fn into_diag(self) -> Diagnostic {
		DiagnosticsHandler::builder(self)
	}
}
