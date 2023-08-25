#![doc = include_str!("../../../README.md")]

use lexer::rich::Enricher;
use parser::Parser;

fn _do_the_work() {
	let input = include_str!("../../../examples/paradigm.dapi");
	let token_stream = Enricher::from_source(input);

	let mut parser = Parser::from_tokens(token_stream);
	let _api = dbg!(parser.parse_api().unwrap());
}
