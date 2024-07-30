/// Asynchronously processes entries to generate summaries and outputs the results.
///
/// This function performs the following steps:
/// 1. Filters and processes the provided entries based on the given pattern and separator.
/// 2. Spawns asynchronous tasks to generate summaries for each entry.
/// 3. Collects the results and outputs them.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   - `Entry`: A vector of vectors, where each inner vector contains the components of a file path.
///   - `Separator`: A character used to join the components of the file path.
///   - `Pattern`: A string pattern to match against the last element of each entry.
///   - `Omit`: A vector of strings representing patterns to omit.
///
/// # Example
///
/// ```rust
/// let options = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.git".to_string()]],
///     Separator: '/',
///     Pattern: ".git".to_string(),
///     Omit: vec!["target".to_string()],
/// };
/// Fn(options).await;
/// ```
///
/// # Errors
///
/// This function will log errors if it fails to generate summaries or send results.
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	let (Approval, mut Receipt) = tokio::sync::mpsc::unbounded_channel();

	let Entry = Entry
		.into_par_iter()
		.filter_map(|Entry| {
			Entry
				.last()
				.filter(|Last| *Last == &Pattern)
				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
		})
		.collect::<Vec<String>>();

	let Queue = FuturesUnordered::new();

	for Entry in Entry {
		let Omit = Omit.clone();
		let Approval = Approval.clone();

		Queue.push(tokio::spawn(async move {
			match crate::Fn::Summary::Fn(
				&Entry,
				&crate::Struct::Summary::Difference::Struct { Omit },
			)
			.await
			{
				Ok(Summary) => {
					if let Err(_Error) = Approval.send((Entry, Summary)) {
						eprintln!("Failed to send result: {}", _Error);
					}
				}
				Err(_Error) => eprintln!("Error generating summary for {}: {}", Entry, _Error),
			}
		}));
	}

	tokio::spawn(async move {
		Queue.collect::<Vec<_>>().await;
		drop(Approval);
	});

	let Output = DashMap::new();

	while let Some((Entry, Summary)) = Receipt.recv().await {
		for (_, (Difference, Message)) in Summary.into_iter() {
			Output
				.entry(Message + " in " + &Entry)
				.and_modify(|Existing: &mut HashSet<String>| {
					Existing.insert(Difference.clone());
				})
				.or_insert_with(|| {
					let mut Set = HashSet::new();
					Set.insert(Difference);
					Set
				});
		}
	}

	Output.into_iter().for_each(|(Message, Difference)| {
		println!("{}", Message);

		for Difference in Difference {
			println!("{}", Difference);
		}
	});
}

use dashmap::DashMap;
use futures::stream::{FuturesUnordered, StreamExt};
use rayon::prelude::*;
use std::collections::HashSet;

use crate::Struct::Binary::Command::Entry::Struct as Option;
