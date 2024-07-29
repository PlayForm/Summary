/// Represents the entry options for processing in the Summary command.
pub struct Struct {
	/// The path.
	pub Entry: Type,

	/// Flag indicating whether to use parallel processing.
	pub Parallel: Parallel,

	/// The pattern to match for inclusion in processing.
	pub Pattern: Pattern,

	/// The path separator character.
	pub Separator: Separator,
}

impl Struct {
	pub fn Fn(Option: &Option) -> Self {
		Self {
			Entry: crate::Fn::Binary::Command::Entry::Fn(Option),
			Parallel: Option.Parallel,
			Pattern: Option.Pattern.clone(),
			Separator: Option.Separator,
		}
	}
}

use crate::Struct::Binary::Command::Option::{Parallel, Pattern, Separator, Struct as Option};

/// Defines a type alias for a vector of vectors of strings.
pub type Type = Vec<Vec<String>>;
