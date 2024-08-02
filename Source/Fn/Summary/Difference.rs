/// Generates a diff summary between two commits in a git repository.
///
/// This function computes the differences between two specified commits in a git repository,
/// while filtering out changes to files that match a set of predefined patterns or user-specified
/// patterns to omit. The resulting diff is returned as a string.
///
/// # Arguments
///
/// * `Repository` - A reference to the git repository.
/// * `Start` - A string slice representing the starting commit hash.
/// * `End` - A string slice representing the ending commit hash.
/// * `Option` - A reference to a struct containing options for generating the diff summary.
///
/// # Returns
///
/// Returns a `Result` containing the diff summary as a `String` if successful, or a `git2::Error`
/// if an error occurs.
///
/// # Errors
///
/// This function will return an error if there are issues with accessing the repository, parsing
/// the commit hashes, or generating the diff.
///
/// # Example
///
/// ```rust
/// let repo = git2::Repository::open("/path/to/repo").expect("Cannot open repository.");
/// let start_commit = "abc123";
/// let end_commit = "def456";
/// let options = crate::Struct::Summary::Difference::Struct {
///     Omit: vec!["(?i)\\.log$".to_string()],
/// };
/// let diff_summary = Fn(&repo, start_commit, end_commit, &options).expect("Cannot generate diff.");
/// println!("{}", diff_summary);
/// ```
///
/// # Panics
///
/// This function will panic if the regex set cannot be created.
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

	let mut FileChanges = HashMap::new();
	let mut CurrentFile = String::new();

	Repository
		.diff_tree_to_tree(
			Some(&Repository.revparse_single(Start)?.peel_to_commit()?.tree()?),
			Some(&Repository.revparse_single(End)?.peel_to_commit()?.tree()?),
			Some(&mut Options),
		)?
		.print(git2::DiffFormat::Patch, |Delta, _, Line| {
			let Path = (
				&Delta.old_file().path().unwrap().display().to_string(),
				&Delta.new_file().path().unwrap().display().to_string(),
			);

			if !Regex.is_match(Path.0) && !Regex.is_match(Path.1) {
				if let Ok(Content) = std::str::from_utf8(Line.content()) {
					match Line.origin() {
						'F' => {
							CurrentFile = format!("{} - {}", Path.0, Path.1);
						}
						'+' | '-' => {
							FileChanges
								.entry(CurrentFile.clone())
								.or_insert_with(Vec::new)
								.push((Line.origin(), Content.trim_end().to_string()));
						}
						_ => (),
					}
				};
			}

			true
		})?;

	let mut Output = String::new();

	for (File, Changes) in FileChanges {
		Output.push_str(&format!("In file '{}':\n", File));
		let (OldContent, NewContent): (Vec<_>, Vec<_>) =
			Changes.iter().partition(|&(origin, _)| *origin == '-');

		let OldText: String =
			OldContent.iter().map(|(_, content)| content.as_str()).collect::<Vec<_>>().join("\n");
		let NewText: String =
			NewContent.iter().map(|(_, content)| content.as_str()).collect::<Vec<_>>().join("\n");

		let Diff = diff::lines(&OldText, &NewText);
		let mut Additions = Vec::new();
		let mut Deletions = Vec::new();
		let mut Modifications = Vec::new();

		for d in Diff {
			match d {
				diff::Result::Left(l) => Deletions.push(l.to_string()),
				diff::Result::Right(r) => Additions.push(r.to_string()),
				diff::Result::Both(l, r) if l != r => {
					Modifications.push((l.to_string(), r.to_string()))
				}
				_ => {}
			}
		}

		if !Additions.is_empty() {
			Output.push_str(&format!("- Added {} line(s):\n", Additions.len()));
			for (i, addition) in Additions.iter().enumerate().take(3) {
				Output.push_str(&format!("  {}. {}\n", i + 1, addition));
			}
			if Additions.len() > 3 {
				Output.push_str(&format!("  ... and {} more additions\n", Additions.len() - 3));
			}
		}

		if !Deletions.is_empty() {
			Output.push_str(&format!("- Removed {} line(s):\n", Deletions.len()));
			for (i, deletion) in Deletions.iter().enumerate().take(3) {
				Output.push_str(&format!("  {}. {}\n", i + 1, deletion));
			}
			if Deletions.len() > 3 {
				Output.push_str(&format!("  ... and {} more deletions\n", Deletions.len() - 3));
			}
		}

		if !Modifications.is_empty() {
			Output.push_str(&format!("- Modified {} line(s):\n", Modifications.len()));
			for (i, (old, new)) in Modifications.iter().enumerate().take(3) {
				Output.push_str(&format!("  {}. Changed from \"{}\" to \"{}\"\n", i + 1, old, new));
			}
			if Modifications.len() > 3 {
				Output.push_str(&format!(
					"  ... and {} more modifications\n",
					Modifications.len() - 3
				));
			}
		}

		Output.push_str("\n");
	}

	Ok(Output)
}

use diff;
use std::collections::HashMap;
use prettytable::{Table, Row, Cell};
