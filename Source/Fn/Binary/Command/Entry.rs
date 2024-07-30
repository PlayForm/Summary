//! This module provides functionality for processing binary command entries.

/// Processes entries based on the provided options.
///
/// # Arguments
///
/// * `Option` - A reference to an Option struct containing processing parameters.
///
/// # Returns
///
/// Returns a vector of processed entries.
pub fn Fn(Option { Exclude, Pattern, Root, Separator, .. }: &Option) -> Return {
	WalkDir::new(Root)
		.follow_links(false)
		.into_iter()
		.filter_map(|Entry| {
			let Path = Entry.expect("Cannot Entry.").path().display().to_string();

			// TODO: Separate this into Entry/Exclude.rs
			if !Exclude
				.clone()
				.into_iter()
				.filter(|Exclude| *Pattern != *Exclude)
				.any(|Exclude| Path.contains(&Exclude))
			{
				Some(Path.split(*Separator).map(|Entry| Entry.to_string()).collect())
			} else {
				None
			}
		})
		.collect::<Vec<_>>()
}

use crate::Struct::Binary::Command::{Entry::Type as Return, Option::Struct as Option};

use walkdir::WalkDir;
