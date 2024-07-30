
🗣️ Summary from latest tag: Summary/v0.0.6 to last commit: 553a8e80b8c1d738fce5e967f0bce8a9dace1bcb:
diff --git a/README.md b/README.md
index 786f795..8963bab 100644
--- a/README.md
+++ b/README.md
@@ -1,11 +1,18 @@
# 🗣️ [Summary] —

`Summary` is a powerful command-line tool designed for efficient `Git`
[Summary] is a powerful command-line tool designed for efficient `Git`
repository analysis and summarization. It offers both sequential and parallel
processing capabilities, along with flexible file filtering options.

[Summary]: HTTPS://crates.io/crates/psummary

```sh
Summary -P -O target -O Summary.md > Summary.md
```

[Summary] will now generate the following [Summary.md](./Summary.md) for all the
commits and tags between the first and the latest commit.

## Features

-   Customizable file pattern matching.
@@ -18,14 +25,14 @@ processing capabilities, along with flexible file filtering options.

## [Pieces OS] Integration

The `Summary` CLI supports [Pieces OS], allowing it to:
The [Summary] CLI supports [Pieces OS], allowing it to:

-   Generate comprehensive diff logs and release notes automatically.
-   Provide AI-driven code analysis and insights.
-   Offer improved context-aware processing of repository changes.
-   Seamlessly interact with other [Pieces OS]-compatible development tools.

By leveraging [Pieces OS], `Summary` can tap into a broader ecosystem of development
By leveraging [Pieces OS], [Summary] can tap into a broader ecosystem of development
tools and services, significantly expanding its capabilities beyond basic file processing.

## Installation
@@ -58,7 +65,7 @@ repository.

## Options

The `Summary` tool can be used with various options:
The [Summary] tool can be used with various options:

#### --Exclude or -E:

@@ -112,7 +119,7 @@ Summary -O "\.md$" -O "\.txt$"

## Dependencies

`Summary` relies on several Rust crates to provide its functionality:
[Summary] relies on several Rust crates to provide its functionality:

-   `clap` - For parsing command-line arguments.
-   `futures` - For asynchronous programming abstractions.
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index 4cfbb9f..e355a33 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -1,16 +1,13 @@
/// Defines and configures command-line arguments for the "Summary" command.
///
/// # Returns
//! This module defines the command-line interface for the Summary application.

