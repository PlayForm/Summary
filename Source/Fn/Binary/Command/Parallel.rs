/// Processes entries in parallel, filtering and executing commands based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   * `Entry`: Vec<Vec<String>> - List of entries to process
///   * `Separator`: char - The path separator character
///   * `Pattern`: String - The pattern to match for inclusion
///
/// # Example
///
/// ```
/// let option = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.txt".to_string()]],
///     Separator: '/',
///     Pattern: "file.txt".to_string(),
/// };
/// Fn(option).await;
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	futures::stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
				Entry
					.last()
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
			.collect::<Vec<String>>(),
	)
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
				Err(_Error) => Err(format!("Error generating summary for {}: {}", Entry, _Error)),
			}
		}
	})
	.buffer_unordered(num_cpus::get())
	.collect::<Vec<_>>()
	.await;
}

use futures::stream::StreamExt;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::Struct::Binary::Command::Entry::Struct as Option;
