use crate::Parser;
use ast::{
	types::{
		AttrId, AttrItem, AttrKind, AttrStyle, AttrVec, Attribute, Expr, ExprKind, FieldDef, Item,
		ItemKind, NodeId, NormalAttr, PropertyDef, Type,
	},
	P,
};
use session::{Ident, Span, Symbol};

impl<'a> Parser<'a> {
	pub(super) fn span(&self, lo: Span) -> Span {
		lo.to(self.prev_token.span)
	}

	// --- Items ---
	pub(super) fn make_item(
		attrs: AttrVec,
		kind: ItemKind,
		ident: Option<Ident>,
		span: Span,
	) -> P<Item> {
		P(Item {
			attrs,
			kind,
			ident: ident.unwrap_or(Ident::EMPTY),
			id: NodeId::DUMMY,
			span,
		})
	}

	// --- Expressions ---
	pub(super) fn make_expr(attrs: AttrVec, kind: ExprKind, span: Span) -> P<Expr> {
		P(Expr {
			attrs,
			kind,
			id: NodeId::DUMMY,
			span,
		})
	}

	pub(super) fn make_property_def(
		attrs: AttrVec,
		ident: Ident,
		expr: P<Expr>,
		span: Span,
	) -> P<PropertyDef> {
		P(PropertyDef {
			attrs,
			ident,
			expr,
			id: NodeId::DUMMY,
			span,
		})
	}

	pub(super) fn make_field_def(
		attrs: AttrVec,
		ident: Ident,
		ty: P<Type>,
		span: Span,
	) -> P<FieldDef> {
		P(FieldDef {
			attrs,
			ident,
			ty,
			id: NodeId::DUMMY,
			span,
		})
	}

	// --- Attributes ---
	pub(super) fn make_doc_attr(content: Symbol, style: AttrStyle, span: Span) -> Attribute {
		Attribute {
			kind: AttrKind::DocComment(content),
			style,
			id: AttrId::make_id(),
			span,
		}
	}

	pub(super) fn make_normal_attr(style: AttrStyle, span: Span) -> Attribute {
		let normal = NormalAttr { item: AttrItem {} };

		Attribute {
			kind: AttrKind::Normal(normal),
			style,
			id: AttrId::make_id(),
			span,
		}
	}
}
