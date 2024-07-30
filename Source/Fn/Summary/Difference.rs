//! This module provides functionality for generating difference summaries between git commits.

/// Generates a difference summary between two git commits.
///
/// # Arguments
///
/// * `Repository` - A reference to the git Repository.
/// * `Start` - The starting commit or reference.
/// * `End` - The ending commit or reference.
/// * `Option` - A reference to a struct containing difference options.
///
/// # Returns
///
/// Returns a Result containing a String with the difference summary if successful,
/// or a boxed dynamic error if an error occurs.
pub fn Fn(
	Repository: &git2::Repository,
	Start: &str,
	End: &str,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<String, git2::Error> {
	let mut Common = vec![
		r"(?i)\.7z$",
		r"(?i)\.accdb$",
		r"(?i)\.avi$",
		r"(?i)\.bak$",
		r"(?i)\.bin$",
		r"(?i)\.bmp$",
		r"(?i)\.class$",
		r"(?i)\.dat$",
		r"(?i)\.db$",
		r"(?i)\.dll$",
		r"(?i)\.dll\.lib$",
		r"(?i)\.dll\.exp$",
		r"(?i)\.doc$",
		r"(?i)\.docx$",
		r"(?i)\.dylib$",
		r"(?i)\.exe$",
		r"(?i)\.flac$",
		r"(?i)\.gif$",
		r"(?i)\.gz$",
		r"(?i)\.heic$",
		r"(?i)\.ico$",
		r"(?i)\.img$",
		r"(?i)\.iso$",
		r"(?i)\.jpeg$",
		r"(?i)\.jpg$",
		r"(?i)\.m4a$",
		r"(?i)\.mdb$",
		r"(?i)\.mkv$",
		r"(?i)\.mov$",
		r"(?i)\.mp3$",
		r"(?i)\.mp4$",
		r"(?i)\.o$",
		r"(?i)\.obj$",
		r"(?i)\.ogg$",
		r"(?i)\.pdb$",
		r"(?i)\.pdf$",
		r"(?i)\.png$",
		r"(?i)\.ppt$",
		r"(?i)\.pptx$",
		r"(?i)\.pyc$",
		r"(?i)\.pyo$",
		r"(?i)\.rar$",
		r"(?i)\.so$",
		r"(?i)\.sqlite$",
		r"(?i)\.svg$",
		r"(?i)\.tar$",
		r"(?i)\.tiff$",
		r"(?i)\.wav$",
		r"(?i)\.webp$",
		r"(?i)\.wmv$",
		r"(?i)\.xls$",
		r"(?i)\.xlsx$",
		r"(?i)\.zip$",
	];

	Common.extend(Option.Omit.iter().map(|Omit| Omit.as_str()));

	let Regex = regex::RegexSet::new(Common).expect("Cannot RegexSet.");

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
	Options.show_binary(false);
	Options.force_binary(false);

	let mut Output = String::new();

	Repository
		.diff_tree_to_tree(
			Some(&Repository.revparse_single(Start)?.peel_to_commit()?.tree()?),
			Some(&Repository.revparse_single(End)?.peel_to_commit()?.tree()?),
			Some(&mut Options),
		)?
		.print(git2::DiffFormat::Patch, |Delta, _, Line| {
			if !Regex.is_match(&Delta.old_file().path().unwrap().display().to_string())
				&& !Regex.is_match(&Delta.new_file().path().unwrap().display().to_string())
			{
				match std::str::from_utf8(Line.content()) {
					Ok(Line) => Output.push_str(Line),
					Err(_) => (),
				}
			}

			true
		})?;

	Ok(Output)
}
