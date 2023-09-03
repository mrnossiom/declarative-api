mod dev;

#[derive(Debug, clap::Parser)]
pub(crate) struct Args {
	#[clap(subcommand)]
	pub(crate) command: Commands,
}

impl Args {
	pub(crate) fn act(&mut self) {
		self.command.act();
	}
}

#[derive(Debug, clap::Subcommand)]
pub(crate) enum Commands {
	Dev(dev::Dev),
}

impl Commands {
	fn act(&mut self) {
		match self {
			Self::Dev(dev) => dev.act(),
		}
	}
}
