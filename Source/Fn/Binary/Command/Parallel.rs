//! This module contains functions for parallel command execution in a binary context.

/// Executes a sequence of operations asynchronously in parallel based on the provided options.
///
/// # Arguments
///
/// * `Option` - A struct containing various options for execution, including:
///   - `Entry`: A collection of entries to process
///   - `Separator`: A separator used for joining entry parts
///   - `Pattern`: A pattern to match against the last element of each entry
///   - `Omit`: A collection of items to omit from processing
///
/// # Async
///
/// This function is asynchronous and returns a future.
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

	// let Output = DashMap::new();

	while let Some((Entry, Summary)) = Receipt.recv().await {
		println!("Entry: {}", Entry);

		for (Hash, (Difference, Message)) in Summary.into_iter() {
			println!("{}", Hash);
			println!("{}", Message);
			println!("{}", Difference);

			// Output
			//     .entry(message.clone())
			//     .or_insert_with(Vec::new)
			//     .push(format!("{}: {}", Entry, content));
		}
	}
}

use dashmap::DashMap;
use futures::stream::{FuturesUnordered, StreamExt};
use rayon::prelude::*;

use crate::Struct::Binary::Command::Entry::Struct as Option;
