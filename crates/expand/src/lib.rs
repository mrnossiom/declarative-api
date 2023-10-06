use ast::visitor::{noop, MutVisitor};
use session::Span;

struct ScopeExpander;

impl MutVisitor for ScopeExpander {
	fn visit_item(&mut self, item: &mut ast::types::P<ast::types::Item>) {
		item.ident.span = Span::DUMMY;
		noop::visit_item(self, item)
	}
}

pub fn expand_ast(api: &mut ast::types::Api) {
	ScopeExpander.visit_root(api)
}
