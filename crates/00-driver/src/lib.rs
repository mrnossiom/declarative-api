use lexer::rich::Enricher;
use parser::Parser;

fn do_the_work() {
	let input = todo!();
	let token_stream = Enricher::from_source(input);

	let mut parser = Parser::from_tokens(/* token_stream */);
	let api = parser.parse_api()?;
}
