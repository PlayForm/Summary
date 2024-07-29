/// Represents the configuration options for the Summary command.
pub struct Struct {
	/// List of patterns to exclude from processing.
	pub Exclude: Vec<String>,

	/// Flag indicating whether to use parallel processing.
	pub Parallel: Parallel,

	/// The pattern to match for inclusion in processing.
	pub Pattern: Pattern,

	/// The root directory to start processing from.
	pub Root: String,

	/// The path separator character.
	pub Separator: Separator,
}

impl Struct {
	/// Creates a new Struct instance from the provided Option.
	pub fn Fn(Option { Separator, .. }: Option) -> Self {
		Self {
			Exclude: Fn()
				.get_one::<String>("Exclude")
				.expect("Cannot Exclude.")
				.split(" ")
				.map(|Command| Command.to_string())
				.collect::<Vec<_>>(),
			Parallel: Fn().get_flag("Parallel"),
			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Fn().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Separator,
		}
	}
}

use crate::{Fn::Binary::Command::Fn, Struct::Binary::Command::Struct as Option};

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;
