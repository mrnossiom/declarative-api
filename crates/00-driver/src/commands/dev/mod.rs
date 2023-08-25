mod lex;
mod parse;

#[derive(Debug, clap::Parser)]
pub(crate) struct Dev {
	#[clap(subcommand)]
	command: DevCommands,
}

impl Dev {
	pub(crate) fn act(&mut self) {
		self.command.act()
	}
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum DevCommands {
	Lex(lex::Lex),
	Parse(parse::Parse),
}

impl DevCommands {
	fn act(&mut self) {
		match self {
			Self::Lex(lex) => lex.act(),
			Self::Parse(parse) => parse.act(),
		};
	}
}
