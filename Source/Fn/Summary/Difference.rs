/// Calculates the difference between two summaries.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for calculating the difference.
///
/// # Returns
///
/// * `Return` - The calculated difference between the summaries.
///
/// # Example
///
/// ```
/// let option = Option {
///     // Fields needed for difference calculation
/// };
/// let difference = Fn(&option);
/// ```
pub fn Fn(
	Repository: &git2::Repository,
	Start: &str,
	End: &str,
	Option: &Option,
) -> Result<String, git2::Error> {
	let mut Difference = String::new();

	Repository
		.diff_tree_to_tree(
			Some(&Repository.revparse_single(Start)?.peel_to_commit()?.tree()?),
			Some(&Repository.revparse_single(End)?.peel_to_commit()?.tree()?),
			Some(
				&mut git2::DiffOptions::new().pathspec(
					std::ffi::CString::new(
						Option
							.Omit
							.iter()
							.map(|Omit| format!("!{}", Omit))
							.collect::<Vec<String>>()
							.join("\0"),
					)
					.expect("Cannot create CString"),
				),
			),
		)?
		.print(git2::DiffFormat::Patch, |_, _, line| {
			Difference.push_str(std::str::from_utf8(line.content()).unwrap());
			true
		})?;

	Ok(Difference)
}

pub struct Option {
	pub Omit: Vec<String>,
}
