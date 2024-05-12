//! Declarative API entry point.
//!
//! It offers multiple abstractions to run the compiler process and the end-generator.
//! A CLI is provided for development purposes but also for the end-user.

use crate::commands::Args;
use clap::Parser;
use commands::Act;
use std::error::Error;
use tracing_subscriber::{
	filter::LevelFilter, fmt::format::FmtSpan, util::SubscriberInitExt, EnvFilter,
};

mod commands;

fn main() -> Result<(), Box<dyn Error>> {
	tracing_subscriber::fmt::fmt()
		.with_env_filter(
			EnvFilter::builder()
				.with_default_directive(LevelFilter::INFO.into())
				.from_env()?,
		)
		.with_span_events(FmtSpan::ENTER | FmtSpan::EXIT)
		.with_file(true)
		.with_line_number(true)
		.with_target(false)
		.without_time()
		.finish()
		.init();

	let mut op = Args::parse();

	op.act()
}
