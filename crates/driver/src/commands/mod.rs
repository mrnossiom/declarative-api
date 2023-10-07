use std::error::Error;

mod compile;
mod dev;

#[derive(Debug, clap::Parser)]
pub(crate) struct Args {
	#[clap(subcommand)]
	pub(crate) command: Commands,
}

impl Act for Args {
	fn act(&mut self) -> Result<(), Box<dyn Error>> {
		self.command.act()
	}
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum Commands {
	Dev(dev::Dev),
	Compile(compile::Compile),
}

impl Act for Commands {
	fn act(&mut self) -> Result<(), Box<dyn Error>> {
		match self {
			Self::Dev(dev) => dev.act(),
			Self::Compile(compile) => compile.act(),
		}
	}
}

pub(crate) trait Act {
	fn act(&mut self) -> Result<(), Box<dyn Error>>;
}
