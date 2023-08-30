use crate::commands::Args;
use clap::Parser;
use tracing_subscriber::{
	filter::LevelFilter, fmt::format::FmtSpan, util::SubscriberInitExt, EnvFilter,
};

mod commands;

fn main() {
	tracing_subscriber::fmt::fmt()
		.with_env_filter(
			EnvFilter::builder()
				.with_default_directive(LevelFilter::INFO.into())
				.from_env()
				.unwrap(),
		)
		.with_span_events(FmtSpan::ENTER)
		.with_file(true)
		.with_line_number(true)
		.with_target(false)
		.without_time()
		.finish()
		.init();

	let mut op = Args::parse();

	op.act();
}
