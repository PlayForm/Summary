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
