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
pub async fn Fn(Entry: &str) -> Result<(), Box<dyn std::error::Error>> {
	let Repository = Repository::open(Entry)?;

	let Tag = Repository.tag_names(None)?;
	let mut Start = None;

	let Summary = "Summary";
	fs::create_dir_all(Summary)?;

	for i in 0..Tag.len() {
		if let Some(Tag) = Tag.get(i) {
			if let Some(Start) = Start {
				let Difference = crate::Fn::Summary::Difference::Fn(&Repository, Start, Tag)?;

				File::create(&format!("{}/Difference_{}_{}.txt", Summary, Start, Tag))?.write_all(
					crate::Fn::Summary::Difference::Fn(&Repository, Start, Tag)?.as_bytes(),
				)?;

				File::create(&format!("{}/Release_{}_{}.txt", Summary, Start, Tag))?
					.write_all(crate::Fn::Summary::Release::Fn(&Difference).as_bytes())?;
			}

			Start = Some(Tag);
		}
	}

	Ok(())
}

use git2::Repository;
use std::{
	fs::{self, File},
	io::Write,
};

pub mod Difference;
pub mod Release;
