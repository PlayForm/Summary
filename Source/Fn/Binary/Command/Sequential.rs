/// Processes entries sequentially, filtering and executing commands based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   * `Entry`: Vec<Vec<String>> - List of entries to process
///   * `Pattern`: String - The pattern to match for inclusion
///   * `Separator`: char - The path separator character
///
/// # Example
///
/// ```
/// let option = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.txt".to_string()]],
///     Pattern: "file.txt".to_string(),
///     Separator: '/',
/// };
/// Fn(option);
/// ```
pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
	Entry
		.into_iter()
		.filter_map(|Entry| {
			Entry
				.last()
				.filter(|Last| *Last == &Pattern)
				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
		})
		.for_each(|_Entry| {
			// TODO: GENERATE SUMMARY
		})
}

use crate::Struct::Binary::Command::Entry::Struct as Option;