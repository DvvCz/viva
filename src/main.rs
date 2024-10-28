use std::io::Write;

mod cli;
mod config;
mod project;

fn main() -> anyhow::Result<()> {
	let cmd: cli::Cli = clap::Parser::parse();
	let cd = std::env::current_dir()?;

	match cmd.command {
		cli::Commands::Build => {
			let p = project::Project::open(&cd)?;
			p.build()?;
		}

		cli::Commands::Synth => {}

		cli::Commands::Simulate { uvm, uvm_verbosity } => {
			let p = project::Project::open(&cd)?;
			p.simulate(uvm)?;
		}

		cli::Commands::Tcl => {
			let mut cmd = std::process::Command::new("vivado")
				.args(&["-mode", "tcl"])
				.stdin(std::process::Stdio::piped())
				.stdout(std::process::Stdio::inherit())
				.spawn()?;

			let stdin = cmd.stdin.as_mut().unwrap();
			stdin.write_all(b"puts what")?;

			let _ = cmd.wait_with_output()?;
		}
	}

	Ok(())
}
