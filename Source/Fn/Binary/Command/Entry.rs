/// Walks through a directory and filters files based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   * `Exclude`: Vec<String> - List of patterns to exclude
///   * `Pattern`: String - The pattern to match for inclusion
///   * `Root`: String - The root directory to start the walk from
///   * `Separator`: char - The path separator character
///
/// # Returns
///
/// * `Return` - A vector of vectors of strings, where each inner vector represents a file path
///   split into its components.
///
/// # Example
///
/// ```
/// let option = Option {
///     Exclude: vec!["node_modules".to_string()],
///     Pattern: ".git".to_string(),
///     Root: ".".to_string(),
///     Separator: std::path::MAIN_SEPARATOR,
/// };
/// let result = Fn(&option);
/// ```
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
