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
) -> Result<(), Box<dyn std::error::Error>> {
	match Repository::open(Entry) {
		Ok(Repository) => {
			let Name = Repository.tag_names(None)?;

			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			let Head = Repository.head()?;

			let First = Repository.find_commit(First::Fn(&Repository)?)?.id().to_string();

			let Last = Head.peel_to_commit()?.id().to_string();

			if Tag.is_empty() {
				println!("🗣️ Summary from first commit to last commit:");

				println!(
					"```\n{}\n```",
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option,)?
				);
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					println!("🗣️ Summary from tag: {} to tag: {}:", Start, End);

					println!(
						"```\n{}\n```",
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?
					);
				}

				if let Some(Latest) = Tag.last() {
					println!("🗣️ Summary from first commit to latest tag: {}:", Latest);

					println!(
						"```\n{}\n```",
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?
					);

					println!("🗣️ Summary from latest tag: {} to last commit:", Latest);

					println!(
						"```\n{}\n```",
						crate::Fn::Summary::Difference::Fn(&Repository, Latest, &Last, Option)?
					);
				}
			}
		}
		Err(_Error) => {
			println!("Cannot Repository: {}", _Error);

			return Err(_Error.into());
		}
	}

	Ok(())
}

use git2::Repository;

pub mod Difference;
pub mod First;
