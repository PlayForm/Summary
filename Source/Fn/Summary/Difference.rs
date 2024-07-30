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
	let mut Omit = vec![
		r"\.7z$",
		r"\.accdb$",
		r"\.avi$",
		r"\.bak$",
		r"\.bin$",
		r"\.bmp$",
		r"\.class$",
		r"\.dat$",
		r"\.db$",
		r"\.dll$",
		r"\.dll\.lib$",
		r"\.dll\.exp$",
		r"\.doc$",
		r"\.docx$",
		r"\.dylib$",
		r"\.exe$",
		r"\.flac$",
		r"\.gif$",
		r"\.gz$",
		r"\.heic$",
		r"\.ico$",
		r"\.img$",
		r"\.iso$",
		r"\.jpeg$",
		r"\.jpg$",
		r"\.m4a$",
		r"\.mdb$",
		r"\.mkv$",
		r"\.mov$",
		r"\.mp3$",
		r"\.mp4$",
		r"\.o$",
		r"\.obj$",
		r"\.ogg$",
		r"\.pdb$",
		r"\.pdf$",
		r"\.png$",
		r"\.ppt$",
		r"\.pptx$",
		r"\.pyc$",
		r"\.pyo$",
		r"\.rar$",
		r"\.so$",
		r"\.sqlite$",
		r"\.svg$",
		r"\.tar$",
		r"\.tiff$",
		r"\.wav$",
		r"\.webp$",
		r"\.wmv$",
		r"\.xls$",
		r"\.xlsx$",
		r"\.zip$",
	];

	Omit.extend(Option.Omit.iter().map(|Omit| Omit.as_str()));

	let Regex = Omit.into_par_iter().filter_map(|Omit| Regex::new(Omit).ok()).collect::<Vec<_>>();

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

	let mut Difference = String::new();

	Repository
		.diff_tree_to_tree(
			Some(&Repository.revparse_single(Start)?.peel_to_commit()?.tree()?),
			Some(&Repository.revparse_single(End)?.peel_to_commit()?.tree()?),
			Some(&mut Options),
		)?
		.print(git2::DiffFormat::Patch, |Delta, _, Line| {
			if !Regex.iter().any(|Omit| {
				Omit.is_match(&Delta.old_file().path().unwrap().display().to_string())
					|| Omit.is_match(&Delta.new_file().path().unwrap().display().to_string())
			}) {
				match std::str::from_utf8(Line.content()) {
					Ok(Line) => Difference.push_str(Line),
					Err(_) => (),
				}
			};

			true
		})?;

	Ok(Difference)
}

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
