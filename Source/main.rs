use git2::{DiffOptions, Repository};
use std::{
	fs::{self, File},
	io::Write,
	path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Clone the repository (if you haven't already)
	let repo_url = "<repository_url>";
	let repo_path = "<repository_name>";
	if !Path::new(repo_path).exists() {
		Repository::clone(repo_url, repo_path)?;
	}
	let repo = Repository::open(repo_path)?;

	// List all tags
	let tags = repo.tag_names(None)?;
	let mut previous_tag = None;

	// Create the Summary directory if it doesn't exist
	let summary_dir = "Summary";
	fs::create_dir_all(summary_dir)?;

	for i in 0..tags.len() {
		if let Some(tag) = tags.get(i) {
			if let Some(prev_tag) = previous_tag {
				// Generate diff between previous_tag and current tag
				let diff = Diff(&repo, prev_tag, tag)?;
				let diff_file_path = format!("{}/diff_{}_{}.txt", summary_dir, prev_tag, tag);
				let mut diff_file = File::create(&diff_file_path)?;
				diff_file.write_all(diff.as_bytes())?;

				// Generate release message
				let release_message = Release(&diff);
				let release_file_path = format!("{}/release_{}_{}.txt", summary_dir, prev_tag, tag);
				let mut release_file = File::create(&release_file_path)?;
				release_file.write_all(release_message.as_bytes())?;
			}
			previous_tag = Some(tag);
		}
	}

	Ok(())
}

fn Diff(repo: &Repository, old_tag: &str, new_tag: &str) -> Result<String, git2::Error> {
	let Old = repo.revparse_single(old_tag)?.peel_to_commit()?;
	let New = repo.revparse_single(new_tag)?.peel_to_commit()?;

	let mut Options = DiffOptions::new();
	let diff = repo.diff_tree_to_tree(
		Some(&Old.tree()?),
		Some(&New.tree()?),
		Some(&mut Options),
	)?;

	let mut diff_str = String::new();
	diff.print(git2::DiffFormat::Patch, |_, _, line| {
		diff_str.push_str(std::str::from_utf8(line.content()).unwrap());
		true
	})?;

	Ok(diff_str)
}

fn Release(diff: &str) -> String {
	// This is a simple example. You can enhance this function to generate more detailed messages.
	let mut release_message = String::new();
	release_message.push_str("Release Notes:\n");
	for line in diff.lines() {
		if line.starts_with("+") && !line.starts_with("+++") {
			release_message.push_str(&format!("Added: {}\n", &line[1..]));
		} else if line.starts_with("-") && !line.starts_with("---") {
			release_message.push_str(&format!("Removed: {}\n", &line[1..]));
		}
	}
	release_message
}
use git2::{DiffOptions, Repository};
use std::{
	fs::{self, File},
	io::Write,
	path::Path,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Clone the repository (if you haven't already)
	let repo_url = "<repository_url>";
	let repo_path = "<repository_name>";
	if !Path::new(repo_path).exists() {
		Repository::clone(repo_url, repo_path)?;
	}
	let repo = Repository::open(repo_path)?;

	// List all tags
	let tags = repo.tag_names(None)?;
	let mut previous_tag = None;

	// Create the Summary directory if it doesn't exist
	let summary_dir = "Summary";
	fs::create_dir_all(summary_dir)?;

	for i in 0..tags.len() {
		if let Some(tag) = tags.get(i) {
			if let Some(prev_tag) = previous_tag {
				// Generate diff between previous_tag and current tag
				let diff = Diff(&repo, prev_tag, tag)?;
				let diff_file_path = format!("{}/diff_{}_{}.txt", summary_dir, prev_tag, tag);
				let mut diff_file = File::create(&diff_file_path)?;
				diff_file.write_all(diff.as_bytes())?;

				// Generate release message
				let release_message = Release(&diff);
				let release_file_path = format!("{}/release_{}_{}.txt", summary_dir, prev_tag, tag);
				let mut release_file = File::create(&release_file_path)?;
				release_file.write_all(release_message.as_bytes())?;
			}
			previous_tag = Some(tag);
		}
	}

	Ok(())
}

fn Diff(repo: &Repository, old_tag: &str, new_tag: &str) -> Result<String, git2::Error> {
	let Old = repo.revparse_single(old_tag)?.peel_to_commit()?;
	let New = repo.revparse_single(new_tag)?.peel_to_commit()?;

	let mut Options = DiffOptions::new();
	let diff = repo.diff_tree_to_tree(
		Some(&Old.tree()?),
		Some(&New.tree()?),
		Some(&mut Options),
	)?;

	let mut diff_str = String::new();
	diff.print(git2::DiffFormat::Patch, |_, _, line| {
		diff_str.push_str(std::str::from_utf8(line.content()).unwrap());
		true
	})?;

	Ok(diff_str)
}

fn Release(diff: &str) -> String {
	// This is a simple example. You can enhance this function to generate more detailed messages.
	let mut release_message = String::new();
	release_message.push_str("Release Notes:\n");
	for line in diff.lines() {
		if line.starts_with("+") && !line.starts_with("+++") {
			release_message.push_str(&format!("Added: {}\n", &line[1..]));
		} else if line.starts_with("-") && !line.starts_with("---") {
			release_message.push_str(&format!("Removed: {}\n", &line[1..]));
		}
	}
	release_message
}
