use ast::{
	types::{ItemKind, ScopeKind},
	visitor::{noop, MutVisitor},
};
use parser::Parser;
use session::Session;
use std::path::Path;

struct ScopeExpander<'a> {
	session: &'a Session,
}

impl<'a> MutVisitor for ScopeExpander<'a> {
	fn visit_item(&mut self, item: &mut ast::types::P<ast::types::Item>) {
		let ItemKind::Scope(ScopeKind::Unloaded) = item.kind else {
			noop::visit_item(self, item);
			return;
		};

		let name = format!("{}.dapi", item.ident.symbol.as_str());
		let path = std::env::current_dir().unwrap().join(Path::new(&name));
		let source = self.session.parse.source_map.load_file(&path).unwrap();

		let parser = &mut Parser::from_source(&self.session.parse, &source);

		let lo = parser.token.span;
		let items = parser.parse_scope_content(Some(&mut item.attrs)).unwrap();
		let span = lo.to(parser.prev_token.span);

		let ItemKind::Scope(kind) = &mut item.kind else {
			unreachable!("we checked that the item was an unloaded scope")
		};

		*kind = ScopeKind::Loaded {
			items,
			inline: false,
			span,
		};

		noop::visit_item(self, item);
	}
}

pub fn expand_ast(session: &Session, api: &mut ast::types::Api) {
	ScopeExpander { session }.visit_root(api)
}
