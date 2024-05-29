use dapic_ast::{types::NodeId, visit_mut::MutVisitor};
use dapic_session::Idx;
use std::mem;

#[derive(Debug)]
pub struct NodeExpander {
	next_node_id: NodeId,
}

impl Default for NodeExpander {
	fn default() -> Self {
		Self {
			// TODO: enforce `Ast` is first visited node
			next_node_id: NodeId::ROOT,
		}
	}
}

impl NodeExpander {
	fn next_node_id(&mut self) -> NodeId {
		let next = self.next_node_id.inc();
		mem::replace(&mut self.next_node_id, next)
	}
}

impl MutVisitor for NodeExpander {
	fn visit_id(&mut self, id: &mut NodeId) {
		*id = self.next_node_id();
	}
}
