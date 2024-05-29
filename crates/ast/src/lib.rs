//! Declarative API abstract syntax tree
//!
//! Essentialy contains AST type definitions. Modules [`visit_mut`] is used for
//! the expansion phase while [`visit`] is used to collect items during `AST`
//! lowering.

mod ptr;
pub mod types;
pub mod visit;
pub mod visit_mut;
