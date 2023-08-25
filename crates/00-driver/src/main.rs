use crate::commands::Args;
use clap::Parser;

mod commands;

fn main() {
	let mut op = Args::parse();

	op.act();
}
