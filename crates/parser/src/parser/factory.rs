use crate::Parser;
use dapic_ast::types::{
	AttrId, AttrKind, AttrStyle, AttrVec, Attribute, Expr, ExprKind, FieldDef, Item, ItemKind,
	MetaAttr, NodeId, NormalAttr, P, Path, PathSegment, PropertyDef, Ty, TyKind,
};
use dapic_lexer::rich::{Delimiter, Token};
use dapic_session::{Ident, Span, Symbol};
use thin_vec::{ThinVec, thin_vec};

impl Parser<'_> {
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
		ty: P<Ty>,
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
			id: AttrId::make_one(),
			span,
		}
	}

	pub(super) fn make_normal_attr(
		path: Ident,
		delim: Delimiter,
		tokens: ThinVec<Token>,
		style: AttrStyle,
		span: Span,
	) -> Attribute {
		let normal = NormalAttr {
			path,
			delim,
			tokens,
		};

		Attribute {
			kind: AttrKind::Normal(normal),
			style,
			id: AttrId::make_one(),
			span,
		}
	}

	pub(super) fn make_meta_attr(
		ident: Ident,
		expr: Option<P<Expr>>,
		style: AttrStyle,
		span: Span,
	) -> Attribute {
		Attribute {
			kind: AttrKind::Meta(MetaAttr { ident, expr }),
			style,
			id: AttrId::make_one(),
			span,
		}
	}

	// --- Types ---
	pub(super) fn make_ty(kind: TyKind, span: Span) -> P<Ty> {
		P(Ty {
			kind,
			id: NodeId::DUMMY,
			span,
		})
	}

	pub(super) fn make_ty_kind_single(ident: Ident, span: Span) -> TyKind {
		TyKind::Path(Path {
			segments: thin_vec![PathSegment {
				ident,
				id: NodeId::DUMMY,
			}],
			span,
		})
	}
}