/// Configures and returns the command-line argument matches for the Summary application.
///
/// * `ArgMatches` - The parsed command-line arguments.
/// This function sets up the command-line interface using the clap crate, defining
/// various arguments and their properties.
///
/// # Example
/// # Returns
///
/// ```
/// let matches = Fn();
/// let parallel = matches.get_flag("Parallel");
/// let root = matches.get_one::<String>("Root").unwrap();
/// ```
/// Returns an `ArgMatches` struct containing the parsed command-line arguments.
pub fn Fn() -> ArgMatches {
	Command::new("Summary")
		.version(env!("CARGO_PKG_VERSION"))
@@ -35,7 +32,7 @@ pub fn Fn() -> ArgMatches {
				.required(false)
				.help("🚫 Omit —")
				.action(clap::ArgAction::Append)
				.default_value("Documentation"),
				.default_values(["Target", "Documentation", r"Summary\.md$"]),
		)
		.arg(
			Arg::new("Parallel")
diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
index 64351c7..387c0e4 100644
--- a/Source/Fn/Binary/Command/Entry.rs
+++ b/Source/Fn/Binary/Command/Entry.rs
@@ -1,29 +1,14 @@
/// Walks through a directory and filters files based on specified criteria.
//! This module provides functionality for processing binary command entries.

/// Processes entries based on the provided options.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   * `Exclude`: Vec<String> - List of patterns to exclude
///   * `Pattern`: String - The pattern to match for inclusion
///   * `Root`: String - The root directory to start the walk from
///   * `Separator`: char - The path separator character
/// * `Option` - A reference to an Option struct containing processing parameters.
///
/// # Returns
///
/// * `Return` - A vector of vectors of strings, where each inner vector represents a file path
///   split into its components.
///
/// # Example
///
/// ```
/// let option = Option {
///     Exclude: vec!["node_modules".to_string()],
///     Pattern: ".git".to_string(),
///     Root: ".".to_string(),
///     Separator: std::path::MAIN_SEPARATOR,
/// };
/// let result = Fn(&option);
/// ```
/// Returns a vector of processed entries.
pub fn Fn(Option { Exclude, Pattern, Root, Separator, .. }: &Option) -> Return {
	WalkDir::new(Root)
		.follow_links(false)
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 22654c0..75ef690 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -1,22 +1,18 @@
/// Processes entries in parallel, filtering and executing commands based on specified criteria.
//! This module contains functions for parallel command execution in a binary context.

/// Executes a sequence of operations asynchronously in parallel based on the provided options.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   * `Entry`: Vec<Vec<String>> - List of entries to process
///   * `Separator`: char - The path separator character
///   * `Pattern`: String - The pattern to match for inclusion
/// * `Option` - A struct containing various options for execution, including:
///   - `Entry`: A collection of entries to process
///   - `Separator`: A separator used for joining entry parts
///   - `Pattern`: A pattern to match against the last element of each entry
///   - `Omit`: A collection of items to omit from processing
///
/// # Example
/// # Async
///
/// ```
/// let option = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.txt".to_string()]],
///     Separator: '/',
///     Pattern: "file.txt".to_string(),
/// };
/// Fn(option).await;
/// ```
/// This function is asynchronous and returns a future.
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	futures::stream::iter(
		Entry
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index 7ffeb0f..a84435b 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -1,22 +1,14 @@
/// Processes entries sequentially, filtering and executing commands based on specified criteria.
//! This module contains functions for sequential command execution in a binary context.

/// Executes a sequence of operations asynchronously based on the provided options.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   * `Entry`: Vec<Vec<String>> - List of entries to process
///   * `Pattern`: String - The pattern to match for inclusion
///   * `Separator`: char - The path separator character
/// * `Option` - A struct containing various options for execution.
///
/// # Example
/// # Async
///
/// ```
/// let option = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.txt".to_string()]],
///     Pattern: "file.txt".to_string(),
///     Separator: '/',
/// };
/// Fn(option);
/// ```
/// This function is asynchronous and returns a future.
pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
	futures::future::join_all(
		Entry
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index 0e302ee..016d449 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -1,23 +1,20 @@
/// Generates a summary based on the provided options.
//! This module provides functionality for generating summaries of git repositories.

/// Generates a summary for a given git repository entry.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for generating the summary.
/// * `Entry` - A string representing the repository path.
/// * `Option` - A reference to a struct containing summary options.
///
/// # Returns
///
/// * `Return` - The generated summary.
///
/// # Example
///
/// ```
/// let Option = Option {
///     // Fields needed for summary generation
/// };
/// Returns a Result containing () if successful, or a boxed dynamic error if an error occurs.
///
/// let summary = Fn(&Option);
/// # Errors
///
/// ```
/// This function will return an error if the repository cannot be opened or if there are issues
/// generating the summary.
pub async fn Fn(
	Entry: &str,
	Option: &crate::Struct::Summary::Difference::Struct,
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
index cc25057..544095b 100644
--- a/Source/Fn/Summary/Difference.rs
+++ b/Source/Fn/Summary/Difference.rs
@@ -1,21 +1,18 @@
/// Calculates the difference between two summaries.
//! This module provides functionality for generating difference summaries between git commits.

/// Generates a difference summary between two git commits.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for calculating the difference.
/// * `Repository` - A reference to the git Repository.
/// * `Start` - The starting commit or reference.
/// * `End` - The ending commit or reference.
/// * `Option` - A reference to a struct containing difference options.
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
/// Returns a Result containing a String with the difference summary if successful,
/// or a boxed dynamic error if an error occurs.
pub fn Fn(
	Repository: &git2::Repository,
	Start: &str,
@@ -23,56 +20,59 @@ pub fn Fn(
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<String, git2::Error> {
	let mut Omit = vec![
		r"\.pdf$",
		r"\.exe$",
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
		r"\.so$",
		r"\.dll\.lib$",
		r"\.dll\.exp$",
		r"\.doc$",
		r"\.docx$",
		r"\.dylib$",
		r"\.zip$",
		r"\.tar$",
		r"\.gz$",
		r"\.7z$",
		r"\.rar$",
		r"\.jpg$",
		r"\.jpeg$",
		r"\.png$",
		r"\.exe$",
		r"\.flac$",
		r"\.gif$",
		r"\.bmp$",
		r"\.tiff$",
		r"\.ico$",
		r"\.svg$",
		r"\.webp$",
		r"\.gz$",
		r"\.heic$",
		r"\.mp3$",
		r"\.wav$",
		r"\.ogg$",
		r"\.flac$",
		r"\.ico$",
		r"\.img$",
		r"\.iso$",
		r"\.jpeg$",
		r"\.jpg$",
		r"\.m4a$",
		r"\.mp4$",
		r"\.avi$",
		r"\.mov$",
		r"\.mdb$",
		r"\.mkv$",
		r"\.wmv$",
		r"\.doc$",
		r"\.docx$",
		r"\.xls$",
		r"\.xlsx$",
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
		r"\.db$",
		r"\.sqlite$",
		r"\.mdb$",
		r"\.accdb$",
		r"\.class$",
		r"\.pyc$",
		r"\.pyo$",
		r"\.o$",
		r"\.obj$",
		r"\.bin$",
		r"\.dat$",
		r"\.bak$",
		r"\.iso$",
		r"\.img$",
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
diff --git a/Source/Fn/Summary/First.rs b/Source/Fn/Summary/First.rs
index 7bfdeee..a0c3df6 100644
--- a/Source/Fn/Summary/First.rs
+++ b/Source/Fn/Summary/First.rs
@@ -1,10 +1,16 @@
/// Function to iterate over the commits in a repository in reverse topological order.
//! This module provides functionality for generating summaries of the first commit in a git repository.

/// Generates a summary of the first commit in a git repository.
///
/// # Arguments
/// * `Repository` - A reference to the repository to iterate over.
///
/// * `Repository` - A reference to the git Repository.
/// * `Option` - A reference to a struct containing summary options.
///
/// # Returns
/// * Result containing the next commit in the iteration or an error if no commits are found.
///
/// Returns a Result containing a String with the summary if successful,
/// or a boxed dynamic error if an error occurs.
pub fn Fn(Repository: &Repository) -> Result<Oid, git2::Error> {
	let mut Walk = Repository.revwalk()?;
	Walk.push_head()?;
diff --git a/Source/Library.rs b/Source/Library.rs
index 6864249..7b297ec 100644
--- a/Source/Library.rs
+++ b/Source/Library.rs
@@ -1,9 +1,13 @@
//! The main entry point for the Summary application.

#![allow(non_snake_case)]

/// The main entry point for the Summary application.
/// The main function that initializes and runs the `Summary` application.
///
/// # Errors
///
/// This function initializes the command structure and executes the appropriate
/// command based on the provided command-line arguments.
/// This function will return an error if there are issues parsing arguments
/// or executing the requested commands.
#[allow(dead_code)]
#[tokio::main]
async fn main() {
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
index 4a356ae..7473837 100644
--- a/Source/Struct/Binary/Command.rs
+++ b/Source/Struct/Binary/Command.rs
@@ -1,12 +1,23 @@
/// Represents the main command structure for the Summary application.
//! This module defines the main command structure and its implementation for the binary command execution.

/// Represents the main command structure for binary command execution.
pub struct Struct {
	/// The path separator character.
	/// The separator used for file paths.
	pub Separator: Option::Separator,
	/// A boxed function that returns a pinned future.

	/// A boxed function that returns a pinned future representing the command execution.
	pub Fn: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static>,
}

impl Struct {
	/// Creates a new instance of the Struct.
	///
	/// This function initializes the Struct with the system's main separator
	/// and a boxed async function that handles command execution.
	///
	/// # Returns
	///
	/// Returns a new instance of Struct.
	pub fn Fn() -> Self {
		Self {
			Separator: std::path::MAIN_SEPARATOR,
@@ -28,10 +39,10 @@ impl Struct {
	}
}

use crate::Fn::Binary::Command::{Parallel, Sequential};

use futures::Future;
use std::pin::Pin;

pub mod Entry;
pub mod Option;

use crate::Fn::Binary::Command::{Parallel, Sequential};
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
index 45d3e41..ee24305 100644
--- a/Source/Struct/Binary/Command/Entry.rs
+++ b/Source/Struct/Binary/Command/Entry.rs
@@ -1,22 +1,28 @@
/// Represents the entry options for processing in the Summary command.
//! This module defines structures and functions related to binary command entries.

/// Represents the structure for binary command entries.
pub struct Struct {
	/// The path.
	pub Entry: Type,

	/// Flag indicating whether to use parallel processing.
	pub Parallel: Parallel,

	/// The pattern to match for inclusion in processing.
	pub Pattern: Pattern,

	/// The path separator character.
	pub Separator: Separator,

	/// List of items to omit from processing.
	pub Omit: Omit,
}

impl Struct {
	/// Creates a new Struct instance from the given options.
	///
	/// # Arguments
	///
	/// * `Option` - A reference to an Option struct containing initialization parameters.
	///
	/// # Returns
	///
	/// Returns a new instance of Struct.
	pub fn Fn(Option: &Option) -> Self {
		Self {
			Entry: crate::Fn::Binary::Command::Entry::Fn(Option),
@@ -32,5 +38,4 @@ use crate::Struct::Binary::Command::Option::{
	Omit, Parallel, Pattern, Separator, Struct as Option,
};

/// Defines a type alias for a vector of vectors of strings.
pub type Type = Vec<Vec<String>>;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index 8d32f31..3f069ff 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
@@ -1,26 +1,30 @@
/// Represents the configuration options for the Summary command.
//! This module defines structures and functions related to binary command options.

/// Represents the structure for binary command options.
pub struct Struct {
	/// List of patterns to exclude from processing.
	pub Exclude: Vec<String>,

	/// List of items to omit from processing.
	pub Omit: Vec<String>,

	/// Flag indicating whether to use parallel processing.
	pub Parallel: Parallel,

	/// The pattern to match for inclusion in processing.
	pub Pattern: Pattern,

	/// The root directory to start processing from.
	pub Root: String,

	/// The path separator character.
	pub Separator: Separator,
}

impl Struct {
	/// Creates a new Struct instance from the provided Option.
	/// Creates a new Struct instance from the given options.
	///
	/// # Arguments
	///
	/// * `Option` - An Option struct containing initialization parameters.
	///
	/// # Returns
	///
	/// Returns a new instance of Struct.
	pub fn Fn(Option { Separator, .. }: Option) -> Self {
		Self {
			Exclude: Command()
diff --git a/Source/Struct/Summary/Difference.rs b/Source/Struct/Summary/Difference.rs
index 7583a0a..bb7ec8e 100644
--- a/Source/Struct/Summary/Difference.rs
+++ b/Source/Struct/Summary/Difference.rs
@@ -1,3 +1,6 @@
//! This module defines structures related to git diff summary options.

/// Represents the options for generating a git diff summary.
pub struct Struct {
	pub Omit: Vec<String>,
}

