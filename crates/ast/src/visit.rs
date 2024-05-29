use crate::types::{Ast, Attribute, Expr, FieldDef, Item, NodeId, Path, PropertyDef, Ty, P};
use dapic_lexer::rich::Token;
use dapic_session::{Ident, Span};
use thin_vec::ThinVec;

// TODO: macro to keep clean and sync?

pub trait Visitor: Sized {
	fn visit_root(&mut self, api: &Ast) {
		noop::visit_root(self, api);
	}

	fn visit_attribute(&mut self, api: &Attribute) {
		noop::visit_attribute(self, api);
	}

	fn visit_ident(&mut self, ident: &Ident) {
		noop::visit_ident(self, ident);
	}

	fn visit_path(&mut self, path: &Path) {
		noop::visit_path(self, path);
	}

	fn visit_expr(&mut self, expr: &P<Expr>) {
		noop::visit_expr(self, expr);
	}

	fn visit_ty(&mut self, ty: &Ty) {
		noop::visit_ty(self, ty);
	}

	fn visit_item(&mut self, item: &P<Item>) {
		noop::visit_item(self, item);
	}

	fn visit_field_def(&mut self, field: &P<FieldDef>) {
		noop::visit_field_def(self, field);
	}
	fn visit_property_def(&mut self, property: &P<PropertyDef>) {
		noop::visit_property_def(self, property);
	}

	// Nothing to explore further
	// For the next functions we don't use a `_` prefix because of trait impl autocomplete
	fn visit_tokens(&mut self, tokens: &ThinVec<Token>) {
		let _ = tokens;
	}
	fn visit_id(&mut self, id: &NodeId) {
		let _ = id;
	}
	fn visit_span(&mut self, span: &Span) {
		let _ = span;
	}
}
pub mod noop {
	use super::{ns, Visitor};
	use crate::types::{
		Ast, AttrKind, Attribute, Auth, Body, Enum, Expr, ExprKind, FieldDef, Headers, Item,
		ItemKind, MetaAttr, Metadata, Model, NormalAttr, Params, Path, PathItem, PathSegment,
		PropertyDef, Query, ScopeKind, StatusCode, Ty, TyKind, Verb,
	};
	use dapic_session::Ident;

	pub fn visit_root<V: Visitor>(
		v: &mut V,
		Ast {
			attrs,
			items,
			id,
			span,
		}: &Ast,
	) {
		v.visit_id(id);
		ns::visit_attrs(v, attrs);
		ns::visit_thin_vec(items, |item| v.visit_item(item));

		v.visit_span(span);
	}

	pub fn visit_ident<V: Visitor>(v: &mut V, Ident { span, symbol: _ }: &Ident) {
		v.visit_span(span);
	}

	pub fn visit_path<V: Visitor>(v: &mut V, Path { segments, span }: &Path) {
		for PathSegment { ident, id } in segments {
			v.visit_ident(ident);
			v.visit_id(id);
		}
		v.visit_span(span);
	}

	pub fn visit_expr<V: Visitor>(
		v: &mut V,
		Expr {
			attrs,
			id,
			kind,
			span,
		}: &Expr,
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

	pub fn visit_ty<V: Visitor>(v: &mut V, Ty { id, kind, span }: &Ty) {
		v.visit_id(id);
		v.visit_span(span);

		match kind {
			TyKind::Array(ty) | TyKind::Paren(ty) => v.visit_ty(ty),
			TyKind::InlineModel(fields) => ns::visit_thin_vec(fields, |fd| v.visit_field_def(fd)),
			TyKind::Path(path) => v.visit_path(path),
			TyKind::Tuple(tys) => ns::visit_thin_vec(tys, |ty| v.visit_ty(ty)),
		}
	}

	pub fn visit_field_def<V: Visitor>(
		v: &mut V,
		FieldDef {
			attrs,
			id,
			ident,
			span,
			ty,
		}: &FieldDef,
	) {
		ns::visit_attrs(v, attrs);
		v.visit_id(id);
		v.visit_ident(ident);
		v.visit_span(span);
		v.visit_ty(ty);
	}

	pub fn visit_property_def<V: Visitor>(
		v: &mut V,
		PropertyDef {
			attrs,
			expr,
			id,
			ident,
			span,
		}: &PropertyDef,
	) {
		ns::visit_attrs(v, attrs);
		v.visit_expr(expr);
		v.visit_id(id);
		v.visit_ident(ident);
		v.visit_span(span);
	}

	pub fn visit_item<V: Visitor>(
		v: &mut V,
		Item {
			attrs,
			id,
			ident,
			kind,
			span,
		}: &Item,
	) {
		ns::visit_attrs(v, attrs);
		v.visit_id(id);
		v.visit_span(span);
		v.visit_ident(ident);

		match kind {
			ItemKind::Auth(auth) => match auth {
				Auth::Define {} | Auth::Use => {}
			},
			ItemKind::Body(Body { ty }) => v.visit_ty(ty),
			ItemKind::Enum(Enum { variants }) => {
				ns::visit_thin_vec(variants, |pd| v.visit_property_def(pd));
			}
			ItemKind::Headers(Headers { headers }) => {
				ns::visit_thin_vec(headers, |fd| v.visit_field_def(fd));
			}
			ItemKind::Meta(Metadata { fields }) => {
				ns::visit_thin_vec(fields, |pd| v.visit_property_def(pd));
			}
			ItemKind::Model(Model { fields }) => {
				ns::visit_thin_vec(fields, |fd| v.visit_field_def(fd));
			}
			ItemKind::Params(Params { properties }) => {
				ns::visit_thin_vec(properties, |fd| v.visit_field_def(fd));
			}
			ItemKind::Path(PathItem { items, kind }) => {
				ns::visit_thin_vec(items, |item| v.visit_item(item));
				ns::visit_path_kind(v, kind);
			}
			ItemKind::Query(Query { fields }) => {
				ns::visit_thin_vec(fields, |fd| v.visit_field_def(fd));
			}
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
			ItemKind::Verb(Verb { items, method }) => {
				ns::visit_thin_vec(items, |item| v.visit_item(item));
				v.visit_ident(method);
			}
		}
	}

	pub fn visit_attribute<V: Visitor>(
		v: &mut V,
		Attribute {
			id: _,
			kind,
			span,
			style: _,
		}: &Attribute,
	) {
		v.visit_span(span);

		match kind {
			AttrKind::Normal(attr) => {
				let NormalAttr {
					path,
					tokens,
					delim: _,
				} = attr;

				v.visit_ident(path);
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
	use super::Visitor;
	use crate::types::{AttrVec, PathKind};
	use thin_vec::ThinVec;

	#[inline]
	pub fn visit_attrs<V: Visitor>(v: &mut V, attrs: &AttrVec) {
		visit_thin_vec(attrs, |attr| v.visit_attribute(attr));
	}

	#[inline]
	pub fn visit_thin_vec<T, F>(elements: &ThinVec<T>, mut visitor: F)
	where
		F: FnMut(&T),
	{
		for element in elements {
			visitor(element);
		}
	}

	pub fn visit_path_kind<V: Visitor>(v: &mut V, kind: &PathKind) {
		match kind {
			PathKind::Current => {}
			PathKind::Simple(path) | PathKind::Variable(path) => v.visit_ident(path),
			PathKind::Complex(path) => visit_thin_vec(path, |part| visit_path_kind(v, part)),
		}
	}
}
