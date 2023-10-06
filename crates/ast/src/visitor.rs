use crate::{
	ptr::P,
	types::{Api, Attribute, Expr, Item, NodeId, Path, Ty},
};
use lexer::rich::Token;
use session::{Ident, Span};
use thin_vec::ThinVec;

pub trait MutVisitor: Sized {
	fn visit_root(&mut self, api: &mut Api) {
		noop::visit_root(self, api);
	}

	fn visit_attribute(&mut self, api: &mut Attribute) {
		noop::visit_attribute(self, api);
	}

	fn visit_ident(&mut self, ident: &mut Ident) {
		noop::visit_ident(self, ident);
	}

	fn visit_path(&mut self, path: &mut Path) {
		noop::visit_path(self, path);
	}

	fn visit_expr(&mut self, expr: &mut P<Expr>) {
		noop::visit_expr(self, expr);
	}

	fn visit_ty(&mut self, ty: &mut Ty) {
		noop::visit_ty(self, ty);
	}

	fn visit_item(&mut self, item: &mut P<Item>) {
		noop::visit_item(self, item);
	}

	fn visit_tokens(&mut self, _tokens: &mut ThinVec<Token>) {
		// TODO
	}

	fn visit_id(&mut self, _id: &mut NodeId) {
		// Nothing to explore further
	}
	fn visit_span(&mut self, _span: &mut Span) {
		// Nothing to explore further
	}
}

// No-operation
pub mod noop {
	use super::{ns, MutVisitor};
	use crate::types::{
		Api, AttrKind, Attribute, Auth, Body, Enum, Expr, ExprKind, Headers, Item, ItemKind,
		MetaAttr, Metadata, Model, NormalAttr, Params, Path, PathItem, PathSegment, Query,
		ScopeKind, StatusCode, Ty, TyKind, Verb,
	};
	use session::Ident;

	pub fn visit_root<V: MutVisitor>(
		v: &mut V,
		Api {
			attrs,
			items,
			id,
			span,
		}: &mut Api,
	) {
		v.visit_id(id);
		ns::visit_attrs(v, attrs);
		ns::visit_thin_vec(items, |item| v.visit_item(item));

		v.visit_span(span);
	}

	pub fn visit_ident<V: MutVisitor>(v: &mut V, Ident { span, symbol: _ }: &mut Ident) {
		v.visit_span(span);
	}

	pub fn visit_path<V: MutVisitor>(v: &mut V, Path { segments, span }: &mut Path) {
		for PathSegment { ident, id } in segments {
			v.visit_ident(ident);
			v.visit_id(id);
		}
		v.visit_span(span);
	}

	pub fn visit_expr<V: MutVisitor>(
		v: &mut V,
		Expr {
			attrs,
			id,
			kind,
			span,
		}: &mut Expr,
	) {
		ns::visit_attrs(v, attrs);
		v.visit_id(id);
		v.visit_span(span);

		match kind {
			ExprKind::Array(array) => {
				for expr in array {
					v.visit_expr(expr);
				}
			}
			ExprKind::Field(field, ident) => {
				v.visit_expr(field);
				v.visit_ident(ident);
			}
			ExprKind::Path(path) => v.visit_path(path),
			// Noop
			ExprKind::Literal(_, _) | ExprKind::Template(()) => {}
		}
	}

	pub fn visit_ty<V: MutVisitor>(v: &mut V, Ty { id, kind, span }: &mut Ty) {
		v.visit_id(id);
		v.visit_span(span);

		match kind {
			TyKind::Array(ty) | TyKind::Paren(ty) => v.visit_ty(ty),
			TyKind::InlineModel(model) => todo!(),
			TyKind::Path(path) => v.visit_path(path),
			TyKind::Tuple(tys) => ns::visit_thin_vec(tys, |ty| v.visit_ty(ty)),
		}
	}

	pub fn visit_item<V: MutVisitor>(
		v: &mut V,
		Item {
			attrs,
			id,
			ident: _,
			kind,
			span,
		}: &mut Item,
	) {
		ns::visit_attrs(v, attrs);
		v.visit_id(id);
		v.visit_span(span);

		match kind {
			ItemKind::Auth(auth) => match auth {
				Auth::Define {} | Auth::Use => {}
			},
			ItemKind::Body(Body { ty }) => v.visit_ty(ty),
			ItemKind::Enum(Enum { variants }) => ns::visit_thin_vec(variants, |pd| {}),
			ItemKind::Headers(Headers { headers }) => ns::visit_thin_vec(headers, |fd| {}),
			ItemKind::Meta(Metadata { fields }) => ns::visit_thin_vec(fields, |pd| {}),
			ItemKind::Model(Model { fields }) => ns::visit_thin_vec(fields, |fd| {}),
			ItemKind::Params(Params { properties }) => ns::visit_thin_vec(properties, |fd| {}),
			ItemKind::Path(PathItem { items, kind }) => {
				ns::visit_thin_vec(items, |item| v.visit_item(item));
			}
			ItemKind::Query(Query { fields }) => ns::visit_thin_vec(fields, |fd| {}),
			ItemKind::Scope(scope) => match scope {
				ScopeKind::Unloaded => {}
				ScopeKind::Loaded {
					items,
					inline: _,
					span,
				} => {
					v.visit_span(span);
					ns::visit_thin_vec(items, |item| v.visit_item(item));
				}
			},
			ItemKind::StatusCode(StatusCode { code, items }) => {
				v.visit_expr(code);
				ns::visit_thin_vec(items, |item| v.visit_item(item));
			}
			ItemKind::Verb(Verb { items, method: _ }) => {
				ns::visit_thin_vec(items, |item| v.visit_item(item));
			}
		}
	}

	pub fn visit_attribute<V: MutVisitor>(
		v: &mut V,
		Attribute {
			id: _,
			kind,
			span,
			style: _,
		}: &mut Attribute,
	) {
		v.visit_span(span);

		match kind {
			AttrKind::Normal(attr) => {
				let NormalAttr {
					path,
					tokens,
					delim: _,
				} = attr;

				v.visit_path(path);
				v.visit_tokens(tokens);
			}
			AttrKind::Meta(attr) => {
				let MetaAttr { expr, ident } = attr;

				if let Some(expr) = expr {
					v.visit_expr(expr);
				}
				v.visit_ident(ident);
			}
			AttrKind::DocComment(..) => {}
		}
	}
}

// Non-standard
pub mod ns {
	use super::MutVisitor;
	use crate::types::AttrVec;
	use thin_vec::ThinVec;

	#[inline]
	pub fn visit_attrs<V: MutVisitor>(v: &mut V, attrs: &mut AttrVec) {
		visit_thin_vec(attrs, |attr| v.visit_attribute(attr));
	}

	#[inline]
	pub fn visit_thin_vec<T, F>(elements: &mut ThinVec<T>, mut visitor: F)
	where
		F: FnMut(&mut T),
	{
		for element in elements {
			visitor(element);
		}
	}
}
