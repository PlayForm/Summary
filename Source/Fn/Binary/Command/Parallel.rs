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
pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
	let Queue: Vec<_> = stream::iter(
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
	.map(|Entry| async move {
		match crate::Fn::Summary::Fn(&Entry).await {
			Ok(summary) => Ok(summary),
			Err(e) => Err(format!("Error generating summary for {}: {}", Entry, e)),
		}
	})
	.buffer_unordered(num_cpus::get())
	.collect()
	.await;

	Queue.par_iter().for_each(|Output| match Output {
		Ok(Summary) => println!("Summary: {:?}", Summary),
		Err(Error) => eprintln!("Error: {}", Error),
	});
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
use futures::stream::{self, StreamExt};
use rayon::prelude::*;
