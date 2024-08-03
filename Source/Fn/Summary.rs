/// Asynchronously generates a summary of differences between commits in a git repository.
///
/// This function performs the following steps:
/// 1. Opens the specified git repository.
/// 2. Retrieves and sorts the tags in the repository.
/// 3. Identifies the first and last commits in the repository.
/// 4. Generates summaries of differences between the first commit and the last commit, as well as between each pair of consecutive tags.
/// 5. Inserts the generated summaries into a DashMap.
///
/// # Arguments
///
/// * Entry - A string slice representing the path to the git repository.
/// * Option - A reference to a struct containing options for generating the diff summary.
///
/// # Returns
///
/// Returns a Result containing a DashMap with the generated summaries if successful, or a boxed dyn std::error::Error if an error occurs.
///
/// # Errors
///
/// This function will return an error if there are issues with opening the repository, retrieving tags, or generating the diff summaries.
///
/// # Example
///
/// ```rust
/// let options = crate::Struct::Summary::Difference::Struct {
///     Omit: vec!["(?i)\\.log$".to_string()],
/// };
/// let summary = Fn("/path/to/repo", &options).await.expect("Cannot generate summary.");
/// for entry in summary.iter() {
///     println!("{:?}", entry);
/// }
/// ```
///
/// # Panics
///
/// This function does not panic.
pub async fn Fn(
	Entry: &str,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<DashMap<u64, (String, String)>, Box<dyn std::error::Error>> {
	let Summary = DashMap::new();

	let Repository = Arc::new(Repository::open(Entry)?);

	let Name = Repository.tag_names(None)?;

	let mut Date: Vec<(String, DateTime<FixedOffset>)> = Name
		.par_iter()
		.filter_map(|Tag| {
			Tag.and_then(|Tag| {
				Repository
					.revparse_single(&Tag)
					.ok()
					.and_then(|Commit| Commit.peel_to_commit().ok())
					.map(|Commit| {
						(
							Tag.to_string(),
							DateTime::from_timestamp(Commit.time().seconds(), 0)
								.unwrap()
								.fixed_offset(),
						)
					})
			})
		})
		.collect();

	Date.par_sort_by(|A, B| B.1.cmp(&A.1));

	let Tag: Vec<String> = Date.into_iter().map(|(Tag, _)| Tag).collect();

	let Head = Repository.head()?;

	let First = Repository.find_commit(First::Fn(&Repository)?)?.id().to_string();

	let Last = Head.peel_to_commit()?.id().to_string();

	let (Approval, mut Receipt) = mpsc::unbounded_channel();
	let Approval = Arc::new(Mutex::new(Approval));

	let mut Queue = FuturesUnordered::new();

	if Tag.is_empty() {
		let RepositoryClone = Repository.clone();
		let ApprovalClone = Approval.clone();
		let OptionClone = Option.clone();

		Queue.push(tokio::spawn(async move {
			let Summary =
				crate::Fn::Summary::Difference::Fn(&RepositoryClone, &First, &Last, &OptionClone)?;
			ApprovalClone
				.lock()
				.await
				.send(("🗣️ Summary from first commit to last commit".to_string(), Summary))?;
			Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
		}));
	} else {
		for Window in Tag.windows(2) {
			let Start = Window[0].clone();
			let End = Window[1].clone();
			let RepositoryClone = Repository.clone();
			let ApprovalClone = Approval.clone();
			let OptionClone = Option.clone();

			Queue.push(tokio::spawn(async move {
				let Summary = crate::Fn::Summary::Difference::Fn(
					&RepositoryClone,
					&Start,
					&End,
					&OptionClone,
				)?;
				ApprovalClone
					.lock()
					.await
					.send((format!("🗣️ Summary from {} to {}", Start, End), Summary))?;
				Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
			}));
		}

		if let Some(Latest) = Tag.last() {
			let Latest = Latest.clone();
			let RepositoryClone = Repository.clone();
			let ApprovalClone = Approval.clone();
			let OptionClone = Option.clone();

			Queue.push(tokio::spawn(async move {
				let Summary = crate::Fn::Summary::Difference::Fn(
					&RepositoryClone,
					&First,
					&Latest,
					&OptionClone,
				)?;
				ApprovalClone
					.lock()
					.await
					.send((format!("🗣️ Summary from first commit to {}", Latest), Summary))?;
				Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
			}));

			let RepositoryClone = Repository.clone();
			let ApprovalClone = Approval.clone();
			let OptionClone = Option.clone();

			Queue.push(tokio::spawn(async move {
				let Summary = crate::Fn::Summary::Difference::Fn(
					&RepositoryClone,
					&Latest,
					&Last,
					&OptionClone,
				)?;
				ApprovalClone
					.lock()
					.await
					.send((format!("🗣️ Summary from {} to last commit", Latest), Summary))?;
				Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
			}));
		}
	}

	drop(Approval);

	while let Some(Result) = Queue.next().await {
		if let Err(E) = Result {
			eprintln!("Task error: {}", E);
			continue;
		}

		if let Err(E) = Result.unwrap() {
			eprintln!("Inner task error: {}", E);
		}
	}

	while let Some((Message, Difference)) = Receipt.recv().await {
		Insert::Fn(&Summary, Difference, Message);
	}

	Ok(Summary)
}

use chrono::{DateTime, FixedOffset};
use dashmap::DashMap;
use futures::stream::{FuturesUnordered, StreamExt};
use git2::Repository;
use rayon::prelude::*;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

pub mod Difference;
pub mod First;
pub mod Group;
pub mod Insert;
