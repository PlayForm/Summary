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
/// let Option = Option {
///     // Fields needed for summary generation
/// };
///
/// let summary = Fn(&Option);
///
/// ```
pub async fn Fn(
	Entry: &str,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<(), Box<dyn std::error::Error>> {
	match Repository::open(Entry) {
		Ok(Repository) => {
			let Name = Repository.tag_names(None)?;

			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			let Count = Tag.len();

			let Head = Repository.head()?;

			let First = Repository.find_commit(First::Fn(&Repository)?)?.id().to_string();

			let Last = Head.peel_to_commit()?.id().to_string();

			match Count {
				0 => {
					println!("Summary from {} to {}:", First, Last);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option,)?
					);
				}
				1 => {
					let Latest = Tag.get(0).unwrap();

					println!("Summary from {} to {}:", First, Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option,)?
					);

					println!("Summary from {} to {}:", Latest, Last);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Latest, &Last, Option,)?
					);
				}
				_ => {
					let Latest = Tag.get(Count - 1).unwrap();
					let Previous = Tag.get(Count - 2).unwrap();

					println!("Summary from {} to {}:", Previous, Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Previous, Latest, Option,)?
					);

					println!("Summary from {} to {}:", Previous, Last);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Previous, &Last, Option,)?
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
pub mod First;
