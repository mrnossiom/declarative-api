//! Declarative API parser
//!
//! Entrypoint is [`Parser::from_source`]. Takes a stream of rich lexer
//! [`Token`](dapic_lexer::rich::Token)s and construct an abstract syntax tree.

mod error;
mod parser;

pub use crate::{error::PResult, parser::Parser};
