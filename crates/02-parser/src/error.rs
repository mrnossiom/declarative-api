use ast::types::AttrStyle;
use lexer::rich::TokenKind;
use session::{
	Diagnostic, DiagnosticSource, DiagnosticsHandler, IntoDiagnostic, SourceFile, SourceMap, Span,
	Symbol,
};
use std::rc::Rc;
use thiserror::Error;

pub type PResult<'a, T> = Result<T, Diagnostic>;

#[derive(Debug, Error, miette::Diagnostic)]
#[error("we expected an {style} attribute but found a {parsed_style} attribute")]
#[diagnostic(code(dapi::wrong_attr_style), severity(Warning))]
pub struct WrongAttrStyle {
	#[label = "expected {style}"]
	pub attr: Span,

	pub style: AttrStyle,
	pub parsed_style: AttrStyle,
}

impl DiagnosticSource for WrongAttrStyle {
	fn source_file(&self, source_map: &SourceMap) -> Rc<SourceFile> {
		source_map
			.lookup_source_file_and_relative_pos(self.attr.start)
			.0
	}
}

impl<'a> IntoDiagnostic<'a> for WrongAttrStyle {
	fn into_diag(self, handler: &'a DiagnosticsHandler) -> Diagnostic {
		handler.builder(self)
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

impl DiagnosticSource for UnexpectedToken {
	fn source_file(&self, source_map: &SourceMap) -> Rc<SourceFile> {
		source_map
			.lookup_source_file_and_relative_pos(self.token.start)
			.0
	}
}

impl<'a> IntoDiagnostic<'a> for UnexpectedToken {
	fn into_diag(self, handler: &'a DiagnosticsHandler) -> Diagnostic {
		handler.builder(self)
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

impl DiagnosticSource for UnexpectedTokenInsteadOfKeyword {
	fn source_file(&self, source_map: &SourceMap) -> Rc<SourceFile> {
		source_map
			.lookup_source_file_and_relative_pos(self.token.start)
			.0
	}
}

impl<'a> IntoDiagnostic<'a> for UnexpectedTokenInsteadOfKeyword {
	fn into_diag(self, handler: &'a DiagnosticsHandler) -> Diagnostic {
		handler.builder(self)
	}
}
