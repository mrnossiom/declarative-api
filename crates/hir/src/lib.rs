//! Declarative API high-level intermediate representation
//!
//! Entrypoint is [`HirLowerer::lower_root`]. Takes an [AST `Root`](dapic_ast::types::Root) and lowers it to a more
//! queryable form: [HIR `Root`](crate::types::Root). This form is used to easily resolve types.

use crate::types::Root;
use bumpalo::Bump;
use dapic_ast::types as ast;
use dapic_session::IndexVec;
use std::marker::PhantomData;

pub mod types;

#[must_use]
pub fn compile_hir(crate_: &ast::Root) -> Root {
	let lcx = LoweringContext::new();

	HirLowerer { lcx }.lower_root(crate_)
}

struct LoweringContext<'tcx> {
	arena: Bump,
	_marker: PhantomData<&'tcx ()>,
}

impl LoweringContext<'_> {
	fn new() -> Self {
		Self {
			arena: Bump::new(),
			_marker: PhantomData,
		}
	}
}

pub struct HirLowerer<'tcx> {
	lcx: LoweringContext<'tcx>,
}

impl<'tcx> HirLowerer<'tcx> {
	const fn new(lcx: LoweringContext<'tcx>) -> Self {
		Self { lcx }
	}

	pub fn lower_root(&mut self, crate_: &ast::Root) -> Root {
		// crate_.

		let items = IndexVec::default();
		Root { items }
	}
}
