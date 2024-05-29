use crate::commands::Act;
use std::error::Error;

mod expand;
mod lex;
mod parse;

#[derive(Debug, clap::Parser)]
pub(crate) struct Dev {
	#[clap(subcommand)]
	command: DevCommands,
}

impl Act for Dev {
	fn act(&mut self) -> Result<(), Box<dyn Error>> {
		self.command.act()
	}
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum DevCommands {
	Lex(lex::Lex),
	Parse(parse::Parse),
	Expand(expand::Expand),
}

impl Act for DevCommands {
	fn act(&mut self) -> Result<(), Box<dyn Error>> {
		match self {
			Self::Lex(lex) => lex.act(),
			Self::Parse(parse) => parse.act(),
			Self::Expand(parse) => parse.act(),
		}
	}
}
