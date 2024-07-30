/// Function to iterate over the commits in a repository in reverse topological order.
///
/// # Arguments
/// * `Repository` - A reference to the repository to iterate over.
///
/// # Returns
/// * Result containing the next commit in the iteration or an error if no commits are found.
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
