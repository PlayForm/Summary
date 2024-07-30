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
