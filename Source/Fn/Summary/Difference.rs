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
	let mut Options = git2::DiffOptions::new();

	Options.indent_heuristic(true);
	Options.minimal(true);
	Options.force_text(true);
	Options.ignore_blank_lines(true);
	Options.ignore_case(true);
	Options.ignore_filemode(true);
	Options.ignore_whitespace(true);
	Options.ignore_whitespace_change(true);
	Options.ignore_whitespace_eol(true);

	Repository
		.diff_tree_to_tree(
			Some(&Repository.revparse_single(Start)?.peel_to_commit()?.tree()?),
			Some(&Repository.revparse_single(End)?.peel_to_commit()?.tree()?),
			Some(&mut Options),
		)?
		.print(git2::DiffFormat::Patch, |Delta, _, Line| {
			if !Option
				.Omit
				.iter()
				.map(|Omit: &String| regex::Regex::new(Omit).expect("Cannot Regex."))
				.collect::<Vec<_>>()
				.iter()
				.any(|Omit| {
					Omit.is_match(&Delta.old_file().path().unwrap().display().to_string())
						|| Omit.is_match(&Delta.new_file().path().unwrap().display().to_string())
				}) {
				Difference.push_str(std::str::from_utf8(Line.content()).unwrap());
			};

			true
		})?;

	Ok(Difference)
}

pub struct Option {
	pub Omit: Vec<String>,
}
