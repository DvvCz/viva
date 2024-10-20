use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	#[command(about = "Creates a vivado project in build dir.")]
	Build,
	#[command(about = "Runs synthesis")]
	Synth,
	#[command(about = "Simulates")]
	Simulate,

	#[command(about = "Enter tcl repl")]
	Tcl,
}
