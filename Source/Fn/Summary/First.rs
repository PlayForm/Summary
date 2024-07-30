//! This module provides functionality for generating summaries of the first commit in a git repository.

/// Generates a summary of the first commit in a git repository.
///
/// # Arguments
///
/// * `Repository` - A reference to the git Repository.
/// * `Option` - A reference to a struct containing summary options.
///
/// # Returns
///
/// Returns a Result containing a String with the summary if successful,
/// or a boxed dynamic error if an error occurs.
pub fn Fn(Repository: &Repository) -> Result<Oid, git2::Error> {
	let mut Walk = Repository.revwalk()?;
	Walk.push_head()?;
	Walk.set_sorting(Sort::TOPOLOGICAL | Sort::REVERSE)?;

	match Walk.next() {
		Some(Ok(Identifier)) => Ok(Identifier),
		Some(Err(_Error)) => Err(_Error),
		None => Err(git2::Error::from_str("Cannot git2.")),
	}
}

use git2::{Oid, Repository, Sort};
