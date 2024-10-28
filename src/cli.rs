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

	#[command(about = "Runs simulation")]
	Simulate {
		#[arg(long)]
		uvm: Option<String>,
		#[arg(long)]
		uvm_verbosity: Option<String>,
	},

	#[command(about = "Enter tcl repl")]
	Tcl,
}
