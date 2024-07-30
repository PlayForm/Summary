//! This module provides functionality for generating summaries of git repositories.

/// Generates a summary for a given git repository entry.
///
/// # Arguments
///
/// * `Entry` - A string representing the repository path.
/// * `Option` - A reference to a struct containing summary options.
///
/// # Returns
///
/// Returns a Result containing () if successful, or a boxed dynamic error if an error occurs.
///
/// # Errors
///
/// This function will return an error if the repository cannot be opened or if there are issues
/// generating the summary.
pub async fn Fn(
	Entry: &str,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<DashMap<u64, (String, String)>, Box<dyn std::error::Error>> {
	let Summary = DashMap::new();

	match Repository::open(Entry) {
		Ok(Repository) => {
			let Name = Repository.tag_names(None)?;

			let mut Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			Tag.sort();
			Tag.dedup();

			let Head = Repository.head()?;

			let First = Repository.find_commit(First::Fn(&Repository)?)?.id().to_string();

			let Last = Head.peel_to_commit()?.id().to_string();

			if Tag.is_empty() {
				Insert::Fn(
					&Summary,
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option)?,
					format!("🗣️ Summary from first commit to last commit:"),
				)
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					Insert::Fn(
						&Summary,
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?,
						format!("🗣️ Summary from tag: {} to tag: {}:", Start, End),
					);
				}

				if let Some(Latest) = Tag.last() {
					Insert::Fn(
						&Summary,
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?,
						format!("🗣️ Summary from first commit to latest tag: {}:", Latest),
					);

					Insert::Fn(
						&Summary,
						crate::Fn::Summary::Difference::Fn(&Repository, Latest, &Last, Option)?,
						format!("🗣️ Summary from latest tag: {} to last commit:", Latest),
					);
				}
			}
		}
		Err(_Error) => {
			println!("Cannot Repository: {}", _Error);

			return Err(_Error.into());
		}
	}

	Ok(Summary)
}

use dashmap::DashMap;
use git2::Repository;

pub mod Difference;
pub mod First;
pub mod Insert;
