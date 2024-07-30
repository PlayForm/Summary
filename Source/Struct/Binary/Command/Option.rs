pub struct Struct {
	pub Exclude: Vec<String>,

	pub Omit: Vec<String>,

	pub Parallel: Parallel,

	pub Pattern: Pattern,

	pub Root: String,

	pub Separator: Separator,
}

impl Struct {
	pub fn Fn(Option { Separator, .. }: Option) -> Self {
		Self {
			Exclude: Command()
				.get_one::<String>("Exclude")
				.expect("Cannot Exclude.")
				.split(" ")
				.map(|Exclude| Exclude.to_string())
				.collect::<Vec<_>>(),
			Parallel: Command().get_flag("Parallel"),
			Pattern: Command().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Command().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Separator,
			Omit: Command()
				.get_many::<String>("Omit")
				.expect("Cannot Omit.")
				.map(|Omit| Omit.to_string())
				.collect(),
		}
	}
}

use crate::{Fn::Binary::Command::Fn as Command, Struct::Binary::Command::Struct as Option};

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;
pub type Omit = Vec<String>;
