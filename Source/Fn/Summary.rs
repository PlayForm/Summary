/// Asynchronously generates a summary of differences between commits in a git repository.
///
/// This function performs the following steps:
/// 1. Opens the specified git repository.
/// 2. Retrieves and sorts the tags in the repository.
/// 3. Identifies the first and last commits in the repository.
/// 4. Generates summaries of differences between the first commit and the last commit, as well as between each pair of consecutive tags.
/// 5. Inserts the generated summaries into a DashMap.
///
/// # Arguments
///
/// * `Entry` - A string slice representing the path to the git repository.
/// * `Option` - A reference to a struct containing options for generating the diff summary.
///
/// # Returns
///
/// Returns a `Result` containing a DashMap with the generated summaries if successful, or a boxed `dyn std::error::Error` if an error occurs.
///
/// # Errors
///
/// This function will return an error if there are issues with opening the repository, retrieving tags, or generating the diff summaries.
///
/// # Example
///
/// ```rust
/// let options = crate::Struct::Summary::Difference::Struct {
///     Omit: vec!["(?i)\\.log$".to_string()],
/// };
/// let summary = Fn("/path/to/repo", &options).await.expect("Cannot generate summary.");
/// for entry in summary.iter() {
///     println!("{:?}", entry);
/// }
/// ```
///
/// # Panics
///
/// This function does not panic.
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
					format!("🗣️ Summary from first commit to last commit"),
				)
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					Insert::Fn(
						&Summary,
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?,
						format!("🗣️ Summary from {} to {}", Start, End),
					);
				}

				if let Some(Latest) = Tag.last() {
					Insert::Fn(
						&Summary,
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?,
						format!("🗣️ Summary from first commit to {}", Latest),
					);

					Insert::Fn(
						&Summary,
						crate::Fn::Summary::Difference::Fn(&Repository, Latest, &Last, Option)?,
						format!("🗣️ Summary from {} to last commit", Latest),
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
pub mod Group;
pub mod Insert;
