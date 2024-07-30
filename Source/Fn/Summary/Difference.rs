use core::fmt::Debug;

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

	// Options.pathspec(
	// 	std::ffi::CString::new(
	// 		std::iter::once("*".to_string())
	// 			.chain(Option.Omit.iter().map(|Omit| format!("{}", Omit)))
	// 			.collect::<Vec<String>>()
	// 			.join("\0"),
	// 	)
	// 	.expect("Cannot create CString"),
	// );

	Options.indent_heuristic(true);

	Options.minimal(true);

	Options.force_text(true);

	Options.ignore_blank_lines(true);
	Options.ignore_case(true);
	Options.ignore_filemode(true);
	Options.ignore_whitespace(true);
	Options.ignore_whitespace_change(true);
	Options.ignore_whitespace_eol(true);

	// Options.pathspec("*\n!tsconfig.json");

	Repository
		.diff_tree_to_tree(
			Some(&Repository.revparse_single(Start)?.peel_to_commit()?.tree()?),
			Some(&Repository.revparse_single(End)?.peel_to_commit()?.tree()?),
			Some(&mut Options),
		)?
		.deltas()
		.for_each(|Delta| {
			for Omit in &Option.Omit {
				if Delta.old_file().path().unwrap().display().to_string().contains(Omit)
					|| Delta.new_file().path().unwrap().display().to_string().contains(Omit)
				{
					println!("{:?}", Delta.status());
				}
			}
		});

	Ok(Difference)
}

pub struct Option {
	pub Omit: Vec<String>,
}
