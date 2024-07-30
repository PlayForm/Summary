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
pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
	futures::future::join_all(
		Entry
			.into_iter()
			.filter_map(|Entry| {
				Entry
					.last()
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
			.map(|Entry| {
				let Omit = Omit.clone();

				async move {
					match crate::Fn::Summary::Fn(
						&Entry,
						&crate::Struct::Summary::Difference::Struct { Omit },
					)
					.await
					{
						Ok(Summary) => Ok(Summary),
						Err(_Error) => {
							Err(format!("Error generating summary for {}: {}", Entry, _Error))
						}
					}
				}
			})
			.collect::<Vec<_>>(),
	)
	.await;
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
