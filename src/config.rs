nestify::nest! {
	#[derive(serde::Serialize, serde::Deserialize)]*
	pub struct Config {
		pub package: pub struct ConfigPackage {
			pub name: String,
			pub part: String,
		},

		pub entrypoints: pub struct ConfigEntrypoints {
			/// Top module for synthesis
			pub syn: String,
			/// Top module for simulation.
			pub sim: String
		},

		pub dependencies: pub struct ConfigDeps {
			pub files: Vec<String>
		}
	}
}
