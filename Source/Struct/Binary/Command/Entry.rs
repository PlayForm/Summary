//! This module defines structures and functions related to binary command entries.

/// Represents the structure for binary command entries.
pub struct Struct {
	pub Entry: Type,

	pub Parallel: Parallel,

	pub Pattern: Pattern,

	pub Separator: Separator,

	pub Omit: Omit,
}

impl Struct {
	/// Creates a new Struct instance from the given options.
	///
	/// # Arguments
	///
	/// * `Option` - A reference to an Option struct containing initialization parameters.
	///
	/// # Returns
	///
	/// Returns a new instance of Struct.
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
