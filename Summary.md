🗣️ Summary from Summary/v0.0.1 to Summary/v0.0.2 in .
In file 'Cargo.toml - Cargo.toml':
- Added 1 line(s):
  1. version = "0.0.2"
- Removed 1 line(s):
  1. version = "0.0.1"


🗣️ Summary from Summary/v0.0.2 to Summary/v0.0.3 in .
In file 'Source/Fn/Binary/Command/Parallel.rs - Source/Fn/Binary/Command/Parallel.rs':
- Added 12 line(s):
  1. pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
  2. 	stream::iter(
  3. 	.map(|Entry| {
  ... and 9 more additions
- Removed 11 line(s):
  1. pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
  2. 	let Queue: Vec<_> = stream::iter(
  3. 	.map(|Entry| async move {
  ... and 8 more deletions

In file 'build.rs - build.rs':
- Added 1 line(s):
  1. 
- Removed 1 line(s):
  1. 

In file 'Source/Fn/Binary/Command.rs - Source/Fn/Binary/Command.rs':
- Added 12 line(s):
  1. 			Arg::new("Omit")
  2. 				.short('O')
  3. 				.long("Omit")
  ... and 9 more additions

In file 'Source/Fn/Binary/Command/Sequential.rs - Source/Fn/Binary/Command/Sequential.rs':
- Added 20 line(s):
  1. pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
  2. 	futures::future::join_all(
  3. 			.map(|Entry| {
  ... and 17 more additions
- Removed 3 line(s):
  1. pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
  2. 		.for_each(|_Entry| {
  3. 			// TODO: GENERATE SUMMARY

In file 'Source/Struct/Binary/Command.rs - Source/Struct/Binary/Command.rs':
- Added 1 line(s):
  1. 							Sequential::Fn(Option).await;
- Removed 1 line(s):
  1. 							Sequential::Fn(Option);

In file 'Source/Struct/Binary/Command/Entry.rs - Source/Struct/Binary/Command/Entry.rs':
- Added 7 line(s):
  1. 
  2. 	/// List of items to omit from processing.
  3. 	pub Omit: Omit,
  ... and 4 more additions
- Removed 1 line(s):
  1. use crate::Struct::Binary::Command::Option::{Parallel, Pattern, Separator, Struct as Option};

In file 'Cargo.toml - Cargo.toml':
- Added 2 line(s):
  1. regex = "1.10.5"
  2. version = "0.0.3"
- Removed 1 line(s):
  1. version = "0.0.2"

In file 'Source/Fn/Summary.rs - Source/Fn/Summary.rs':
- Added 18 line(s):
  1. pub async fn Fn(
  2. 	Entry: &str,
  3. 	Option: &crate::Fn::Summary::Difference::Option,
  ... and 15 more additions
- Removed 24 line(s):
  1. pub async fn Fn(Entry: &str) -> Result<(), Box<dyn std::error::Error>> {
  2. 	let Repository = Repository::open(Entry)?;
  3. 
  ... and 21 more deletions

In file 'README.md - README.md':
- Added 44 line(s):
  1. `Summary` is a powerful command-line tool designed for efficient file processing
  2. and summarization. It offers both sequential and parallel processing
  3. capabilities, along with flexible file filtering options.
  ... and 41 more additions
- Removed 7 line(s):
  1. `Summary` is a command-line tool that executes commands in multiple directories
  2. simultaneously. It leverages parallel processing and concurrent `I/O` to
  3. efficiently run tasks across directories.
  ... and 4 more deletions

In file 'Source/Fn/Summary/Release.rs - Source/Fn/Summary/Release.rs':
- Removed 33 line(s):
  1. /// Generates a release summary.
  2. ///
  3. /// # Arguments
  ... and 30 more deletions

In file 'Source/Fn/Summary/Difference.rs - Source/Fn/Summary/Difference.rs':
- Added 50 line(s):
  1. pub fn Fn(
  2. 	Repository: &git2::Repository,
  3. 	Start: &str,
  ... and 47 more additions
- Removed 7 line(s):
  1. pub fn Fn(Repo: &git2::Repository, Start: &str, End: &str) -> Result<String, git2::Error> {
  2. 	Repo.diff_tree_to_tree(
  3. 		Some(&Repo.revparse_single(Start)?.peel_to_commit()?.tree()?),
  ... and 4 more deletions

In file 'Source/Struct/Binary/Command/Option.rs - Source/Struct/Binary/Command/Option.rs':
- Added 15 line(s):
  1. 	/// List of items to omit from processing.
  2. 	pub Omit: Vec<String>,
  3. 
  ... and 12 more additions
- Removed 6 line(s):
  1. 			Exclude: Fn()
  2. 				.map(|Command| Command.to_string())
  3. 			Parallel: Fn().get_flag("Parallel"),
  ... and 3 more deletions


🗣️ Summary from Summary/v0.0.3 to Summary/v0.0.4 in .
In file 'README.md - README.md':
- Added 64 line(s):
  1. `Summary` is a powerful command-line tool designed for efficient Git repository
  2. analysis and summarization. It offers both sequential and parallel processing
  3. ## Features
  ... and 61 more additions
- Removed 40 line(s):
  1. `Summary` is a powerful command-line tool designed for efficient file processing
  2. and summarization. It offers both sequential and parallel processing
  3. ## Feature
  ... and 37 more deletions

In file 'Source/Fn/Summary/Difference.rs - Source/Fn/Summary/Difference.rs':
- Removed 10 line(s):
  1. 	// Options.pathspec(
  2. 	// 	std::ffi::CString::new(
  3. 	// 		std::iter::once("*".to_string())
  ... and 7 more deletions

In file 'Cargo.toml - Cargo.toml':
- Added 1 line(s):
  1. version = "0.0.4"
- Removed 1 line(s):
  1. version = "0.0.3"


🗣️ Summary from Summary/v0.0.4 to Summary/v0.0.5 in .
In file 'Source/Fn/Summary/Difference.rs - Source/Fn/Summary/Difference.rs':
- Added 68 line(s):
  1. 	Option: &crate::Struct::Summary::Difference::Struct,
  2. 	let mut Omit = vec![
  3. 		r"\.pdf$",
  ... and 65 more additions
- Removed 12 line(s):
  1. 	Option: &Option,
  2. 			if !Option
  3. 				.Omit
  ... and 9 more deletions

In file 'README.md - README.md':
- Added 12 line(s):
  1. `Summary` is a powerful command-line tool designed for efficient `Git`
  2. repository analysis and summarization. It offers both sequential and parallel
  3. processing capabilities, along with flexible file filtering options.
  ... and 9 more additions
- Removed 12 line(s):
  1. `Summary` is a powerful command-line tool designed for efficient Git repository
  2. analysis and summarization. It offers both sequential and parallel processing
  3. capabilities, along with flexible file filtering options.
  ... and 9 more deletions

In file 'Source/Struct/mod.rs - Source/Struct/mod.rs':
- Added 1 line(s):
  1. pub mod Summary;

In file 'Source/Fn/Summary.rs - Source/Fn/Summary.rs':
- Added 5 line(s):
  1. 	Option: &crate::Struct::Summary::Difference::Struct,
  2. 			let Name = Repository.tag_names(None)?;
  3. 			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();
  ... and 2 more additions
- Removed 5 line(s):
  1. 	Option: &crate::Fn::Summary::Difference::Option,
  2. 			let Tag = Repository.tag_names(None)?;
  3. 			let Tags: Vec<_> = Tag.iter().filter_map(|Tag| Tag).collect();
  ... and 2 more deletions

In file 'Source/Struct/Summary/Difference.rs - Source/Struct/Summary/Difference.rs':
- Added 3 line(s):
  1. pub struct Struct {
  2. 	pub Omit: Vec<String>,
  3. }

In file 'Cargo.toml - Cargo.toml':
- Added 1 line(s):
  1. version = "0.0.5"
- Removed 1 line(s):
  1. version = "0.0.4"

In file 'Source/Fn/Binary/Command/Sequential.rs - Source/Fn/Binary/Command/Sequential.rs':
- Added 3 line(s):
  1. 						&crate::Struct::Summary::Difference::Struct { Omit },
  2. 	)
  3. 	.await;
- Removed 2 line(s):
  1. 						&crate::Fn::Summary::Difference::Option { Omit },
  2. 	).await;

In file 'Source/Struct/Summary/mod.rs - Source/Struct/Summary/mod.rs':
- Added 1 line(s):
  1. pub mod Difference;

In file 'Source/Fn/Binary/Command/Parallel.rs - Source/Fn/Binary/Command/Parallel.rs':
- Added 8 line(s):
  1. 	futures::stream::iter(
  2. 			match crate::Fn::Summary::Fn(
  3. 				&Entry,
  ... and 5 more additions
- Removed 4 line(s):
  1. 	stream::iter(
  2. 			match crate::Fn::Summary::Fn(&Entry, &crate::Fn::Summary::Difference::Option { Omit })
  3. use futures::stream::{self, StreamExt};
  ... and 1 more deletions


🗣️ Summary from Summary/v0.0.5 to Summary/v0.0.6 in .
In file 'Source/Fn/Summary/Difference.rs - Source/Fn/Summary/Difference.rs':
- Added 43 line(s):
  1. //! This module provides functionality for generating difference summaries between git commits.
  2. 
  3. /// Generates a difference summary between two git commits.
  ... and 40 more additions
- Removed 43 line(s):
  1. /// Calculates the difference between two summaries.
  2. /// * `Option` - A struct containing the necessary information for calculating the difference.
  3. /// * `Return` - The calculated difference between the summaries.
  ... and 40 more deletions

In file 'Source/Struct/Binary/Command/Option.rs - Source/Struct/Binary/Command/Option.rs':
- Added 12 line(s):
  1. //! This module defines structures and functions related to binary command options.
  2. 
  3. /// Represents the structure for binary command options.
  ... and 9 more additions
- Removed 8 line(s):
  1. /// Represents the configuration options for the Summary command.
  2. 	/// List of patterns to exclude from processing.
  3. 	/// List of items to omit from processing.
  ... and 5 more deletions

In file 'Source/Struct/Summary/Difference.rs - Source/Struct/Summary/Difference.rs':
- Added 3 line(s):
  1. //! This module defines structures related to git diff summary options.
  2. 
  3. /// Represents the options for generating a git diff summary.

In file 'Source/Struct/Binary/Command/Entry.rs - Source/Struct/Binary/Command/Entry.rs':
- Added 12 line(s):
  1. //! This module defines structures and functions related to binary command entries.
  2. 
  3. /// Represents the structure for binary command entries.
  ... and 9 more additions
- Removed 7 line(s):
  1. /// Represents the entry options for processing in the Summary command.
  2. 	/// The path.
  3. 	/// Flag indicating whether to use parallel processing.
  ... and 4 more deletions

In file 'Source/Fn/Binary/Command.rs - Source/Fn/Binary/Command.rs':
- Added 7 line(s):
  1. //! This module defines the command-line interface for the Summary application.
  2. 
  3. /// Configures and returns the command-line argument matches for the Summary application.
  ... and 4 more additions
- Removed 10 line(s):
  1. /// Defines and configures command-line arguments for the "Summary" command.
  2. ///
  3. /// * `ArgMatches` - The parsed command-line arguments.
  ... and 7 more deletions

In file 'README.md - README.md':
- Added 13 line(s):
  1. [Summary] is a powerful command-line tool designed for efficient `Git`
  2. ```sh
  3. Summary -P -O Target -O target -O Summary.md -O CHANGELOG.md > Summary.md
  ... and 10 more additions
- Removed 6 line(s):
  1. `Summary` is a powerful command-line tool designed for efficient `Git`
  2. The `Summary` CLI supports [Pieces OS], allowing it to:
  3. By leveraging [Pieces OS], `Summary` can tap into a broader ecosystem of development
  ... and 3 more deletions

In file 'Source/Fn/Binary/Command/Parallel.rs - Source/Fn/Binary/Command/Parallel.rs':
- Added 10 line(s):
  1. //! This module contains functions for parallel command execution in a binary context.
  2. 
  3. /// Executes a sequence of operations asynchronously in parallel based on the provided options.
  ... and 7 more additions
- Removed 14 line(s):
  1. /// Processes entries in parallel, filtering and executing commands based on specified criteria.
  2. /// * `Option` - A struct containing the following fields:
  3. ///   * `Entry`: Vec<Vec<String>> - List of entries to process
  ... and 11 more deletions

In file 'Source/Fn/Summary.rs - Source/Fn/Summary.rs':
- Added 50 line(s):
  1. //! This module provides functionality for generating summaries of git repositories.
  2. 
  3. /// Generates a summary for a given git repository entry.
  ... and 47 more additions
- Removed 15 line(s):
  1. /// Generates a summary based on the provided options.
  2. /// * `Option` - A struct containing the necessary information for generating the summary.
  3. /// * `Return` - The generated summary.
  ... and 12 more deletions

In file 'Source/Fn/Summary/First.rs - Source/Fn/Summary/First.rs':
- Added 26 line(s):
  1. //! This module provides functionality for generating summaries of the first commit in a git repository.
  2. 
  3. /// Generates a summary of the first commit in a git repository.
  ... and 23 more additions

In file 'Source/Library.rs - Source/Library.rs':
- Added 7 line(s):
  1. //! The main entry point for the Summary application.
  2. 
  3. /// The main function that initializes and runs the `Summary` application.
  ... and 4 more additions
- Removed 3 line(s):
  1. /// The main entry point for the Summary application.
  2. /// This function initializes the command structure and executes the appropriate
  3. /// command based on the provided command-line arguments.

In file 'Source/Fn/Binary/Command/Entry.rs - Source/Fn/Binary/Command/Entry.rs':
- Added 5 line(s):
  1. //! This module provides functionality for processing binary command entries.
  2. 
  3. /// Processes entries based on the provided options.
  ... and 2 more additions
- Removed 20 line(s):
  1. /// Walks through a directory and filters files based on specified criteria.
  2. /// * `Option` - A struct containing the following fields:
  3. ///   * `Exclude`: Vec<String> - List of patterns to exclude
  ... and 17 more deletions

In file 'Source/Fn/Binary/Command/Sequential.rs - Source/Fn/Binary/Command/Sequential.rs':
- Added 6 line(s):
  1. //! This module contains functions for sequential command execution in a binary context.
  2. 
  3. /// Executes a sequence of operations asynchronously based on the provided options.
  ... and 3 more additions
- Removed 14 line(s):
  1. /// Processes entries sequentially, filtering and executing commands based on specified criteria.
  2. /// * `Option` - A struct containing the following fields:
  3. ///   * `Entry`: Vec<Vec<String>> - List of entries to process
  ... and 11 more deletions

In file 'Cargo.toml - Cargo.toml':
- Added 2 line(s):
  1. toml = "0.8.17"
  2. version = "0.0.6"
- Removed 2 line(s):
  1. toml = "0.8.16"
  2. version = "0.0.5"

In file 'Source/Struct/Binary/Command.rs - Source/Struct/Binary/Command.rs':
- Added 15 line(s):
  1. //! This module defines the main command structure and its implementation for the binary command execution.
  2. 
  3. /// Represents the main command structure for binary command execution.
  ... and 12 more additions
- Removed 4 line(s):
  1. /// Represents the main command structure for the Summary application.
  2. 	/// The path separator character.
  3. 	/// A boxed function that returns a pinned future.
  ... and 1 more deletions


🗣️ Summary from Summary/v0.0.6 to Summary/v0.0.7 in .
In file 'README.md - README.md':
- Added 46 line(s):
  1. Summary -P > SUMMARY.md
  2. [Summary] will now generate the following [SUMMARY.md](./SUMMARY.md) for all the
  3. commits and tags between the first and the last commit.
  ... and 43 more additions
- Removed 9 line(s):
  1. Summary -P -O Target -O target -O Summary.md -O CHANGELOG.md > Summary.md
  2. [Summary] will now generate the following [Summary.md](./Summary.md) for all the
  3. commits and tags between the first and the latest commit.
  ... and 6 more deletions

In file 'Cargo.toml - Cargo.toml':
- Added 1 line(s):
  1. version = "0.0.7"
- Removed 1 line(s):
  1. version = "0.0.6"

In file 'Source/Fn/Summary.rs - Source/Fn/Summary.rs':
- Added 4 line(s):
  1. 					"{}",
  2. 						"{}",
  3. 						"{}",
  ... and 1 more additions
- Removed 4 line(s):
  1. 					"```\n{}\n```",
  2. 						"```\n{}\n```",
  3. 						"```\n{}\n```",
  ... and 1 more deletions

In file 'Source/Fn/Binary/Command.rs - Source/Fn/Binary/Command.rs':
- Added 10 line(s):
  1. 				.default_values([
  2. 					"Target",
  3. 					"target",
  ... and 7 more additions
- Removed 1 line(s):
  1. 				.default_values(["Target", "Documentation", r"Summary\.md$"]),


🗣️ Summary from Summary/v0.0.7 to Summary/v0.0.8 in .
In file 'Source/Fn/Summary/First.rs - Source/Fn/Summary/First.rs':
- Added 24 line(s):
  1. /// Retrieves the OID of the first commit in the repository.
  2. ///
  3. /// This function initializes a revwalk on the given repository, pushes the HEAD reference onto the
  ... and 21 more additions
- Removed 7 line(s):
  1. //! This module provides functionality for generating summaries of the first commit in a git repository.
  2. 
  3. /// Generates a summary of the first commit in a git repository.
  ... and 4 more deletions

In file 'Source/Struct/Binary/Command/Option.rs - Source/Struct/Binary/Command/Option.rs':
- Added 23 line(s):
  1. /// Represents the options for binary command execution.
  2. ///
  3. /// This struct holds various fields related to the command options, including exclude patterns,
  ... and 20 more additions
- Removed 4 line(s):
  1. //! This module defines structures and functions related to binary command options.
  2. /// Represents the structure for binary command options.
  3. 	/// Creates a new Struct instance from the given options.
  ... and 1 more deletions

In file 'Source/Fn/Summary/Insert.rs - Source/Fn/Summary/Insert.rs':
- Added 35 line(s):
  1. /// Inserts a difference summary into the provided DashMap.
  2. ///
  3. /// This function computes the hash of the given difference string and inserts it into the DashMap
  ... and 32 more additions

In file 'Source/Library.rs - Source/Library.rs':
- Added 16 line(s):
  1. /// The main entry point for the application.
  2. /// This function initializes the command structure and executes the asynchronous function
  3. /// defined within it. The function is marked with the `#[tokio::main]` attribute to enable
  ... and 13 more additions
- Removed 6 line(s):
  1. //! The main entry point for the Summary application.
  2. 
  3. /// The main function that initializes and runs the `Summary` application.
  ... and 3 more deletions

In file 'README.md - README.md':
- Added 12 line(s):
  1. Summary -P > Summary.md
  2. [Summary] will now generate the following [Summary.md](./Summary.md) for all the
  3. Summary -P -E node_modules
  ... and 9 more additions
- Removed 16 line(s):
  1. Summary -P > SUMMARY.md
  2. [Summary] will now generate the following [SUMMARY.md](./SUMMARY.md) for all the
  3. Summary -E node_modules
  ... and 13 more deletions

In file 'Source/Fn/Summary.rs - Source/Fn/Summary.rs':
- Added 53 line(s):
  1. /// Asynchronously generates a summary of differences between commits in a git repository.
  2. ///
  3. /// This function performs the following steps:
  ... and 50 more additions
- Removed 30 line(s):
  1. //! This module provides functionality for generating summaries of git repositories.
  2. /// Generates a summary for a given git repository entry.
  3. /// * `Entry` - A string representing the repository path.
  ... and 27 more deletions

In file 'Source/Fn/Binary/Command/Parallel.rs - Source/Fn/Binary/Command/Parallel.rs':
- Added 74 line(s):
  1. /// Asynchronously processes entries to generate summaries and outputs the results.
  2. ///
  3. /// This function performs the following steps:
  ... and 71 more additions
- Removed 22 line(s):
  1. //! This module contains functions for parallel command execution in a binary context.
  2. /// Executes a sequence of operations asynchronously in parallel based on the provided options.
  3. /// * `Option` - A struct containing various options for execution, including:
  ... and 19 more deletions

In file 'Source/Fn/Summary/Difference.rs - Source/Fn/Summary/Difference.rs':
- Added 96 line(s):
  1. /// Generates a diff summary between two commits in a git repository.
  2. ///
  3. /// This function computes the differences between two specified commits in a git repository,
  ... and 93 more additions
- Removed 76 line(s):
  1. //! This module provides functionality for generating difference summaries between git commits.
  2. 
  3. /// Generates a difference summary between two git commits.
  ... and 73 more deletions

In file 'Source/Fn/Summary/Insert/Hash.rs - Source/Fn/Summary/Insert/Hash.rs':
- Added 41 line(s):
  1. /// Computes the hash of the given input using the `DefaultHasher`.
  2. ///
  3. /// This function takes any input that implements the `Hash` trait and computes its hash value
  ... and 38 more additions

In file 'Source/Struct/Binary/Command.rs - Source/Struct/Binary/Command.rs':
- Added 8 line(s):
  1. /// Represents the structure for binary command execution.
  2. ///
  3. /// This struct holds various fields related to the command execution, including the separator for file paths
  ... and 5 more additions
- Removed 6 line(s):
  1. //! This module defines the main command structure and its implementation for the binary command execution.
  2. 
  3. /// Represents the main command structure for binary command execution.
  ... and 3 more deletions

In file 'Cargo.toml - Cargo.toml':
- Added 2 line(s):
  1. dashmap = "6.0.1"
  2. version = "0.0.8"
- Removed 1 line(s):
  1. version = "0.0.7"

In file 'Source/Fn/Binary/Command/Sequential.rs - Source/Fn/Binary/Command/Sequential.rs':
- Added 25 line(s):
  1. /// Asynchronously processes entries to generate summaries and outputs the results sequentially.
  2. ///
  3. /// This function performs the following steps:
  ... and 22 more additions
- Removed 6 line(s):
  1. //! This module contains functions for sequential command execution in a binary context.
  2. 
  3. /// Executes a sequence of operations asynchronously based on the provided options.
  ... and 3 more deletions

In file 'Source/Fn/Binary/Command/Entry.rs - Source/Fn/Binary/Command/Entry.rs':
- Added 28 line(s):
  1. /// Generates a list of file paths from the specified root directory, excluding paths that match
  2. /// any of the specified exclude patterns.
  3. /// * `Option` - A reference to an `Option` struct containing the following fields:
  ... and 25 more additions
- Removed 5 line(s):
  1. //! This module provides functionality for processing binary command entries.
  2. 
  3. /// Processes entries based on the provided options.
  ... and 2 more deletions

In file 'Source/Struct/Binary/Command/Entry.rs - Source/Struct/Binary/Command/Entry.rs':
- Added 13 line(s):
  1. ///
  2. /// This struct holds various fields related to the command entries, including the entry paths,
  3. /// parallel execution flag, pattern to match, separator for file paths, and omit patterns.
  ... and 10 more additions
- Removed 3 line(s):
  1. //! This module defines structures and functions related to binary command entries.
  2. 
  3. 	/// Creates a new Struct instance from the given options.

In file 'Source/Struct/Summary/Difference.rs - Source/Struct/Summary/Difference.rs':
- Added 16 line(s):
  1. /// Represents a structure containing omit patterns.
  2. ///
  3. /// This struct holds a vector of strings representing patterns to omit.
  ... and 13 more additions
- Removed 3 line(s):
  1. //! This module defines structures related to git diff summary options.
  2. 
  3. /// Represents the options for generating a git diff summary.

In file 'Source/Fn/Binary/Command.rs - Source/Fn/Binary/Command.rs':
- Added 36 line(s):
  1. /// Creates and returns the command-line argument matches for the `Summary` application.
  2. /// This function sets up the command-line interface using the `clap` crate, defining various
  3. /// arguments and their properties such as short and long names, help messages, default values,
  ... and 33 more additions
- Removed 14 line(s):
  1. //! This module defines the command-line interface for the Summary application.
  2. 
  3. /// Configures and returns the command-line argument matches for the Summary application.
  ... and 11 more deletions


🗣️ Summary from Summary/v0.0.8 to Summary/v0.0.9 in .
In file 'Cargo.toml - Cargo.toml':
- Added 9 line(s):
  1. version = "0.0.9"
  2. include = [
  3. 	"Source/**/*",
  ... and 6 more additions
- Removed 1 line(s):
  1. version = "0.0.8"


🗣️ Summary from Summary/v0.0.9 to Summary/v0.1.0 in .
In file 'Source/Fn/Binary/Command/Parallel.rs - Source/Fn/Binary/Command/Parallel.rs':
- Added 9 line(s):
  1. 	let Queue = futures::stream::FuturesUnordered::new();
  2. 	for Entry in Entry
  3. 		.collect::<Vec<String>>()
  ... and 6 more additions
- Removed 30 line(s):
  1. 	let Entry = Entry
  2. 		.collect::<Vec<String>>();
  3. 
  ... and 27 more deletions

In file 'Source/Fn/Binary/Command/Sequential.rs - Source/Fn/Binary/Command/Sequential.rs':
- Added 5 line(s):
  1. 	let Queue = futures::future::join_all(
  2. 						Ok(Summary) => Ok((Entry, Summary)),
  3. 			}),
  ... and 2 more additions
- Removed 4 line(s):
  1. 	futures::future::join_all(
  2. 						Ok(Summary) => Ok(Summary),
  3. 			})
  ... and 1 more deletions

In file 'Cargo.toml - Cargo.toml':
- Added 2 line(s):
  1. itertools = "0.13.0"
  2. version = "0.1.0"
- Removed 1 line(s):
  1. version = "0.0.9"

In file 'Source/Fn/Summary.rs - Source/Fn/Summary.rs':
- Added 3 line(s):
  1. 						format!("🗣️ Summary from first commit to {}", Latest),
  2. 						format!("🗣️ Summary from {} to last commit", Latest),
  3. pub mod Group;
- Removed 2 line(s):
  1. 						format!("🗣️ Summary from first commit to latest {}", Latest),
  2. 						format!("🗣️ Summary from latest {} to last commit", Latest),

In file 'Source/Fn/Summary/Group.rs - Source/Fn/Summary/Group.rs':
- Added 81 line(s):
  1. /// Processes and prints summaries of differences.
  2. ///
  3. /// This function takes an iterator of summaries, processes them to aggregate differences
  ... and 78 more additions


🗣️ Summary from Summary/v0.1.0 to Summary/v0.1.1 in .
In file 'Source/Fn/Summary.rs - Source/Fn/Summary.rs':

In file 'Source/Fn/Summary/Difference.rs - Source/Fn/Summary/Difference.rs':
- Added 13 line(s):
  1. 			let Path = (
  2. 				&Delta.old_file().path().unwrap().display().to_string(),
  3. 				&Delta.new_file().path().unwrap().display().to_string(),
  ... and 10 more additions
- Removed 6 line(s):
  1. 			if !Regex.is_match(&Delta.old_file().path().unwrap().display().to_string())
  2. 				&& !Regex.is_match(&Delta.new_file().path().unwrap().display().to_string())
  3. 			{
  ... and 3 more deletions

In file 'Cargo.toml - Cargo.toml':
- Added 1 line(s):
  1. version = "0.1.1"
- Removed 1 line(s):
  1. version = "0.1.0"


🗣️ Summary from Summary/v0.1.1 to last commit in .

🗣️ Summary from first commit to Summary/v0.1.1 in .
In file 'Source/Fn/Binary/Command/Entry.rs - Source/Fn/Binary/Command/Entry.rs':
- Added 59 line(s):
  1. /// Generates a list of file paths from the specified root directory, excluding paths that match
  2. /// any of the specified exclude patterns.
  3. ///
  ... and 56 more additions

In file 'Source/Struct/Binary/Command.rs - Source/Struct/Binary/Command.rs':
- Added 50 line(s):
  1. /// Represents the structure for binary command execution.
  2. ///
  3. /// This struct holds various fields related to the command execution, including the separator for file paths
  ... and 47 more additions

In file 'Source/Struct/Binary/Command/Entry.rs - Source/Struct/Binary/Command/Entry.rs':
- Added 51 line(s):
  1. /// Represents the structure for binary command entries.
  2. ///
  3. /// This struct holds various fields related to the command entries, including the entry paths,
  ... and 48 more additions

In file '.github/FUNDING.yml - .github/FUNDING.yml':
- Added 1 line(s):
  1. open_collective: playform-cloud-collective

In file 'Source/Struct/Summary/mod.rs - Source/Struct/Summary/mod.rs':
- Added 1 line(s):
  1. pub mod Difference;

In file 'Source/Fn/mod.rs - Source/Fn/mod.rs':
- Added 2 line(s):
  1. pub mod Binary;
  2. pub mod Summary;

In file 'Source/Fn/Binary/Command.rs - Source/Fn/Binary/Command.rs':
- Added 104 line(s):
  1. /// Creates and returns the command-line argument matches for the `Summary` application.
  2. ///
  3. /// This function sets up the command-line interface using the `clap` crate, defining various
  ... and 101 more additions

In file 'CONTRIBUTING.md - CONTRIBUTING.md':
- Added 134 line(s):
  1. # Contributing Guidelines
  2. 
  3. Welcome to our community! We are committed to creating a welcoming and inclusive
  ... and 131 more additions

In file '.github/dependabot.yml - .github/dependabot.yml':
- Added 14 line(s):
  1. version: 2
  2. enable-beta-ecosystems: true
  3. 
  ... and 11 more additions

In file 'Source/Fn/Summary/Difference.rs - Source/Fn/Summary/Difference.rs':
- Added 149 line(s):
  1. /// Generates a diff summary between two commits in a git repository.
  2. ///
  3. /// This function computes the differences between two specified commits in a git repository,
  ... and 146 more additions

In file 'build.rs - build.rs':
- Added 26 line(s):
  1. #![allow(non_snake_case)]
  2. 
  3. #[derive(Deserialize)]
  ... and 23 more additions

In file '.github/workflows/Rust.yml - .github/workflows/Rust.yml':
- Added 79 line(s):
  1. name: Rust
  2. 
  3. concurrency:
  ... and 76 more additions

In file 'Source/Fn/Summary/First.rs - Source/Fn/Summary/First.rs':
- Added 43 line(s):
  1. /// Retrieves the OID of the first commit in the repository.
  2. ///
  3. /// This function initializes a revwalk on the given repository, pushes the HEAD reference onto the
  ... and 40 more additions

In file 'Source/Fn/Summary/Group.rs - Source/Fn/Summary/Group.rs':
- Added 81 line(s):
  1. /// Processes and prints summaries of differences.
  2. ///
  3. /// This function takes an iterator of summaries, processes them to aggregate differences
  ... and 78 more additions

In file 'CODE_OF_CONDUCT.md - CODE_OF_CONDUCT.md':
- Added 146 line(s):
  1. # Code of Conduct
  2. 
  3. ## Our Pledge
  ... and 143 more additions

In file '.gitignore - .gitignore':
- Added 9 line(s):
  1. /Target/*
  2. 
  3. !/Target/release
  ... and 6 more additions
- Removed 1 line(s):
  1. /target

In file 'Source/Library.rs - Source/Library.rs':
- Added 28 line(s):
  1. #![allow(non_snake_case)]
  2. 
  3. #[allow(dead_code)]
  ... and 25 more additions

In file 'Source/Struct/mod.rs - Source/Struct/mod.rs':
- Added 2 line(s):
  1. pub mod Binary;
  2. pub mod Summary;

In file '.github/workflows/Dependabot.yml - .github/workflows/Dependabot.yml':
- Added 45 line(s):
  1. name: Dependabot
  2. 
  3. concurrency:
  ... and 42 more additions

In file 'LICENSE - LICENSE':
- Added 20 line(s):
  1. MIT License
  2. 
  3. Copyright (c) 2023-2024 PlayForm
  ... and 17 more additions

In file 'Cargo.toml - Cargo.toml':
- Added 44 line(s):
  1. [[bin]]
  2. name = "PSummary"
  3. path = "Source/Library.rs"
  ... and 41 more additions
- Removed 1 line(s):
  1. version = "0.1.0"

In file '.cargo/Config.toml - .cargo/Config.toml':
- Added 12 line(s):
  1. [build]
  2. target-dir = "Target"
  3. 
  ... and 9 more additions

In file 'src/main.rs - src/main.rs':
- Removed 3 line(s):
  1. fn main() {
  2. 	println!("Hello, world!");
  3. }

In file 'Source/Fn/Summary/Insert/Hash.rs - Source/Fn/Summary/Insert/Hash.rs':
- Added 41 line(s):
  1. /// Computes the hash of the given input using the `DefaultHasher`.
  2. ///
  3. /// This function takes any input that implements the `Hash` trait and computes its hash value
  ... and 38 more additions

In file 'Source/Fn/Binary/Command/Parallel.rs - Source/Fn/Binary/Command/Parallel.rs':
- Added 82 line(s):
  1. /// Asynchronously processes entries to generate summaries and outputs the results.
  2. ///
  3. /// This function performs the following steps:
  ... and 79 more additions

In file 'Source/Fn/Binary/Command/Sequential.rs - Source/Fn/Binary/Command/Sequential.rs':
- Added 64 line(s):
  1. /// Asynchronously processes entries to generate summaries and outputs the results sequentially.
  2. ///
  3. /// This function performs the following steps:
  ... and 61 more additions

In file 'Source/Struct/Summary/Difference.rs - Source/Struct/Summary/Difference.rs':
- Added 19 line(s):
  1. /// Represents a structure containing omit patterns.
  2. ///
  3. /// This struct holds a vector of strings representing patterns to omit.
  ... and 16 more additions

In file 'Source/Fn/Summary.rs - Source/Fn/Summary.rs':
- Added 108 line(s):
  1. /// Asynchronously generates a summary of differences between commits in a git repository.
  2. ///
  3. /// This function performs the following steps:
  ... and 105 more additions

In file 'Source/Fn/Binary/mod.rs - Source/Fn/Binary/mod.rs':
- Added 1 line(s):
  1. pub mod Command;

In file 'Source/Struct/Binary/Command/Option.rs - Source/Struct/Binary/Command/Option.rs':
- Added 74 line(s):
  1. /// Represents the options for binary command execution.
  2. ///
  3. /// This struct holds various fields related to the command options, including exclude patterns,
  ... and 71 more additions

In file 'README.md - README.md':
- Added 173 line(s):
  1. # 🗣️ [Summary] —
  2. 
  3. [Summary] is a powerful command-line tool designed for efficient `Git`
  ... and 170 more additions

In file 'Source/Struct/Binary/mod.rs - Source/Struct/Binary/mod.rs':
- Added 1 line(s):
  1. pub mod Command;

In file '.github/workflows/GitHub.yml - .github/workflows/GitHub.yml':
- Added 58 line(s):
  1. name: GitHub
  2. 
  3. concurrency:
  ... and 55 more additions

In file 'Source/Fn/Summary/Insert.rs - Source/Fn/Summary/Insert.rs':
- Added 35 line(s):
  1. /// Inserts a difference summary into the provided DashMap.
  2. ///
  3. /// This function computes the hash of the given difference string and inserts it into the DashMap
  ... and 32 more additions


