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
pub fn Fn(Repo: &git2::Repository, Start: &str, End: &str) -> Result<String, git2::Error> {
	let mut Difference = String::new();

	Repo.diff_tree_to_tree(
		Some(&Repo.revparse_single(Start)?.peel_to_commit()?.tree()?),
		Some(&Repo.revparse_single(End)?.peel_to_commit()?.tree()?),
		Some(&mut git2::DiffOptions::new()),
	)?
	.print(git2::DiffFormat::Patch, |_, _, line| {
		Difference.push_str(std::str::from_utf8(line.content()).unwrap());
		true
	})?;

	Ok(Difference)
}
