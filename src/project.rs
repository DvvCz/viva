use anyhow::Context;

pub struct Project<'a> {
	path: &'a std::path::Path,
	config: crate::config::Config,
}

impl<'a> Project<'a> {
	const SRC: &'static str = "src";
	const TARGET: &'static str = "target";
	const TESTS: &'static str = "tests";

	pub fn src(&self) -> std::path::PathBuf {
		self.path.join(Self::SRC)
	}

	pub fn target(&self) -> std::path::PathBuf {
		self.path.join(Self::TARGET)
	}

	pub fn tests(&self) -> std::path::PathBuf {
		self.path.join(Self::TESTS)
	}

	pub fn get_or_mkdir(path: std::path::PathBuf) -> anyhow::Result<std::path::PathBuf> {
		if !path.is_dir() {
			std::fs::create_dir(&path)?;
		}

		Ok(path)
	}

	fn read_config(dir: &std::path::Path) -> anyhow::Result<crate::config::Config> {
		let path = dir.join("viva.toml");
		let conf = std::fs::read_to_string(path).context("Couldn't find viva.toml")?;
		let conf = toml::from_str::<crate::config::Config>(&conf)?;
		Ok(conf)
	}

	pub fn open(path: &'a std::path::Path) -> anyhow::Result<Self> {
		anyhow::ensure!(path.is_dir(), "Provide a directory.");

		let config = Self::read_config(&path)?;
		Ok(Self { path, config })
	}

	pub fn build(&self) -> anyhow::Result<()> {
		let target = Self::get_or_mkdir(self.target())?;
		let part = &self.config.package.part;
		let sim_top = &self.config.entrypoints.sim;
		let syn_top = &self.config.entrypoints.syn;
		let externs = self.config.dependencies.files.join(" ");

		let build_script = target.join("build.tcl");

		std::fs::write(
			&build_script,
			indoc::formatdoc! {"
				create_project -part {part} build ./target/debug -force

				add_files -quiet {externs}
				add_files -quiet [glob -nocomplain ./src/*.sv]
				add_files -quiet [glob -nocomplain ./vendor/*]
				add_files -quiet [glob -nocomplain ./*.mem]

				add_files -quiet -fileset sim_1 [glob -nocomplain ./tests/*.sv]
				add_files -quiet -fileset constrs_1 ./constraints.xdc

				set_property top {syn_top} [current_fileset]
				update_compile_order -fileset sources_1

				set_property top {sim_top} [get_filesets sim_1]
				update_compile_order -fileset sim_1
			"},
		)?;

		let mut cmd = std::process::Command::new("vivado");

		cmd.args(&["-mode", "batch"])
			.arg("-source")
			.arg(build_script)
			.args(&["-nolog", "-nojournal"]);

		let e = cmd.output().context("uh")?;
		if !e.status.success() {
			let msg = String::from_utf8_lossy(&e.stderr);
			anyhow::bail!("Failed to run vivado: {msg}");
		}

		Ok(())
	}

	pub fn simulate(&self, uvm: Option<String>) -> anyhow::Result<()> {
		let target = Self::get_or_mkdir(self.target())?;
		let sim_script = target.join("sim.tcl");
		let uvm = uvm.as_ref().unwrap_or(&self.config.entrypoints.sim);

		std::fs::write(
			&sim_script,
			indoc::formatdoc! {"
				open_project ./target/debug/build.xpr
				set_property -name {{xsim.simulate.xsim.more_options}} -value {{-testplusarg UVM_TESTNAME={uvm}}} -objects [get_filesets sim_1]
				launch_simulation
			"},
		)?;

		std::process::Command::new("vivado")
			.args(&["-mode", "batch"])
			.arg("-journal")
			.arg(target.join("vivado.jou"))
			.arg("-log")
			.arg(target.join("vivado.log"))
			.arg("-source")
			.arg(&sim_script)
			.spawn()?
			.wait()?;

		Ok(())
	}
}
