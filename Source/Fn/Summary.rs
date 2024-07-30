/// Generates a summary based on the provided options.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for generating the summary.
///
/// # Returns
///
/// * `Return` - The generated summary.
///
/// # Example
///
/// ```
/// let option = Option {
///     // Fields needed for summary generation
/// };
/// let summary = Fn(&option);
/// ```
pub async fn Fn(
	Entry: &str,
	Option: &crate::Fn::Summary::Difference::Option,
) -> Result<(), Box<dyn std::error::Error>> {
	match Repository::open(Entry) {
		Ok(Repository) => {
			let Tag = Repository.tag_names(None)?;

			let Tags: Vec<_> = Tag.iter().filter_map(|Tag| Tag).collect();

			for (Index, &Current) in Tags.iter().enumerate() {
				for (_, &Next) in Tags.iter().enumerate().skip(Index + 1) {
					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Current, Next, Option)?
					);
				}
			}
		}
		Err(_Error) => {
			println!("Failed to open repository: {}", _Error);
			return Err(_Error.into());
		}
	}

	Ok(())
}

use git2::Repository;

pub mod Difference;
