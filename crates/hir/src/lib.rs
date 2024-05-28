use dapic_ast::types::Ast;
use dapic_session::{new_index_ty, IndexVec};

new_index_ty! {
	pub struct HirId;
}

pub struct Hir {
	items: IndexVec<HirId, ()>,
}

#[must_use]
pub fn compile_hir(crate_: &Ast) -> Hir {
	Hir {
		items: IndexVec::default(),
	}
}
