//! Declarative API Parser
//!
//! The main part is [`Parser`]. It takes a stream of rich lexer
//! [`Tokens`](lexer::rich::Token) and parses them into a tree of AST nodes.

mod error;
mod parser;

pub use crate::{error::PResult, parser::Parser};
