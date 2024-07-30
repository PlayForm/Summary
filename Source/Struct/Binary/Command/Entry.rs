pub struct Struct {
	pub Entry: Type,

	pub Parallel: Parallel,

	pub Pattern: Pattern,

	pub Separator: Separator,

	pub Omit: Omit,
}

impl Struct {
	pub fn Fn(Option: &Option) -> Self {
		Self {
			Entry: crate::Fn::Binary::Command::Entry::Fn(Option),
			Omit: Option.Omit.clone(),
			Parallel: Option.Parallel,
			Pattern: Option.Pattern.clone(),
			Separator: Option.Separator,
		}
	}
}

use crate::Struct::Binary::Command::Option::{
	Omit, Parallel, Pattern, Separator, Struct as Option,
};

pub type Type = Vec<Vec<String>>;
