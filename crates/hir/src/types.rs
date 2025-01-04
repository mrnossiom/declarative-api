use dapic_session::{Ident, IndexVec, Span, new_index_ty};

new_index_ty! {
	pub struct HirId;
}

new_index_ty! {
	pub struct PathId;
}

// TODO: openapi lib with spec types as defined in the OAS

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TyKind<'tcx> {
	/// The base type, either
	/// - a path: `scope.Type`
	/// - or a local type: `Type`
	Path(&'tcx Path<'tcx>),

	/// An array of types: `[Type]`
	Array(&'tcx TyKind<'tcx>),

	/// A tuple of types: `(Ty1, Ty2, Ty3)`
	/// Can also define the unit type: `()`
	Tuple(&'tcx [TyKind<'tcx>]),

	/// A model defined inlined
	/// e.g. `{ error string }`
	InlineModel(&'tcx [FieldDef<'tcx>]),
}

pub struct Item {
	pub kind: ItemKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDef<'tcx> {
	pub ident: Ident,
	pub ty: &'tcx TyKind<'tcx>,

	pub id: HirId,
	pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path<'tcx> {
	/// The segments in the path: the things separated by `::`.
	pub segments: &'tcx [PathSegment],

	pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathSegment {
	pub ident: Ident,
	pub id: HirId,
}

pub enum ItemKind {
	Model,
}

pub struct Root {
	pub(crate) items: IndexVec<HirId, Item>,
}

impl Root {
	#[must_use]
	pub const fn items(&self) -> &Vec<Item> {
		self.items.items()
	}
}
