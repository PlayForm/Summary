diff --git a/Cargo.toml b/Cargo.toml
index 745ad03..c769c35 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -34,5 +34,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.1"
version = "0.0.2"
edition = "2021"
diff --git a/CHANGELOG.md b/CHANGELOG.md
index 43d7b3d..68e22b1 100644
--- a/CHANGELOG.md
+++ b/CHANGELOG.md
@@ -1,3 +1,7 @@
## 0.0.2

-   Version 2.0

## 0.0.1

-   Initial version

diff --git a/build.rs b/build.rs
index 73ccc94..1f0de60 100644
--- a/build.rs
+++ b/build.rs
@@ -1,8 +1,5 @@
#![allow(non_snake_case)]

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Toml {
	package: Package,
@@ -24,3 +21,6 @@ fn main() {
		.version
	);
}

use serde::Deserialize;
use std::fs;
diff --git a/Cargo.toml b/Cargo.toml
index 745ad03..c10016a 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -14,6 +14,7 @@ rayon = "1.10.0"
tokio = { version = "1.39.2", features = ["full"] }
git2 = { version = "0.19.0" }
num_cpus = "1.16.0"
regex = "1.10.5"

[build-dependencies]
serde = { version = "1.0.204", features = ["derive"] }
@@ -34,5 +35,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.1"
version = "0.0.3"
edition = "2021"
diff --git a/CHANGELOG.md b/CHANGELOG.md
index 43d7b3d..2821f26 100644
--- a/CHANGELOG.md
+++ b/CHANGELOG.md
@@ -1,3 +1,11 @@
## 0.0.3

-   Cleanup

## 0.0.2

-   Version 2.0

## 0.0.1

-   Initial version
diff --git a/README.md b/README.md
index fd4bfe5..018729b 100644
--- a/README.md
+++ b/README.md
@@ -1,11 +1,33 @@
# 🗣️ [Summary] —

`Summary` is a command-line tool that executes commands in multiple directories
simultaneously. It leverages parallel processing and concurrent `I/O` to
efficiently run tasks across directories.
`Summary` is a powerful command-line tool designed for efficient file processing
and summarization. It offers both sequential and parallel processing
capabilities, along with flexible file filtering options.

[Summary]: HTTPS://crates.io/crates/psummary

## Feature

-   Directory traversal and file filtering
-   Parallel and sequential processing modes
-   Customizable file pattern matching
-   Exclusion of specified files or directories
-   Integration with Pieces OS for enhanced functionality

## Pieces OS Integration

The `Summary` CLI supports Pieces OS, essentially acting as a plugin that can
rewrite the whole system. This integration allows for:

-   Enhanced code analysis and summarization.
-   Improved context-aware processing.
-   Seamless integration with other Pieces OS-compatible tools.
-   Potential for AI-driven insights and optimizations.

By leveraging Pieces OS, `Summary` can tap into a broader ecosystem of
development tools and services, significantly expanding its capabilities beyond
basic file processing.

## Installation

```sh
@@ -14,6 +36,18 @@ cargo install psummary

## Usage

The `Summary` tool can be used with various options:

-   `--Root` or `-R`: Set the current working directory
-   `--Parallel` or `-P`: Run commands in parallel
-   `--Exclude`: Exclude certain files or directories
-   `--Pattern`: Specify a custom pattern for matching
-   `--Separator`: Define a custom separator

For Pieces OS integration, refer to the Pieces OS documentation for specific
flags and configuration options.
[Pieces](HTTPS://GitHub.Com/PlayForm/Pieces.git)

```sh
Summary
```
@@ -60,10 +94,13 @@ Define a custom separator

`Summary` relies on several Rust crates to provide its functionality:

-   `clap` - Parses command-line arguments
-   `rayon` - Enables parallel processing
-   `tokio` - Provides an asynchronous runtime
-   `walkdir` - Facilitates efficient filesystem traversal
-   `clap` - For parsing command-line arguments.
-   `rayon` - For parallel processing.
-   `tokio` - For asynchronous runtime.
-   `walkdir` - For efficient filesystem traversal.

[Pieces OS](HTTPS://Pieces.App): For extended functionality and system
integration.

[Summary]: HTTPS://crates.io/crates/psummary

diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index c590122..4cfbb9f 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -16,6 +16,27 @@ pub fn Fn() -> ArgMatches {
		.version(env!("CARGO_PKG_VERSION"))
		.author("🖋️ Source — 👐🏻 Open — <Source/Open@PlayForm.Cloud>")
		.about("🗣️ Summary —")
		.arg(
			Arg::new("Exclude")
				.short('E')
				.long("Exclude")
				.display_order(4)
				.value_name("EXCLUDE")
				.required(false)
				.help("🚫 Exclude —")
				.default_value("node_modules"),
		)
		.arg(
			Arg::new("Omit")
				.short('O')
				.long("Omit")
				.display_order(6)
				.value_name("OMIT")
				.required(false)
				.help("🚫 Omit —")
				.action(clap::ArgAction::Append)
				.default_value("Documentation"),
		)
		.arg(
			Arg::new("Parallel")
				.short('P')
@@ -26,6 +47,15 @@ pub fn Fn() -> ArgMatches {
				.required(false)
				.help("⏩ Parallel —"),
		)
		.arg(
			Arg::new("Pattern")
				.long("Pattern")
				.display_order(5)
				.value_name("PATTERN")
				.required(false)
				.help("🔍 Pattern —")
				.default_value(".git"),
		)
		.arg(
			Arg::new("Root")
				.short('R')
@@ -36,24 +66,6 @@ pub fn Fn() -> ArgMatches {
				.help("📂 Root —")
				.default_value("."),
		)
		.arg(
			Arg::new("Exclude")
				.short('E')
				.long("Exclude")
				.display_order(4)
				.value_name("EXCLUDE")
				.required(false)
				.help("🚫 Exclude —")
				.default_value("node_modules"),
		)
		.arg(
			Arg::new("Pattern")
				.display_order(5)
				.value_name("PATTERN")
				.required(false)
				.help("🔍 Pattern —")
				.default_value(".git"),
		)
		.get_matches()
}

diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 37a5949..463f890 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -17,8 +17,8 @@
/// };
/// Fn(option).await;
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
	let Queue: Vec<_> = stream::iter(
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
@@ -29,20 +29,21 @@ pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
			})
			.collect::<Vec<String>>(),
	)
	.map(|Entry| async move {
		match crate::Fn::Summary::Fn(&Entry).await {
			Ok(summary) => Ok(summary),
			Err(e) => Err(format!("Error generating summary for {}: {}", Entry, e)),
	.map(|Entry| {
		let Omit = Omit.clone();

		async move {
			match crate::Fn::Summary::Fn(&Entry, &crate::Fn::Summary::Difference::Option { Omit })
				.await
			{
				Ok(Summary) => Ok(Summary),
				Err(_Error) => Err(format!("Error generating summary for {}: {}", Entry, _Error)),
			}
		}
	})
	.buffer_unordered(num_cpus::get())
	.collect()
	.collect::<Vec<_>>()
	.await;

	Queue.par_iter().for_each(|Output| match Output {
		Ok(Summary) => println!("Summary: {:?}", Summary),
		Err(Error) => eprintln!("Error: {}", Error),
	});
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index 63923f7..c19801b 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -17,7 +17,8 @@
/// };
/// Fn(option);
/// ```
pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
	futures::future::join_all(
		Entry
			.into_iter()
			.filter_map(|Entry| {
@@ -26,9 +27,25 @@ pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
		.for_each(|_Entry| {
			// TODO: GENERATE SUMMARY
			.map(|Entry| {
				let Omit = Omit.clone();

				async move {
					match crate::Fn::Summary::Fn(
						&Entry,
						&crate::Fn::Summary::Difference::Option { Omit },
					)
					.await
					{
						Ok(Summary) => Ok(Summary),
						Err(_Error) => {
							Err(format!("Error generating summary for {}: {}", Entry, _Error))
						}
					}
				}
			})
			.collect::<Vec<_>>(),
	).await;
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index eb2d2ce..f6825a8 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -16,29 +16,28 @@
/// };
/// let summary = Fn(&option);
/// ```
pub async fn Fn(Entry: &str) -> Result<(), Box<dyn std::error::Error>> {
	let Repository = Repository::open(Entry)?;

pub async fn Fn(
	Entry: &str,
	Option: &crate::Fn::Summary::Difference::Option,
) -> Result<(), Box<dyn std::error::Error>> {
	match Repository::open(Entry) {
		Ok(Repository) => {
			let Tag = Repository.tag_names(None)?;
	let mut Start = None;

	let Summary = "Summary";
	fs::create_dir_all(Summary)?;

	for i in 0..Tag.len() {
		if let Some(Tag) = Tag.get(i) {
			if let Some(Start) = Start {
				let Difference = crate::Fn::Summary::Difference::Fn(&Repository, Start, Tag)?;

				File::create(&format!("{}/Difference_{}_{}.txt", Summary, Start, Tag))?.write_all(
					crate::Fn::Summary::Difference::Fn(&Repository, Start, Tag)?.as_bytes(),
				)?;
			let Tags: Vec<_> = Tag.iter().filter_map(|Tag| Tag).collect();

				File::create(&format!("{}/Release_{}_{}.txt", Summary, Start, Tag))?
					.write_all(crate::Fn::Summary::Release::Fn(&Difference).as_bytes())?;
			for (Index, &Current) in Tags.iter().enumerate() {
				for (_, &Next) in Tags.iter().enumerate().skip(Index + 1) {
					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Current, Next, Option)?
					);
				}

			Start = Some(Tag);
			}
		}
		Err(_Error) => {
			println!("Failed to open repository: {}", _Error);
			return Err(_Error.into());
		}
	}

@@ -46,10 +45,5 @@ pub async fn Fn(Entry: &str) -> Result<(), Box<dyn std::error::Error>> {
}

use git2::Repository;
use std::{
	fs::{self, File},
	io::Write,
};

pub mod Difference;
pub mod Release;
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
index 8d7badf..4951c08 100644
--- a/Source/Fn/Summary/Difference.rs
+++ b/Source/Fn/Summary/Difference.rs
@@ -16,18 +16,61 @@
/// };
/// let difference = Fn(&option);
/// ```
pub fn Fn(Repo: &git2::Repository, Start: &str, End: &str) -> Result<String, git2::Error> {
pub fn Fn(
	Repository: &git2::Repository,
	Start: &str,
	End: &str,
	Option: &Option,
) -> Result<String, git2::Error> {
	let mut Difference = String::new();
	let mut Options = git2::DiffOptions::new();

	Repo.diff_tree_to_tree(
		Some(&Repo.revparse_single(Start)?.peel_to_commit()?.tree()?),
		Some(&Repo.revparse_single(End)?.peel_to_commit()?.tree()?),
		Some(&mut git2::DiffOptions::new()),
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

	Repository
		.diff_tree_to_tree(
			Some(&Repository.revparse_single(Start)?.peel_to_commit()?.tree()?),
			Some(&Repository.revparse_single(End)?.peel_to_commit()?.tree()?),
			Some(&mut Options),
		)?
	.print(git2::DiffFormat::Patch, |_, _, line| {
		Difference.push_str(std::str::from_utf8(line.content()).unwrap());
		.print(git2::DiffFormat::Patch, |Delta, _, Line| {
			if !Option
				.Omit
				.iter()
				.map(|Omit: &String| regex::Regex::new(Omit).expect("Cannot Regex."))
				.collect::<Vec<_>>()
				.iter()
				.any(|Omit| {
					Omit.is_match(&Delta.old_file().path().unwrap().display().to_string())
						|| Omit.is_match(&Delta.new_file().path().unwrap().display().to_string())
				}) {
				Difference.push_str(std::str::from_utf8(Line.content()).unwrap());
			};

			true
		})?;

	Ok(Difference)
}

pub struct Option {
	pub Omit: Vec<String>,
}
diff --git a/Source/Fn/Summary/Release.rs b/Source/Fn/Summary/Release.rs
deleted file mode 100644
index 611f1eb..0000000
--- a/Source/Fn/Summary/Release.rs
+++ /dev/null
@@ -1,33 +0,0 @@
/// Generates a release summary.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for generating the release summary.
///
/// # Returns
///
/// * `Return` - The generated release summary.
///
/// # Example
///
/// ```
/// let option = Option {
///     // Fields needed for release summary generation
/// };
/// let release_summary = Fn(&option);
/// ```
pub fn Fn(Difference: &str) -> String {
	let mut Release = String::new();

	Release.push_str("Release Notes:\n");

	for Difference in Difference.lines() {
		if Difference.starts_with("+") && !Difference.starts_with("+++") {
			Release.push_str(&format!("Added: {}\n", &Difference[1..]));
		} else if Difference.starts_with("-") && !Difference.starts_with("---") {
			Release.push_str(&format!("Removed: {}\n", &Difference[1..]));
		}
	}

	Release
}
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
index da03f34..4a356ae 100644
--- a/Source/Struct/Binary/Command.rs
+++ b/Source/Struct/Binary/Command.rs
@@ -19,7 +19,7 @@ impl Struct {
							Parallel::Fn(Option).await;
						}
						false => {
							Sequential::Fn(Option);
							Sequential::Fn(Option).await;
						}
					};
				})
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
index c0c6b89..45d3e41 100644
--- a/Source/Struct/Binary/Command/Entry.rs
+++ b/Source/Struct/Binary/Command/Entry.rs
@@ -11,12 +11,16 @@ pub struct Struct {

	/// The path separator character.
	pub Separator: Separator,

	/// List of items to omit from processing.
	pub Omit: Omit,
}

impl Struct {
	pub fn Fn(Option: &Option) -> Self {
		Self {
			Entry: crate::Fn::Binary::Command::Entry::Fn(Option),
			Omit: Option.Omit.clone(),
			Parallel: Option.Parallel,
			Pattern: Option.Pattern.clone(),
			Separator: Option.Separator,
@@ -24,7 +28,9 @@ impl Struct {
	}
}

use crate::Struct::Binary::Command::Option::{Parallel, Pattern, Separator, Struct as Option};
use crate::Struct::Binary::Command::Option::{
	Omit, Parallel, Pattern, Separator, Struct as Option,
};

/// Defines a type alias for a vector of vectors of strings.
pub type Type = Vec<Vec<String>>;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index 8453d33..8d32f31 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
@@ -3,6 +3,9 @@ pub struct Struct {
	/// List of patterns to exclude from processing.
	pub Exclude: Vec<String>,

	/// List of items to omit from processing.
	pub Omit: Vec<String>,

	/// Flag indicating whether to use parallel processing.
	pub Parallel: Parallel,

@@ -20,23 +23,29 @@ impl Struct {
	/// Creates a new Struct instance from the provided Option.
	pub fn Fn(Option { Separator, .. }: Option) -> Self {
		Self {
			Exclude: Fn()
			Exclude: Command()
				.get_one::<String>("Exclude")
				.expect("Cannot Exclude.")
				.split(" ")
				.map(|Command| Command.to_string())
				.map(|Exclude| Exclude.to_string())
				.collect::<Vec<_>>(),
			Parallel: Fn().get_flag("Parallel"),
			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Fn().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Parallel: Command().get_flag("Parallel"),
			Pattern: Command().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Command().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Separator,
			Omit: Command()
				.get_many::<String>("Omit")
				.expect("Cannot Omit.")
				.map(|Omit| Omit.to_string())
				.collect(),
		}
	}
}

use crate::{Fn::Binary::Command::Fn, Struct::Binary::Command::Struct as Option};
use crate::{Fn::Binary::Command::Fn as Command, Struct::Binary::Command::Struct as Option};

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;
pub type Omit = Vec<String>;

diff --git a/build.rs b/build.rs
index 73ccc94..1f0de60 100644
--- a/build.rs
+++ b/build.rs
@@ -1,8 +1,5 @@
#![allow(non_snake_case)]

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Toml {
	package: Package,
@@ -24,3 +21,6 @@ fn main() {
		.version
	);
}

use serde::Deserialize;
use std::fs;
diff --git a/Cargo.toml b/Cargo.toml
index 745ad03..3d3159a 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -14,6 +14,7 @@ rayon = "1.10.0"
tokio = { version = "1.39.2", features = ["full"] }
git2 = { version = "0.19.0" }
num_cpus = "1.16.0"
regex = "1.10.5"

[build-dependencies]
serde = { version = "1.0.204", features = ["derive"] }
@@ -34,5 +35,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.1"
version = "0.0.4"
edition = "2021"
diff --git a/CHANGELOG.md b/CHANGELOG.md
index 43d7b3d..0060a33 100644
--- a/CHANGELOG.md
+++ b/CHANGELOG.md
@@ -1,3 +1,15 @@
## 0.0.4

-   Cleanup

## 0.0.3

-   Cleanup

## 0.0.2

-   Version 2.0

## 0.0.1

-   Initial version
diff --git a/README.md b/README.md
index fd4bfe5..49bf9ea 100644
--- a/README.md
+++ b/README.md
@@ -1,11 +1,33 @@
# 🗣️ [Summary] —

`Summary` is a command-line tool that executes commands in multiple directories
simultaneously. It leverages parallel processing and concurrent `I/O` to
efficiently run tasks across directories.
`Summary` is a powerful command-line tool designed for efficient Git repository
analysis and summarization. It offers both sequential and parallel processing
capabilities, along with flexible file filtering options.

[Summary]: HTTPS://crates.io/crates/psummary

## Features

-   Customizable file pattern matching
-   Diff generation between `Git` tags
-   Directory traversal and file filtering
-   Exclusion of specified files or directories
-   `Git` repository analysis
-   Integration with [Pieces OS] for enhanced functionality
-   Parallel and sequential processing modes

## [Pieces OS] Integration

The `Summary` CLI supports [Pieces OS], allowing it to:

-   Generate comprehensive diff logs and release notes automatically.
-   Provide AI-driven code analysis and insights.
-   Offer improved context-aware processing of repository changes.
-   Seamlessly interact with other [Pieces OS]-compatible development tools.

By leveraging [Pieces OS], `Summary` can tap into a broader ecosystem of development
tools and services, significantly expanding its capabilities beyond basic file processing.

## Installation

```sh
@@ -14,58 +36,97 @@ cargo install psummary

## Usage

```sh
Summary
```

This command will fetch from upstream for all `.git` repositories inside the
current directory. It essentially replaces the following command:
The Summary tool can be used with various options:

```sh
find -iname .git -type d -execdir git fetch upstream \;
```
🗣️ Summary —

Usage: Summary [OPTIONS]

Options:
  -P, --Parallel           ⏩ Parallel —
  -R, --Root <ROOT>        📂 Root — [default: .]
  -E, --Exclude <EXCLUDE>  🚫 Exclude — [default: node_modules]
      --Pattern <PATTERN>  🔍 Pattern — [default: .git]
  -O, --Omit <OMIT>        🚫 Omit — [default: Documentation]
  -h, --help               Print help
  -V, --version            Print version
```

This command will generate summaries for all the Git tags inside the specified
repository.

## Options

The `Summary` tool can be used with various options:

#### --Exclude or -E:

Exclude certain files or directories (defailt is `node_modules`).

#### --Omit or -O:

Specify regex patterns to omit files from processing (default is
"Documentation").

#### --Parallel or -P:

Run processing in parallel (default is `sequential`):

#### --Pattern:

Specify a custom pattern for matching (defailt is `.git`).

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):

```sh
Summary -R D:\Developer .git git fetch upstream
```
For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
flags and configuration options. [Pieces OS]

#### --Parallel or -P:
## Examples

Summary commands in `parallel` (default is `sequential`):
Analyze the current directory:

```sh
Summary -P -R D:\Developer .git git fetch upstream
Summary
```

#### --Exclude:
Analyze a specific directory in parallel:

Exclude certain files or directories (defailt is
`node_modules target dist vendor`)
```sh
Summary -P -R D:\Developer
```

#### --Pattern:
Exclude additional directories:

Specify a custom pattern for matching (defailt is `.git`)
```sh
Summary -E "node_modules target dist vendor"
```

#### --Separator:
Omit specific file patterns:

Define a custom separator
```sh
Summary -O "\.md$" -O "\.txt$"
```

## Dependencies

`Summary` relies on several Rust crates to provide its functionality:

-   `clap` - Parses command-line arguments
-   `rayon` - Enables parallel processing
-   `tokio` - Provides an asynchronous runtime
-   `walkdir` - Facilitates efficient filesystem traversal
-   `clap` - For parsing command-line arguments.
-   `futures` - For asynchronous programming abstractions.
-   `git2` - For Git repository operations.
-   `num_cpus` - For determining the number of CPUs for parallel processing.
-   `rayon` - For parallel processing.
-   `regex` - For pattern matching and text manipulation.
-   `tokio` - For asynchronous runtime.
-   `walkdir` - For efficient filesystem traversal.

[Pieces OS] For extended functionality and system integration.

[Summary]: HTTPS://crates.io/crates/psummary
[Pieces OS]: HTTPS://Pieces.App

## Changelog

diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index c590122..4cfbb9f 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -16,6 +16,27 @@ pub fn Fn() -> ArgMatches {
		.version(env!("CARGO_PKG_VERSION"))
		.author("🖋️ Source — 👐🏻 Open — <Source/Open@PlayForm.Cloud>")
		.about("🗣️ Summary —")
		.arg(
			Arg::new("Exclude")
				.short('E')
				.long("Exclude")
				.display_order(4)
				.value_name("EXCLUDE")
				.required(false)
				.help("🚫 Exclude —")
				.default_value("node_modules"),
		)
		.arg(
			Arg::new("Omit")
				.short('O')
				.long("Omit")
				.display_order(6)
				.value_name("OMIT")
				.required(false)
				.help("🚫 Omit —")
				.action(clap::ArgAction::Append)
				.default_value("Documentation"),
		)
		.arg(
			Arg::new("Parallel")
				.short('P')
@@ -26,6 +47,15 @@ pub fn Fn() -> ArgMatches {
				.required(false)
				.help("⏩ Parallel —"),
		)
		.arg(
			Arg::new("Pattern")
				.long("Pattern")
				.display_order(5)
				.value_name("PATTERN")
				.required(false)
				.help("🔍 Pattern —")
				.default_value(".git"),
		)
		.arg(
			Arg::new("Root")
				.short('R')
@@ -36,24 +66,6 @@ pub fn Fn() -> ArgMatches {
				.help("📂 Root —")
				.default_value("."),
		)
		.arg(
			Arg::new("Exclude")
				.short('E')
				.long("Exclude")
				.display_order(4)
				.value_name("EXCLUDE")
				.required(false)
				.help("🚫 Exclude —")
				.default_value("node_modules"),
		)
		.arg(
			Arg::new("Pattern")
				.display_order(5)
				.value_name("PATTERN")
				.required(false)
				.help("🔍 Pattern —")
				.default_value(".git"),
		)
		.get_matches()
}

diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 37a5949..463f890 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -17,8 +17,8 @@
/// };
/// Fn(option).await;
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
	let Queue: Vec<_> = stream::iter(
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
@@ -29,20 +29,21 @@ pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
			})
			.collect::<Vec<String>>(),
	)
	.map(|Entry| async move {
		match crate::Fn::Summary::Fn(&Entry).await {
			Ok(summary) => Ok(summary),
			Err(e) => Err(format!("Error generating summary for {}: {}", Entry, e)),
	.map(|Entry| {
		let Omit = Omit.clone();

		async move {
			match crate::Fn::Summary::Fn(&Entry, &crate::Fn::Summary::Difference::Option { Omit })
				.await
			{
				Ok(Summary) => Ok(Summary),
				Err(_Error) => Err(format!("Error generating summary for {}: {}", Entry, _Error)),
			}
		}
	})
	.buffer_unordered(num_cpus::get())
	.collect()
	.collect::<Vec<_>>()
	.await;

	Queue.par_iter().for_each(|Output| match Output {
		Ok(Summary) => println!("Summary: {:?}", Summary),
		Err(Error) => eprintln!("Error: {}", Error),
	});
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index 63923f7..c19801b 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -17,7 +17,8 @@
/// };
/// Fn(option);
/// ```
pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
	futures::future::join_all(
		Entry
			.into_iter()
			.filter_map(|Entry| {
@@ -26,9 +27,25 @@ pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
		.for_each(|_Entry| {
			// TODO: GENERATE SUMMARY
			.map(|Entry| {
				let Omit = Omit.clone();

				async move {
					match crate::Fn::Summary::Fn(
						&Entry,
						&crate::Fn::Summary::Difference::Option { Omit },
					)
					.await
					{
						Ok(Summary) => Ok(Summary),
						Err(_Error) => {
							Err(format!("Error generating summary for {}: {}", Entry, _Error))
						}
					}
				}
			})
			.collect::<Vec<_>>(),
	).await;
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index eb2d2ce..f6825a8 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -16,29 +16,28 @@
/// };
/// let summary = Fn(&option);
/// ```
pub async fn Fn(Entry: &str) -> Result<(), Box<dyn std::error::Error>> {
	let Repository = Repository::open(Entry)?;

pub async fn Fn(
	Entry: &str,
	Option: &crate::Fn::Summary::Difference::Option,
) -> Result<(), Box<dyn std::error::Error>> {
	match Repository::open(Entry) {
		Ok(Repository) => {
			let Tag = Repository.tag_names(None)?;
	let mut Start = None;

	let Summary = "Summary";
	fs::create_dir_all(Summary)?;

	for i in 0..Tag.len() {
		if let Some(Tag) = Tag.get(i) {
			if let Some(Start) = Start {
				let Difference = crate::Fn::Summary::Difference::Fn(&Repository, Start, Tag)?;

				File::create(&format!("{}/Difference_{}_{}.txt", Summary, Start, Tag))?.write_all(
					crate::Fn::Summary::Difference::Fn(&Repository, Start, Tag)?.as_bytes(),
				)?;
			let Tags: Vec<_> = Tag.iter().filter_map(|Tag| Tag).collect();

				File::create(&format!("{}/Release_{}_{}.txt", Summary, Start, Tag))?
					.write_all(crate::Fn::Summary::Release::Fn(&Difference).as_bytes())?;
			for (Index, &Current) in Tags.iter().enumerate() {
				for (_, &Next) in Tags.iter().enumerate().skip(Index + 1) {
					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Current, Next, Option)?
					);
				}

			Start = Some(Tag);
			}
		}
		Err(_Error) => {
			println!("Failed to open repository: {}", _Error);
			return Err(_Error.into());
		}
	}

@@ -46,10 +45,5 @@ pub async fn Fn(Entry: &str) -> Result<(), Box<dyn std::error::Error>> {
}

use git2::Repository;
use std::{
	fs::{self, File},
	io::Write,
};

pub mod Difference;
pub mod Release;
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
index 8d7badf..d96794d 100644
--- a/Source/Fn/Summary/Difference.rs
+++ b/Source/Fn/Summary/Difference.rs
@@ -16,18 +16,51 @@
/// };
/// let difference = Fn(&option);
/// ```
pub fn Fn(Repo: &git2::Repository, Start: &str, End: &str) -> Result<String, git2::Error> {
pub fn Fn(
	Repository: &git2::Repository,
	Start: &str,
	End: &str,
	Option: &Option,
) -> Result<String, git2::Error> {
	let mut Difference = String::new();
	let mut Options = git2::DiffOptions::new();

	Repo.diff_tree_to_tree(
		Some(&Repo.revparse_single(Start)?.peel_to_commit()?.tree()?),
		Some(&Repo.revparse_single(End)?.peel_to_commit()?.tree()?),
		Some(&mut git2::DiffOptions::new()),
	Options.indent_heuristic(true);
	Options.minimal(true);
	Options.force_text(true);
	Options.ignore_blank_lines(true);
	Options.ignore_case(true);
	Options.ignore_filemode(true);
	Options.ignore_whitespace(true);
	Options.ignore_whitespace_change(true);
	Options.ignore_whitespace_eol(true);

	Repository
		.diff_tree_to_tree(
			Some(&Repository.revparse_single(Start)?.peel_to_commit()?.tree()?),
			Some(&Repository.revparse_single(End)?.peel_to_commit()?.tree()?),
			Some(&mut Options),
		)?
	.print(git2::DiffFormat::Patch, |_, _, line| {
		Difference.push_str(std::str::from_utf8(line.content()).unwrap());
		.print(git2::DiffFormat::Patch, |Delta, _, Line| {
			if !Option
				.Omit
				.iter()
				.map(|Omit: &String| regex::Regex::new(Omit).expect("Cannot Regex."))
				.collect::<Vec<_>>()
				.iter()
				.any(|Omit| {
					Omit.is_match(&Delta.old_file().path().unwrap().display().to_string())
						|| Omit.is_match(&Delta.new_file().path().unwrap().display().to_string())
				}) {
				Difference.push_str(std::str::from_utf8(Line.content()).unwrap());
			};

			true
		})?;

	Ok(Difference)
}

pub struct Option {
	pub Omit: Vec<String>,
}
diff --git a/Source/Fn/Summary/Release.rs b/Source/Fn/Summary/Release.rs
deleted file mode 100644
index 611f1eb..0000000
--- a/Source/Fn/Summary/Release.rs
+++ /dev/null
@@ -1,33 +0,0 @@
/// Generates a release summary.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for generating the release summary.
///
/// # Returns
///
/// * `Return` - The generated release summary.
///
/// # Example
///
/// ```
/// let option = Option {
///     // Fields needed for release summary generation
/// };
/// let release_summary = Fn(&option);
/// ```
pub fn Fn(Difference: &str) -> String {
	let mut Release = String::new();

	Release.push_str("Release Notes:\n");

	for Difference in Difference.lines() {
		if Difference.starts_with("+") && !Difference.starts_with("+++") {
			Release.push_str(&format!("Added: {}\n", &Difference[1..]));
		} else if Difference.starts_with("-") && !Difference.starts_with("---") {
			Release.push_str(&format!("Removed: {}\n", &Difference[1..]));
		}
	}

	Release
}
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
index da03f34..4a356ae 100644
--- a/Source/Struct/Binary/Command.rs
+++ b/Source/Struct/Binary/Command.rs
@@ -19,7 +19,7 @@ impl Struct {
							Parallel::Fn(Option).await;
						}
						false => {
							Sequential::Fn(Option);
							Sequential::Fn(Option).await;
						}
					};
				})
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
index c0c6b89..45d3e41 100644
--- a/Source/Struct/Binary/Command/Entry.rs
+++ b/Source/Struct/Binary/Command/Entry.rs
@@ -11,12 +11,16 @@ pub struct Struct {

	/// The path separator character.
	pub Separator: Separator,

	/// List of items to omit from processing.
	pub Omit: Omit,
}

impl Struct {
	pub fn Fn(Option: &Option) -> Self {
		Self {
			Entry: crate::Fn::Binary::Command::Entry::Fn(Option),
			Omit: Option.Omit.clone(),
			Parallel: Option.Parallel,
			Pattern: Option.Pattern.clone(),
			Separator: Option.Separator,
@@ -24,7 +28,9 @@ impl Struct {
	}
}

use crate::Struct::Binary::Command::Option::{Parallel, Pattern, Separator, Struct as Option};
use crate::Struct::Binary::Command::Option::{
	Omit, Parallel, Pattern, Separator, Struct as Option,
};

/// Defines a type alias for a vector of vectors of strings.
pub type Type = Vec<Vec<String>>;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index 8453d33..8d32f31 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
@@ -3,6 +3,9 @@ pub struct Struct {
	/// List of patterns to exclude from processing.
	pub Exclude: Vec<String>,

	/// List of items to omit from processing.
	pub Omit: Vec<String>,

	/// Flag indicating whether to use parallel processing.
	pub Parallel: Parallel,

@@ -20,23 +23,29 @@ impl Struct {
	/// Creates a new Struct instance from the provided Option.
	pub fn Fn(Option { Separator, .. }: Option) -> Self {
		Self {
			Exclude: Fn()
			Exclude: Command()
				.get_one::<String>("Exclude")
				.expect("Cannot Exclude.")
				.split(" ")
				.map(|Command| Command.to_string())
				.map(|Exclude| Exclude.to_string())
				.collect::<Vec<_>>(),
			Parallel: Fn().get_flag("Parallel"),
			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Fn().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Parallel: Command().get_flag("Parallel"),
			Pattern: Command().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Command().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Separator,
			Omit: Command()
				.get_many::<String>("Omit")
				.expect("Cannot Omit.")
				.map(|Omit| Omit.to_string())
				.collect(),
		}
	}
}

use crate::{Fn::Binary::Command::Fn, Struct::Binary::Command::Struct as Option};
use crate::{Fn::Binary::Command::Fn as Command, Struct::Binary::Command::Struct as Option};

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;
pub type Omit = Vec<String>;

diff --git a/build.rs b/build.rs
index 73ccc94..1f0de60 100644
--- a/build.rs
+++ b/build.rs
@@ -1,8 +1,5 @@
#![allow(non_snake_case)]

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Toml {
	package: Package,
@@ -24,3 +21,6 @@ fn main() {
		.version
	);
}

use serde::Deserialize;
use std::fs;
diff --git a/Cargo.toml b/Cargo.toml
index 745ad03..3615257 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -14,6 +14,7 @@ rayon = "1.10.0"
tokio = { version = "1.39.2", features = ["full"] }
git2 = { version = "0.19.0" }
num_cpus = "1.16.0"
regex = "1.10.5"

[build-dependencies]
serde = { version = "1.0.204", features = ["derive"] }
@@ -34,5 +35,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.1"
version = "0.0.5"
edition = "2021"
diff --git a/CHANGELOG.md b/CHANGELOG.md
index 43d7b3d..56f65a8 100644
--- a/CHANGELOG.md
+++ b/CHANGELOG.md
@@ -1,3 +1,19 @@
## 0.0.5

-   Cleanup

## 0.0.4

-   Cleanup

## 0.0.3

-   Cleanup

## 0.0.2

-   Version 2.0

## 0.0.1

-   Initial version
diff --git a/README.md b/README.md
index fd4bfe5..4449a1b 100644
--- a/README.md
+++ b/README.md
@@ -1,11 +1,33 @@
# 🗣️ [Summary] —

`Summary` is a command-line tool that executes commands in multiple directories
simultaneously. It leverages parallel processing and concurrent `I/O` to
efficiently run tasks across directories.
`Summary` is a powerful command-line tool designed for efficient `Git`
repository analysis and summarization. It offers both sequential and parallel
processing capabilities, along with flexible file filtering options.

[Summary]: HTTPS://crates.io/crates/psummary

## Features

-   Customizable file pattern matching.
-   Diff generation between `Git` tags.
-   Directory traversal and file filtering.
-   Exclusion of specified files or directories.
-   `Git` repository analysis.
-   Integration with [Pieces OS] for enhanced functionality.
-   Parallel and sequential processing modes.

## [Pieces OS] Integration

The `Summary` CLI supports [Pieces OS], allowing it to:

-   Generate comprehensive diff logs and release notes automatically.
-   Provide AI-driven code analysis and insights.
-   Offer improved context-aware processing of repository changes.
-   Seamlessly interact with other [Pieces OS]-compatible development tools.

By leveraging [Pieces OS], `Summary` can tap into a broader ecosystem of development
tools and services, significantly expanding its capabilities beyond basic file processing.

## Installation

```sh
@@ -14,58 +36,97 @@ cargo install psummary

## Usage

```sh
Summary
```

This command will fetch from upstream for all `.git` repositories inside the
current directory. It essentially replaces the following command:
The Summary tool can be used with various options:

```sh
find -iname .git -type d -execdir git fetch upstream \;
```
🗣️ Summary —

Usage: Summary [OPTIONS]

Options:
  -P, --Parallel           ⏩ Parallel —
  -R, --Root <ROOT>        📂 Root — [default: .]
  -E, --Exclude <EXCLUDE>  🚫 Exclude — [default: node_modules]
      --Pattern <PATTERN>  🔍 Pattern — [default: .git]
  -O, --Omit <OMIT>        🚫 Omit — [default: Documentation]
  -h, --help               Print help
  -V, --version            Print version
```

This command will generate summaries for all the `Git` tags inside the specified
repository.

## Options

The `Summary` tool can be used with various options:

#### --Exclude or -E:

Exclude certain files or directories (defailt is `node_modules`).

#### --Omit or -O:

Specify regex patterns to omit files from processing (default is
"Documentation").

#### --Parallel or -P:

Run processing in parallel (default is `sequential`):

#### --Pattern:

Specify a custom pattern for matching (defailt is `.git`).

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):

```sh
Summary -R D:\Developer .git git fetch upstream
```
For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
flags and configuration options. [Pieces OS]

#### --Parallel or -P:
## Examples

Summary commands in `parallel` (default is `sequential`):
Analyze the current directory:

```sh
Summary -P -R D:\Developer .git git fetch upstream
Summary
```

#### --Exclude:
Analyze a specific directory in parallel:

Exclude certain files or directories (defailt is
`node_modules target dist vendor`)
```sh
Summary -P -R D:\Developer
```

#### --Pattern:
Exclude additional directories:

Specify a custom pattern for matching (defailt is `.git`)
```sh
Summary -E "node_modules target dist vendor"
```

#### --Separator:
Omit specific file patterns:

Define a custom separator
```sh
Summary -O "\.md$" -O "\.txt$"
```

## Dependencies

`Summary` relies on several Rust crates to provide its functionality:

-   `clap` - Parses command-line arguments
-   `rayon` - Enables parallel processing
-   `tokio` - Provides an asynchronous runtime
-   `walkdir` - Facilitates efficient filesystem traversal
-   `clap` - For parsing command-line arguments.
-   `futures` - For asynchronous programming abstractions.
-   `git2` - For `Git` repository operations.
-   `num_cpus` - For determining the number of CPUs for parallel processing.
-   `rayon` - For parallel processing.
-   `regex` - For pattern matching and text manipulation.
-   `tokio` - For asynchronous runtime.
-   `walkdir` - For efficient filesystem traversal.

[Pieces OS] For extended functionality and system integration.

[Summary]: HTTPS://crates.io/crates/psummary
[Pieces OS]: HTTPS://Pieces.App

## Changelog

diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index c590122..4cfbb9f 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -16,6 +16,27 @@ pub fn Fn() -> ArgMatches {
		.version(env!("CARGO_PKG_VERSION"))
		.author("🖋️ Source — 👐🏻 Open — <Source/Open@PlayForm.Cloud>")
		.about("🗣️ Summary —")
		.arg(
			Arg::new("Exclude")
				.short('E')
				.long("Exclude")
				.display_order(4)
				.value_name("EXCLUDE")
				.required(false)
				.help("🚫 Exclude —")
				.default_value("node_modules"),
		)
		.arg(
			Arg::new("Omit")
				.short('O')
				.long("Omit")
				.display_order(6)
				.value_name("OMIT")
				.required(false)
				.help("🚫 Omit —")
				.action(clap::ArgAction::Append)
				.default_value("Documentation"),
		)
		.arg(
			Arg::new("Parallel")
				.short('P')
@@ -26,6 +47,15 @@ pub fn Fn() -> ArgMatches {
				.required(false)
				.help("⏩ Parallel —"),
		)
		.arg(
			Arg::new("Pattern")
				.long("Pattern")
				.display_order(5)
				.value_name("PATTERN")
				.required(false)
				.help("🔍 Pattern —")
				.default_value(".git"),
		)
		.arg(
			Arg::new("Root")
				.short('R')
@@ -36,24 +66,6 @@ pub fn Fn() -> ArgMatches {
				.help("📂 Root —")
				.default_value("."),
		)
		.arg(
			Arg::new("Exclude")
				.short('E')
				.long("Exclude")
				.display_order(4)
				.value_name("EXCLUDE")
				.required(false)
				.help("🚫 Exclude —")
				.default_value("node_modules"),
		)
		.arg(
			Arg::new("Pattern")
				.display_order(5)
				.value_name("PATTERN")
				.required(false)
				.help("🔍 Pattern —")
				.default_value(".git"),
		)
		.get_matches()
}

diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 37a5949..22654c0 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -17,8 +17,8 @@
/// };
/// Fn(option).await;
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
	let Queue: Vec<_> = stream::iter(
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	futures::stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
@@ -29,22 +29,27 @@ pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
			})
			.collect::<Vec<String>>(),
	)
	.map(|Entry| async move {
		match crate::Fn::Summary::Fn(&Entry).await {
			Ok(summary) => Ok(summary),
			Err(e) => Err(format!("Error generating summary for {}: {}", Entry, e)),
	.map(|Entry| {
		let Omit = Omit.clone();

		async move {
			match crate::Fn::Summary::Fn(
				&Entry,
				&crate::Struct::Summary::Difference::Struct { Omit },
			)
			.await
			{
				Ok(Summary) => Ok(Summary),
				Err(_Error) => Err(format!("Error generating summary for {}: {}", Entry, _Error)),
			}
		}
	})
	.buffer_unordered(num_cpus::get())
	.collect()
	.collect::<Vec<_>>()
	.await;

	Queue.par_iter().for_each(|Output| match Output {
		Ok(Summary) => println!("Summary: {:?}", Summary),
		Err(Error) => eprintln!("Error: {}", Error),
	});
}

use futures::stream::StreamExt;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::Struct::Binary::Command::Entry::Struct as Option;
use futures::stream::{self, StreamExt};
use rayon::prelude::*;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index 63923f7..7ffeb0f 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -17,7 +17,8 @@
/// };
/// Fn(option);
/// ```
pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
	futures::future::join_all(
		Entry
			.into_iter()
			.filter_map(|Entry| {
@@ -26,9 +27,26 @@ pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
		.for_each(|_Entry| {
			// TODO: GENERATE SUMMARY
			.map(|Entry| {
				let Omit = Omit.clone();

				async move {
					match crate::Fn::Summary::Fn(
						&Entry,
						&crate::Struct::Summary::Difference::Struct { Omit },
					)
					.await
					{
						Ok(Summary) => Ok(Summary),
						Err(_Error) => {
							Err(format!("Error generating summary for {}: {}", Entry, _Error))
						}
					}
				}
			})
			.collect::<Vec<_>>(),
	)
	.await;
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index eb2d2ce..ae3e650 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -16,29 +16,28 @@
/// };
/// let summary = Fn(&option);
/// ```
pub async fn Fn(Entry: &str) -> Result<(), Box<dyn std::error::Error>> {
	let Repository = Repository::open(Entry)?;

	let Tag = Repository.tag_names(None)?;
	let mut Start = None;

	let Summary = "Summary";
	fs::create_dir_all(Summary)?;

	for i in 0..Tag.len() {
		if let Some(Tag) = Tag.get(i) {
			if let Some(Start) = Start {
				let Difference = crate::Fn::Summary::Difference::Fn(&Repository, Start, Tag)?;

				File::create(&format!("{}/Difference_{}_{}.txt", Summary, Start, Tag))?.write_all(
					crate::Fn::Summary::Difference::Fn(&Repository, Start, Tag)?.as_bytes(),
				)?;

				File::create(&format!("{}/Release_{}_{}.txt", Summary, Start, Tag))?
					.write_all(crate::Fn::Summary::Release::Fn(&Difference).as_bytes())?;
pub async fn Fn(
	Entry: &str,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<(), Box<dyn std::error::Error>> {
	match Repository::open(Entry) {
		Ok(Repository) => {
			let Name = Repository.tag_names(None)?;

			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			for (Index, &Current) in Tag.iter().enumerate() {
				for (_, &Next) in Tag.iter().enumerate().skip(Index + 1) {
					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Current, Next, Option)?
					);
				}

			Start = Some(Tag);
			}
		}
		Err(_Error) => {
			println!("Failed to open repository: {}", _Error);
			return Err(_Error.into());
		}
	}

@@ -46,10 +45,5 @@ pub async fn Fn(Entry: &str) -> Result<(), Box<dyn std::error::Error>> {
}

use git2::Repository;
use std::{
	fs::{self, File},
	io::Write,
};

pub mod Difference;
pub mod Release;
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
index 8d7badf..cc25057 100644
--- a/Source/Fn/Summary/Difference.rs
+++ b/Source/Fn/Summary/Difference.rs
@@ -16,18 +16,107 @@
/// };
/// let difference = Fn(&option);
/// ```
pub fn Fn(Repo: &git2::Repository, Start: &str, End: &str) -> Result<String, git2::Error> {
pub fn Fn(
	Repository: &git2::Repository,
	Start: &str,
	End: &str,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<String, git2::Error> {
	let mut Omit = vec![
		r"\.pdf$",
		r"\.exe$",
		r"\.dll$",
		r"\.so$",
		r"\.dylib$",
		r"\.zip$",
		r"\.tar$",
		r"\.gz$",
		r"\.7z$",
		r"\.rar$",
		r"\.jpg$",
		r"\.jpeg$",
		r"\.png$",
		r"\.gif$",
		r"\.bmp$",
		r"\.tiff$",
		r"\.ico$",
		r"\.svg$",
		r"\.webp$",
		r"\.heic$",
		r"\.mp3$",
		r"\.wav$",
		r"\.ogg$",
		r"\.flac$",
		r"\.m4a$",
		r"\.mp4$",
		r"\.avi$",
		r"\.mov$",
		r"\.mkv$",
		r"\.wmv$",
		r"\.doc$",
		r"\.docx$",
		r"\.xls$",
		r"\.xlsx$",
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

	Repo.diff_tree_to_tree(
		Some(&Repo.revparse_single(Start)?.peel_to_commit()?.tree()?),
		Some(&Repo.revparse_single(End)?.peel_to_commit()?.tree()?),
		Some(&mut git2::DiffOptions::new()),
	Repository
		.diff_tree_to_tree(
			Some(&Repository.revparse_single(Start)?.peel_to_commit()?.tree()?),
			Some(&Repository.revparse_single(End)?.peel_to_commit()?.tree()?),
			Some(&mut Options),
		)?
	.print(git2::DiffFormat::Patch, |_, _, line| {
		Difference.push_str(std::str::from_utf8(line.content()).unwrap());
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
diff --git a/Source/Fn/Summary/Release.rs b/Source/Fn/Summary/Release.rs
deleted file mode 100644
index 611f1eb..0000000
--- a/Source/Fn/Summary/Release.rs
+++ /dev/null
@@ -1,33 +0,0 @@
/// Generates a release summary.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for generating the release summary.
///
/// # Returns
///
/// * `Return` - The generated release summary.
///
/// # Example
///
/// ```
/// let option = Option {
///     // Fields needed for release summary generation
/// };
/// let release_summary = Fn(&option);
/// ```
pub fn Fn(Difference: &str) -> String {
	let mut Release = String::new();

	Release.push_str("Release Notes:\n");

	for Difference in Difference.lines() {
		if Difference.starts_with("+") && !Difference.starts_with("+++") {
			Release.push_str(&format!("Added: {}\n", &Difference[1..]));
		} else if Difference.starts_with("-") && !Difference.starts_with("---") {
			Release.push_str(&format!("Removed: {}\n", &Difference[1..]));
		}
	}

	Release
}
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
index da03f34..4a356ae 100644
--- a/Source/Struct/Binary/Command.rs
+++ b/Source/Struct/Binary/Command.rs
@@ -19,7 +19,7 @@ impl Struct {
							Parallel::Fn(Option).await;
						}
						false => {
							Sequential::Fn(Option);
							Sequential::Fn(Option).await;
						}
					};
				})
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
index c0c6b89..45d3e41 100644
--- a/Source/Struct/Binary/Command/Entry.rs
+++ b/Source/Struct/Binary/Command/Entry.rs
@@ -11,12 +11,16 @@ pub struct Struct {

	/// The path separator character.
	pub Separator: Separator,

	/// List of items to omit from processing.
	pub Omit: Omit,
}

impl Struct {
	pub fn Fn(Option: &Option) -> Self {
		Self {
			Entry: crate::Fn::Binary::Command::Entry::Fn(Option),
			Omit: Option.Omit.clone(),
			Parallel: Option.Parallel,
			Pattern: Option.Pattern.clone(),
			Separator: Option.Separator,
@@ -24,7 +28,9 @@ impl Struct {
	}
}

use crate::Struct::Binary::Command::Option::{Parallel, Pattern, Separator, Struct as Option};
use crate::Struct::Binary::Command::Option::{
	Omit, Parallel, Pattern, Separator, Struct as Option,
};

/// Defines a type alias for a vector of vectors of strings.
pub type Type = Vec<Vec<String>>;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index 8453d33..8d32f31 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
@@ -3,6 +3,9 @@ pub struct Struct {
	/// List of patterns to exclude from processing.
	pub Exclude: Vec<String>,

	/// List of items to omit from processing.
	pub Omit: Vec<String>,

	/// Flag indicating whether to use parallel processing.
	pub Parallel: Parallel,

@@ -20,23 +23,29 @@ impl Struct {
	/// Creates a new Struct instance from the provided Option.
	pub fn Fn(Option { Separator, .. }: Option) -> Self {
		Self {
			Exclude: Fn()
			Exclude: Command()
				.get_one::<String>("Exclude")
				.expect("Cannot Exclude.")
				.split(" ")
				.map(|Command| Command.to_string())
				.map(|Exclude| Exclude.to_string())
				.collect::<Vec<_>>(),
			Parallel: Fn().get_flag("Parallel"),
			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Fn().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Parallel: Command().get_flag("Parallel"),
			Pattern: Command().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Command().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Separator,
			Omit: Command()
				.get_many::<String>("Omit")
				.expect("Cannot Omit.")
				.map(|Omit| Omit.to_string())
				.collect(),
		}
	}
}

use crate::{Fn::Binary::Command::Fn, Struct::Binary::Command::Struct as Option};
use crate::{Fn::Binary::Command::Fn as Command, Struct::Binary::Command::Struct as Option};

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;
pub type Omit = Vec<String>;
diff --git a/Source/Struct/mod.rs b/Source/Struct/mod.rs
index a56e8ce..4ca5f2b 100644
--- a/Source/Struct/mod.rs
+++ b/Source/Struct/mod.rs
@@ -1 +1,2 @@
pub mod Binary;
pub mod Summary;
diff --git a/Source/Struct/Summary/Difference.rs b/Source/Struct/Summary/Difference.rs
new file mode 100644
index 0000000..7583a0a
--- /dev/null
+++ b/Source/Struct/Summary/Difference.rs
@@ -0,0 +1,3 @@
pub struct Struct {
	pub Omit: Vec<String>,
}
diff --git a/Source/Struct/Summary/mod.rs b/Source/Struct/Summary/mod.rs
new file mode 100644
index 0000000..7241509
--- /dev/null
+++ b/Source/Struct/Summary/mod.rs
@@ -0,0 +1 @@
pub mod Difference;

diff --git a/build.rs b/build.rs
index 73ccc94..1f0de60 100644
--- a/build.rs
+++ b/build.rs
@@ -1,8 +1,5 @@
#![allow(non_snake_case)]

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Toml {
	package: Package,
@@ -24,3 +21,6 @@ fn main() {
		.version
	);
}

use serde::Deserialize;
use std::fs;
diff --git a/Cargo.toml b/Cargo.toml
index c769c35..c10016a 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -14,6 +14,7 @@ rayon = "1.10.0"
tokio = { version = "1.39.2", features = ["full"] }
git2 = { version = "0.19.0" }
num_cpus = "1.16.0"
regex = "1.10.5"

[build-dependencies]
serde = { version = "1.0.204", features = ["derive"] }
@@ -34,5 +35,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.2"
version = "0.0.3"
edition = "2021"
diff --git a/CHANGELOG.md b/CHANGELOG.md
index 68e22b1..2821f26 100644
--- a/CHANGELOG.md
+++ b/CHANGELOG.md
@@ -1,3 +1,7 @@
## 0.0.3

-   Cleanup

## 0.0.2

-   Version 2.0
diff --git a/README.md b/README.md
index fd4bfe5..018729b 100644
--- a/README.md
+++ b/README.md
@@ -1,11 +1,33 @@
# 🗣️ [Summary] —

`Summary` is a command-line tool that executes commands in multiple directories
simultaneously. It leverages parallel processing and concurrent `I/O` to
efficiently run tasks across directories.
`Summary` is a powerful command-line tool designed for efficient file processing
and summarization. It offers both sequential and parallel processing
capabilities, along with flexible file filtering options.

[Summary]: HTTPS://crates.io/crates/psummary

## Feature

-   Directory traversal and file filtering
-   Parallel and sequential processing modes
-   Customizable file pattern matching
-   Exclusion of specified files or directories
-   Integration with Pieces OS for enhanced functionality

## Pieces OS Integration

The `Summary` CLI supports Pieces OS, essentially acting as a plugin that can
rewrite the whole system. This integration allows for:

-   Enhanced code analysis and summarization.
-   Improved context-aware processing.
-   Seamless integration with other Pieces OS-compatible tools.
-   Potential for AI-driven insights and optimizations.

By leveraging Pieces OS, `Summary` can tap into a broader ecosystem of
development tools and services, significantly expanding its capabilities beyond
basic file processing.

## Installation

```sh
@@ -14,6 +36,18 @@ cargo install psummary

## Usage

The `Summary` tool can be used with various options:

-   `--Root` or `-R`: Set the current working directory
-   `--Parallel` or `-P`: Run commands in parallel
-   `--Exclude`: Exclude certain files or directories
-   `--Pattern`: Specify a custom pattern for matching
-   `--Separator`: Define a custom separator

For Pieces OS integration, refer to the Pieces OS documentation for specific
flags and configuration options.
[Pieces](HTTPS://GitHub.Com/PlayForm/Pieces.git)

```sh
Summary
```
@@ -60,10 +94,13 @@ Define a custom separator

`Summary` relies on several Rust crates to provide its functionality:

-   `clap` - Parses command-line arguments
-   `rayon` - Enables parallel processing
-   `tokio` - Provides an asynchronous runtime
-   `walkdir` - Facilitates efficient filesystem traversal
-   `clap` - For parsing command-line arguments.
-   `rayon` - For parallel processing.
-   `tokio` - For asynchronous runtime.
-   `walkdir` - For efficient filesystem traversal.

[Pieces OS](HTTPS://Pieces.App): For extended functionality and system
integration.

[Summary]: HTTPS://crates.io/crates/psummary

diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index c590122..4cfbb9f 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -16,6 +16,27 @@ pub fn Fn() -> ArgMatches {
		.version(env!("CARGO_PKG_VERSION"))
		.author("🖋️ Source — 👐🏻 Open — <Source/Open@PlayForm.Cloud>")
		.about("🗣️ Summary —")
		.arg(
			Arg::new("Exclude")
				.short('E')
				.long("Exclude")
				.display_order(4)
				.value_name("EXCLUDE")
				.required(false)
				.help("🚫 Exclude —")
				.default_value("node_modules"),
		)
		.arg(
			Arg::new("Omit")
				.short('O')
				.long("Omit")
				.display_order(6)
				.value_name("OMIT")
				.required(false)
				.help("🚫 Omit —")
				.action(clap::ArgAction::Append)
				.default_value("Documentation"),
		)
		.arg(
			Arg::new("Parallel")
				.short('P')
@@ -26,6 +47,15 @@ pub fn Fn() -> ArgMatches {
				.required(false)
				.help("⏩ Parallel —"),
		)
		.arg(
			Arg::new("Pattern")
				.long("Pattern")
				.display_order(5)
				.value_name("PATTERN")
				.required(false)
				.help("🔍 Pattern —")
				.default_value(".git"),
		)
		.arg(
			Arg::new("Root")
				.short('R')
@@ -36,24 +66,6 @@ pub fn Fn() -> ArgMatches {
				.help("📂 Root —")
				.default_value("."),
		)
		.arg(
			Arg::new("Exclude")
				.short('E')
				.long("Exclude")
				.display_order(4)
				.value_name("EXCLUDE")
				.required(false)
				.help("🚫 Exclude —")
				.default_value("node_modules"),
		)
		.arg(
			Arg::new("Pattern")
				.display_order(5)
				.value_name("PATTERN")
				.required(false)
				.help("🔍 Pattern —")
				.default_value(".git"),
		)
		.get_matches()
}

diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 37a5949..463f890 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -17,8 +17,8 @@
/// };
/// Fn(option).await;
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
	let Queue: Vec<_> = stream::iter(
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
@@ -29,20 +29,21 @@ pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
			})
			.collect::<Vec<String>>(),
	)
	.map(|Entry| async move {
		match crate::Fn::Summary::Fn(&Entry).await {
			Ok(summary) => Ok(summary),
			Err(e) => Err(format!("Error generating summary for {}: {}", Entry, e)),
	.map(|Entry| {
		let Omit = Omit.clone();

		async move {
			match crate::Fn::Summary::Fn(&Entry, &crate::Fn::Summary::Difference::Option { Omit })
				.await
			{
				Ok(Summary) => Ok(Summary),
				Err(_Error) => Err(format!("Error generating summary for {}: {}", Entry, _Error)),
			}
		}
	})
	.buffer_unordered(num_cpus::get())
	.collect()
	.collect::<Vec<_>>()
	.await;

	Queue.par_iter().for_each(|Output| match Output {
		Ok(Summary) => println!("Summary: {:?}", Summary),
		Err(Error) => eprintln!("Error: {}", Error),
	});
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index 63923f7..c19801b 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -17,7 +17,8 @@
/// };
/// Fn(option);
/// ```
pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
	futures::future::join_all(
		Entry
			.into_iter()
			.filter_map(|Entry| {
@@ -26,9 +27,25 @@ pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
		.for_each(|_Entry| {
			// TODO: GENERATE SUMMARY
			.map(|Entry| {
				let Omit = Omit.clone();

				async move {
					match crate::Fn::Summary::Fn(
						&Entry,
						&crate::Fn::Summary::Difference::Option { Omit },
					)
					.await
					{
						Ok(Summary) => Ok(Summary),
						Err(_Error) => {
							Err(format!("Error generating summary for {}: {}", Entry, _Error))
						}
					}
				}
			})
			.collect::<Vec<_>>(),
	).await;
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index eb2d2ce..f6825a8 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -16,29 +16,28 @@
/// };
/// let summary = Fn(&option);
/// ```
pub async fn Fn(Entry: &str) -> Result<(), Box<dyn std::error::Error>> {
	let Repository = Repository::open(Entry)?;

pub async fn Fn(
	Entry: &str,
	Option: &crate::Fn::Summary::Difference::Option,
) -> Result<(), Box<dyn std::error::Error>> {
	match Repository::open(Entry) {
		Ok(Repository) => {
			let Tag = Repository.tag_names(None)?;
	let mut Start = None;

	let Summary = "Summary";
	fs::create_dir_all(Summary)?;

	for i in 0..Tag.len() {
		if let Some(Tag) = Tag.get(i) {
			if let Some(Start) = Start {
				let Difference = crate::Fn::Summary::Difference::Fn(&Repository, Start, Tag)?;

				File::create(&format!("{}/Difference_{}_{}.txt", Summary, Start, Tag))?.write_all(
					crate::Fn::Summary::Difference::Fn(&Repository, Start, Tag)?.as_bytes(),
				)?;
			let Tags: Vec<_> = Tag.iter().filter_map(|Tag| Tag).collect();

				File::create(&format!("{}/Release_{}_{}.txt", Summary, Start, Tag))?
					.write_all(crate::Fn::Summary::Release::Fn(&Difference).as_bytes())?;
			for (Index, &Current) in Tags.iter().enumerate() {
				for (_, &Next) in Tags.iter().enumerate().skip(Index + 1) {
					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Current, Next, Option)?
					);
				}

			Start = Some(Tag);
			}
		}
		Err(_Error) => {
			println!("Failed to open repository: {}", _Error);
			return Err(_Error.into());
		}
	}

@@ -46,10 +45,5 @@ pub async fn Fn(Entry: &str) -> Result<(), Box<dyn std::error::Error>> {
}

use git2::Repository;
use std::{
	fs::{self, File},
	io::Write,
};

pub mod Difference;
pub mod Release;
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
index 8d7badf..4951c08 100644
--- a/Source/Fn/Summary/Difference.rs
+++ b/Source/Fn/Summary/Difference.rs
@@ -16,18 +16,61 @@
/// };
/// let difference = Fn(&option);
/// ```
pub fn Fn(Repo: &git2::Repository, Start: &str, End: &str) -> Result<String, git2::Error> {
pub fn Fn(
	Repository: &git2::Repository,
	Start: &str,
	End: &str,
	Option: &Option,
) -> Result<String, git2::Error> {
	let mut Difference = String::new();
	let mut Options = git2::DiffOptions::new();

	Repo.diff_tree_to_tree(
		Some(&Repo.revparse_single(Start)?.peel_to_commit()?.tree()?),
		Some(&Repo.revparse_single(End)?.peel_to_commit()?.tree()?),
		Some(&mut git2::DiffOptions::new()),
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

	Repository
		.diff_tree_to_tree(
			Some(&Repository.revparse_single(Start)?.peel_to_commit()?.tree()?),
			Some(&Repository.revparse_single(End)?.peel_to_commit()?.tree()?),
			Some(&mut Options),
		)?
	.print(git2::DiffFormat::Patch, |_, _, line| {
		Difference.push_str(std::str::from_utf8(line.content()).unwrap());
		.print(git2::DiffFormat::Patch, |Delta, _, Line| {
			if !Option
				.Omit
				.iter()
				.map(|Omit: &String| regex::Regex::new(Omit).expect("Cannot Regex."))
				.collect::<Vec<_>>()
				.iter()
				.any(|Omit| {
					Omit.is_match(&Delta.old_file().path().unwrap().display().to_string())
						|| Omit.is_match(&Delta.new_file().path().unwrap().display().to_string())
				}) {
				Difference.push_str(std::str::from_utf8(Line.content()).unwrap());
			};

			true
		})?;

	Ok(Difference)
}

pub struct Option {
	pub Omit: Vec<String>,
}
diff --git a/Source/Fn/Summary/Release.rs b/Source/Fn/Summary/Release.rs
deleted file mode 100644
index 611f1eb..0000000
--- a/Source/Fn/Summary/Release.rs
+++ /dev/null
@@ -1,33 +0,0 @@
/// Generates a release summary.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for generating the release summary.
///
/// # Returns
///
/// * `Return` - The generated release summary.
///
/// # Example
///
/// ```
/// let option = Option {
///     // Fields needed for release summary generation
/// };
/// let release_summary = Fn(&option);
/// ```
pub fn Fn(Difference: &str) -> String {
	let mut Release = String::new();

	Release.push_str("Release Notes:\n");

	for Difference in Difference.lines() {
		if Difference.starts_with("+") && !Difference.starts_with("+++") {
			Release.push_str(&format!("Added: {}\n", &Difference[1..]));
		} else if Difference.starts_with("-") && !Difference.starts_with("---") {
			Release.push_str(&format!("Removed: {}\n", &Difference[1..]));
		}
	}

	Release
}
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
index da03f34..4a356ae 100644
--- a/Source/Struct/Binary/Command.rs
+++ b/Source/Struct/Binary/Command.rs
@@ -19,7 +19,7 @@ impl Struct {
							Parallel::Fn(Option).await;
						}
						false => {
							Sequential::Fn(Option);
							Sequential::Fn(Option).await;
						}
					};
				})
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
index c0c6b89..45d3e41 100644
--- a/Source/Struct/Binary/Command/Entry.rs
+++ b/Source/Struct/Binary/Command/Entry.rs
@@ -11,12 +11,16 @@ pub struct Struct {

	/// The path separator character.
	pub Separator: Separator,

	/// List of items to omit from processing.
	pub Omit: Omit,
}

impl Struct {
	pub fn Fn(Option: &Option) -> Self {
		Self {
			Entry: crate::Fn::Binary::Command::Entry::Fn(Option),
			Omit: Option.Omit.clone(),
			Parallel: Option.Parallel,
			Pattern: Option.Pattern.clone(),
			Separator: Option.Separator,
@@ -24,7 +28,9 @@ impl Struct {
	}
}

use crate::Struct::Binary::Command::Option::{Parallel, Pattern, Separator, Struct as Option};
use crate::Struct::Binary::Command::Option::{
	Omit, Parallel, Pattern, Separator, Struct as Option,
};

/// Defines a type alias for a vector of vectors of strings.
pub type Type = Vec<Vec<String>>;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index 8453d33..8d32f31 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
@@ -3,6 +3,9 @@ pub struct Struct {
	/// List of patterns to exclude from processing.
	pub Exclude: Vec<String>,

	/// List of items to omit from processing.
	pub Omit: Vec<String>,

	/// Flag indicating whether to use parallel processing.
	pub Parallel: Parallel,

@@ -20,23 +23,29 @@ impl Struct {
	/// Creates a new Struct instance from the provided Option.
	pub fn Fn(Option { Separator, .. }: Option) -> Self {
		Self {
			Exclude: Fn()
			Exclude: Command()
				.get_one::<String>("Exclude")
				.expect("Cannot Exclude.")
				.split(" ")
				.map(|Command| Command.to_string())
				.map(|Exclude| Exclude.to_string())
				.collect::<Vec<_>>(),
			Parallel: Fn().get_flag("Parallel"),
			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Fn().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Parallel: Command().get_flag("Parallel"),
			Pattern: Command().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Command().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Separator,
			Omit: Command()
				.get_many::<String>("Omit")
				.expect("Cannot Omit.")
				.map(|Omit| Omit.to_string())
				.collect(),
		}
	}
}

use crate::{Fn::Binary::Command::Fn, Struct::Binary::Command::Struct as Option};
use crate::{Fn::Binary::Command::Fn as Command, Struct::Binary::Command::Struct as Option};

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;
pub type Omit = Vec<String>;

diff --git a/build.rs b/build.rs
index 73ccc94..1f0de60 100644
--- a/build.rs
+++ b/build.rs
@@ -1,8 +1,5 @@
#![allow(non_snake_case)]

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Toml {
	package: Package,
@@ -24,3 +21,6 @@ fn main() {
		.version
	);
}

use serde::Deserialize;
use std::fs;
diff --git a/Cargo.toml b/Cargo.toml
index c769c35..3d3159a 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -14,6 +14,7 @@ rayon = "1.10.0"
tokio = { version = "1.39.2", features = ["full"] }
git2 = { version = "0.19.0" }
num_cpus = "1.16.0"
regex = "1.10.5"

[build-dependencies]
serde = { version = "1.0.204", features = ["derive"] }
@@ -34,5 +35,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.2"
version = "0.0.4"
edition = "2021"
diff --git a/CHANGELOG.md b/CHANGELOG.md
index 68e22b1..0060a33 100644
--- a/CHANGELOG.md
+++ b/CHANGELOG.md
@@ -1,3 +1,11 @@
## 0.0.4

-   Cleanup

## 0.0.3

-   Cleanup

## 0.0.2

-   Version 2.0
diff --git a/README.md b/README.md
index fd4bfe5..49bf9ea 100644
--- a/README.md
+++ b/README.md
@@ -1,11 +1,33 @@
# 🗣️ [Summary] —

`Summary` is a command-line tool that executes commands in multiple directories
simultaneously. It leverages parallel processing and concurrent `I/O` to
efficiently run tasks across directories.
`Summary` is a powerful command-line tool designed for efficient Git repository
analysis and summarization. It offers both sequential and parallel processing
capabilities, along with flexible file filtering options.

[Summary]: HTTPS://crates.io/crates/psummary

## Features

-   Customizable file pattern matching
-   Diff generation between `Git` tags
-   Directory traversal and file filtering
-   Exclusion of specified files or directories
-   `Git` repository analysis
-   Integration with [Pieces OS] for enhanced functionality
-   Parallel and sequential processing modes

## [Pieces OS] Integration

The `Summary` CLI supports [Pieces OS], allowing it to:

-   Generate comprehensive diff logs and release notes automatically.
-   Provide AI-driven code analysis and insights.
-   Offer improved context-aware processing of repository changes.
-   Seamlessly interact with other [Pieces OS]-compatible development tools.

By leveraging [Pieces OS], `Summary` can tap into a broader ecosystem of development
tools and services, significantly expanding its capabilities beyond basic file processing.

## Installation

```sh
@@ -14,58 +36,97 @@ cargo install psummary

## Usage

```sh
Summary
```

This command will fetch from upstream for all `.git` repositories inside the
current directory. It essentially replaces the following command:
The Summary tool can be used with various options:

```sh
find -iname .git -type d -execdir git fetch upstream \;
```
🗣️ Summary —

Usage: Summary [OPTIONS]

Options:
  -P, --Parallel           ⏩ Parallel —
  -R, --Root <ROOT>        📂 Root — [default: .]
  -E, --Exclude <EXCLUDE>  🚫 Exclude — [default: node_modules]
      --Pattern <PATTERN>  🔍 Pattern — [default: .git]
  -O, --Omit <OMIT>        🚫 Omit — [default: Documentation]
  -h, --help               Print help
  -V, --version            Print version
```

This command will generate summaries for all the Git tags inside the specified
repository.

## Options

The `Summary` tool can be used with various options:

#### --Exclude or -E:

Exclude certain files or directories (defailt is `node_modules`).

#### --Omit or -O:

Specify regex patterns to omit files from processing (default is
"Documentation").

#### --Parallel or -P:

Run processing in parallel (default is `sequential`):

#### --Pattern:

Specify a custom pattern for matching (defailt is `.git`).

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):

```sh
Summary -R D:\Developer .git git fetch upstream
```
For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
flags and configuration options. [Pieces OS]

#### --Parallel or -P:
## Examples

Summary commands in `parallel` (default is `sequential`):
Analyze the current directory:

```sh
Summary -P -R D:\Developer .git git fetch upstream
Summary
```

#### --Exclude:
Analyze a specific directory in parallel:

Exclude certain files or directories (defailt is
`node_modules target dist vendor`)
```sh
Summary -P -R D:\Developer
```

#### --Pattern:
Exclude additional directories:

Specify a custom pattern for matching (defailt is `.git`)
```sh
Summary -E "node_modules target dist vendor"
```

#### --Separator:
Omit specific file patterns:

Define a custom separator
```sh
Summary -O "\.md$" -O "\.txt$"
```

## Dependencies

`Summary` relies on several Rust crates to provide its functionality:

-   `clap` - Parses command-line arguments
-   `rayon` - Enables parallel processing
-   `tokio` - Provides an asynchronous runtime
-   `walkdir` - Facilitates efficient filesystem traversal
-   `clap` - For parsing command-line arguments.
-   `futures` - For asynchronous programming abstractions.
-   `git2` - For Git repository operations.
-   `num_cpus` - For determining the number of CPUs for parallel processing.
-   `rayon` - For parallel processing.
-   `regex` - For pattern matching and text manipulation.
-   `tokio` - For asynchronous runtime.
-   `walkdir` - For efficient filesystem traversal.

[Pieces OS] For extended functionality and system integration.

[Summary]: HTTPS://crates.io/crates/psummary
[Pieces OS]: HTTPS://Pieces.App

## Changelog

diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index c590122..4cfbb9f 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -16,6 +16,27 @@ pub fn Fn() -> ArgMatches {
		.version(env!("CARGO_PKG_VERSION"))
		.author("🖋️ Source — 👐🏻 Open — <Source/Open@PlayForm.Cloud>")
		.about("🗣️ Summary —")
		.arg(
			Arg::new("Exclude")
				.short('E')
				.long("Exclude")
				.display_order(4)
				.value_name("EXCLUDE")
				.required(false)
				.help("🚫 Exclude —")
				.default_value("node_modules"),
		)
		.arg(
			Arg::new("Omit")
				.short('O')
				.long("Omit")
				.display_order(6)
				.value_name("OMIT")
				.required(false)
				.help("🚫 Omit —")
				.action(clap::ArgAction::Append)
				.default_value("Documentation"),
		)
		.arg(
			Arg::new("Parallel")
				.short('P')
@@ -26,6 +47,15 @@ pub fn Fn() -> ArgMatches {
				.required(false)
				.help("⏩ Parallel —"),
		)
		.arg(
			Arg::new("Pattern")
				.long("Pattern")
				.display_order(5)
				.value_name("PATTERN")
				.required(false)
				.help("🔍 Pattern —")
				.default_value(".git"),
		)
		.arg(
			Arg::new("Root")
				.short('R')
@@ -36,24 +66,6 @@ pub fn Fn() -> ArgMatches {
				.help("📂 Root —")
				.default_value("."),
		)
		.arg(
			Arg::new("Exclude")
				.short('E')
				.long("Exclude")
				.display_order(4)
				.value_name("EXCLUDE")
				.required(false)
				.help("🚫 Exclude —")
				.default_value("node_modules"),
		)
		.arg(
			Arg::new("Pattern")
				.display_order(5)
				.value_name("PATTERN")
				.required(false)
				.help("🔍 Pattern —")
				.default_value(".git"),
		)
		.get_matches()
}

diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 37a5949..463f890 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -17,8 +17,8 @@
/// };
/// Fn(option).await;
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
	let Queue: Vec<_> = stream::iter(
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
@@ -29,20 +29,21 @@ pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
			})
			.collect::<Vec<String>>(),
	)
	.map(|Entry| async move {
		match crate::Fn::Summary::Fn(&Entry).await {
			Ok(summary) => Ok(summary),
			Err(e) => Err(format!("Error generating summary for {}: {}", Entry, e)),
	.map(|Entry| {
		let Omit = Omit.clone();

		async move {
			match crate::Fn::Summary::Fn(&Entry, &crate::Fn::Summary::Difference::Option { Omit })
				.await
			{
				Ok(Summary) => Ok(Summary),
				Err(_Error) => Err(format!("Error generating summary for {}: {}", Entry, _Error)),
			}
		}
	})
	.buffer_unordered(num_cpus::get())
	.collect()
	.collect::<Vec<_>>()
	.await;

	Queue.par_iter().for_each(|Output| match Output {
		Ok(Summary) => println!("Summary: {:?}", Summary),
		Err(Error) => eprintln!("Error: {}", Error),
	});
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index 63923f7..c19801b 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -17,7 +17,8 @@
/// };
/// Fn(option);
/// ```
pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
	futures::future::join_all(
		Entry
			.into_iter()
			.filter_map(|Entry| {
@@ -26,9 +27,25 @@ pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
		.for_each(|_Entry| {
			// TODO: GENERATE SUMMARY
			.map(|Entry| {
				let Omit = Omit.clone();

				async move {
					match crate::Fn::Summary::Fn(
						&Entry,
						&crate::Fn::Summary::Difference::Option { Omit },
					)
					.await
					{
						Ok(Summary) => Ok(Summary),
						Err(_Error) => {
							Err(format!("Error generating summary for {}: {}", Entry, _Error))
						}
					}
				}
			})
			.collect::<Vec<_>>(),
	).await;
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index eb2d2ce..f6825a8 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -16,29 +16,28 @@
/// };
/// let summary = Fn(&option);
/// ```
pub async fn Fn(Entry: &str) -> Result<(), Box<dyn std::error::Error>> {
	let Repository = Repository::open(Entry)?;

pub async fn Fn(
	Entry: &str,
	Option: &crate::Fn::Summary::Difference::Option,
) -> Result<(), Box<dyn std::error::Error>> {
	match Repository::open(Entry) {
		Ok(Repository) => {
			let Tag = Repository.tag_names(None)?;
	let mut Start = None;

	let Summary = "Summary";
	fs::create_dir_all(Summary)?;

	for i in 0..Tag.len() {
		if let Some(Tag) = Tag.get(i) {
			if let Some(Start) = Start {
				let Difference = crate::Fn::Summary::Difference::Fn(&Repository, Start, Tag)?;

				File::create(&format!("{}/Difference_{}_{}.txt", Summary, Start, Tag))?.write_all(
					crate::Fn::Summary::Difference::Fn(&Repository, Start, Tag)?.as_bytes(),
				)?;
			let Tags: Vec<_> = Tag.iter().filter_map(|Tag| Tag).collect();

				File::create(&format!("{}/Release_{}_{}.txt", Summary, Start, Tag))?
					.write_all(crate::Fn::Summary::Release::Fn(&Difference).as_bytes())?;
			for (Index, &Current) in Tags.iter().enumerate() {
				for (_, &Next) in Tags.iter().enumerate().skip(Index + 1) {
					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Current, Next, Option)?
					);
				}

			Start = Some(Tag);
			}
		}
		Err(_Error) => {
			println!("Failed to open repository: {}", _Error);
			return Err(_Error.into());
		}
	}

@@ -46,10 +45,5 @@ pub async fn Fn(Entry: &str) -> Result<(), Box<dyn std::error::Error>> {
}

use git2::Repository;
use std::{
	fs::{self, File},
	io::Write,
};

pub mod Difference;
pub mod Release;
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
index 8d7badf..d96794d 100644
--- a/Source/Fn/Summary/Difference.rs
+++ b/Source/Fn/Summary/Difference.rs
@@ -16,18 +16,51 @@
/// };
/// let difference = Fn(&option);
/// ```
pub fn Fn(Repo: &git2::Repository, Start: &str, End: &str) -> Result<String, git2::Error> {
pub fn Fn(
	Repository: &git2::Repository,
	Start: &str,
	End: &str,
	Option: &Option,
) -> Result<String, git2::Error> {
	let mut Difference = String::new();
	let mut Options = git2::DiffOptions::new();

	Repo.diff_tree_to_tree(
		Some(&Repo.revparse_single(Start)?.peel_to_commit()?.tree()?),
		Some(&Repo.revparse_single(End)?.peel_to_commit()?.tree()?),
		Some(&mut git2::DiffOptions::new()),
	Options.indent_heuristic(true);
	Options.minimal(true);
	Options.force_text(true);
	Options.ignore_blank_lines(true);
	Options.ignore_case(true);
	Options.ignore_filemode(true);
	Options.ignore_whitespace(true);
	Options.ignore_whitespace_change(true);
	Options.ignore_whitespace_eol(true);

	Repository
		.diff_tree_to_tree(
			Some(&Repository.revparse_single(Start)?.peel_to_commit()?.tree()?),
			Some(&Repository.revparse_single(End)?.peel_to_commit()?.tree()?),
			Some(&mut Options),
		)?
	.print(git2::DiffFormat::Patch, |_, _, line| {
		Difference.push_str(std::str::from_utf8(line.content()).unwrap());
		.print(git2::DiffFormat::Patch, |Delta, _, Line| {
			if !Option
				.Omit
				.iter()
				.map(|Omit: &String| regex::Regex::new(Omit).expect("Cannot Regex."))
				.collect::<Vec<_>>()
				.iter()
				.any(|Omit| {
					Omit.is_match(&Delta.old_file().path().unwrap().display().to_string())
						|| Omit.is_match(&Delta.new_file().path().unwrap().display().to_string())
				}) {
				Difference.push_str(std::str::from_utf8(Line.content()).unwrap());
			};

			true
		})?;

	Ok(Difference)
}

pub struct Option {
	pub Omit: Vec<String>,
}
diff --git a/Source/Fn/Summary/Release.rs b/Source/Fn/Summary/Release.rs
deleted file mode 100644
index 611f1eb..0000000
--- a/Source/Fn/Summary/Release.rs
+++ /dev/null
@@ -1,33 +0,0 @@
/// Generates a release summary.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for generating the release summary.
///
/// # Returns
///
/// * `Return` - The generated release summary.
///
/// # Example
///
/// ```
/// let option = Option {
///     // Fields needed for release summary generation
/// };
/// let release_summary = Fn(&option);
/// ```
pub fn Fn(Difference: &str) -> String {
	let mut Release = String::new();

	Release.push_str("Release Notes:\n");

	for Difference in Difference.lines() {
		if Difference.starts_with("+") && !Difference.starts_with("+++") {
			Release.push_str(&format!("Added: {}\n", &Difference[1..]));
		} else if Difference.starts_with("-") && !Difference.starts_with("---") {
			Release.push_str(&format!("Removed: {}\n", &Difference[1..]));
		}
	}

	Release
}
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
index da03f34..4a356ae 100644
--- a/Source/Struct/Binary/Command.rs
+++ b/Source/Struct/Binary/Command.rs
@@ -19,7 +19,7 @@ impl Struct {
							Parallel::Fn(Option).await;
						}
						false => {
							Sequential::Fn(Option);
							Sequential::Fn(Option).await;
						}
					};
				})
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
index c0c6b89..45d3e41 100644
--- a/Source/Struct/Binary/Command/Entry.rs
+++ b/Source/Struct/Binary/Command/Entry.rs
@@ -11,12 +11,16 @@ pub struct Struct {

	/// The path separator character.
	pub Separator: Separator,

	/// List of items to omit from processing.
	pub Omit: Omit,
}

impl Struct {
	pub fn Fn(Option: &Option) -> Self {
		Self {
			Entry: crate::Fn::Binary::Command::Entry::Fn(Option),
			Omit: Option.Omit.clone(),
			Parallel: Option.Parallel,
			Pattern: Option.Pattern.clone(),
			Separator: Option.Separator,
@@ -24,7 +28,9 @@ impl Struct {
	}
}

use crate::Struct::Binary::Command::Option::{Parallel, Pattern, Separator, Struct as Option};
use crate::Struct::Binary::Command::Option::{
	Omit, Parallel, Pattern, Separator, Struct as Option,
};

/// Defines a type alias for a vector of vectors of strings.
pub type Type = Vec<Vec<String>>;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index 8453d33..8d32f31 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
@@ -3,6 +3,9 @@ pub struct Struct {
	/// List of patterns to exclude from processing.
	pub Exclude: Vec<String>,

	/// List of items to omit from processing.
	pub Omit: Vec<String>,

	/// Flag indicating whether to use parallel processing.
	pub Parallel: Parallel,

@@ -20,23 +23,29 @@ impl Struct {
	/// Creates a new Struct instance from the provided Option.
	pub fn Fn(Option { Separator, .. }: Option) -> Self {
		Self {
			Exclude: Fn()
			Exclude: Command()
				.get_one::<String>("Exclude")
				.expect("Cannot Exclude.")
				.split(" ")
				.map(|Command| Command.to_string())
				.map(|Exclude| Exclude.to_string())
				.collect::<Vec<_>>(),
			Parallel: Fn().get_flag("Parallel"),
			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Fn().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Parallel: Command().get_flag("Parallel"),
			Pattern: Command().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Command().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Separator,
			Omit: Command()
				.get_many::<String>("Omit")
				.expect("Cannot Omit.")
				.map(|Omit| Omit.to_string())
				.collect(),
		}
	}
}

use crate::{Fn::Binary::Command::Fn, Struct::Binary::Command::Struct as Option};
use crate::{Fn::Binary::Command::Fn as Command, Struct::Binary::Command::Struct as Option};

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;
pub type Omit = Vec<String>;

diff --git a/build.rs b/build.rs
index 73ccc94..1f0de60 100644
--- a/build.rs
+++ b/build.rs
@@ -1,8 +1,5 @@
#![allow(non_snake_case)]

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Toml {
	package: Package,
@@ -24,3 +21,6 @@ fn main() {
		.version
	);
}

use serde::Deserialize;
use std::fs;
diff --git a/Cargo.toml b/Cargo.toml
index c769c35..3615257 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -14,6 +14,7 @@ rayon = "1.10.0"
tokio = { version = "1.39.2", features = ["full"] }
git2 = { version = "0.19.0" }
num_cpus = "1.16.0"
regex = "1.10.5"

[build-dependencies]
serde = { version = "1.0.204", features = ["derive"] }
@@ -34,5 +35,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.2"
version = "0.0.5"
edition = "2021"
diff --git a/CHANGELOG.md b/CHANGELOG.md
index 68e22b1..56f65a8 100644
--- a/CHANGELOG.md
+++ b/CHANGELOG.md
@@ -1,3 +1,15 @@
## 0.0.5

-   Cleanup

## 0.0.4

-   Cleanup

## 0.0.3

-   Cleanup

## 0.0.2

-   Version 2.0
diff --git a/README.md b/README.md
index fd4bfe5..4449a1b 100644
--- a/README.md
+++ b/README.md
@@ -1,11 +1,33 @@
# 🗣️ [Summary] —

`Summary` is a command-line tool that executes commands in multiple directories
simultaneously. It leverages parallel processing and concurrent `I/O` to
efficiently run tasks across directories.
`Summary` is a powerful command-line tool designed for efficient `Git`
repository analysis and summarization. It offers both sequential and parallel
processing capabilities, along with flexible file filtering options.

[Summary]: HTTPS://crates.io/crates/psummary

## Features

-   Customizable file pattern matching.
-   Diff generation between `Git` tags.
-   Directory traversal and file filtering.
-   Exclusion of specified files or directories.
-   `Git` repository analysis.
-   Integration with [Pieces OS] for enhanced functionality.
-   Parallel and sequential processing modes.

## [Pieces OS] Integration

The `Summary` CLI supports [Pieces OS], allowing it to:

-   Generate comprehensive diff logs and release notes automatically.
-   Provide AI-driven code analysis and insights.
-   Offer improved context-aware processing of repository changes.
-   Seamlessly interact with other [Pieces OS]-compatible development tools.

By leveraging [Pieces OS], `Summary` can tap into a broader ecosystem of development
tools and services, significantly expanding its capabilities beyond basic file processing.

## Installation

```sh
@@ -14,58 +36,97 @@ cargo install psummary

## Usage

```sh
Summary
```

This command will fetch from upstream for all `.git` repositories inside the
current directory. It essentially replaces the following command:
The Summary tool can be used with various options:

```sh
find -iname .git -type d -execdir git fetch upstream \;
```
🗣️ Summary —

Usage: Summary [OPTIONS]

Options:
  -P, --Parallel           ⏩ Parallel —
  -R, --Root <ROOT>        📂 Root — [default: .]
  -E, --Exclude <EXCLUDE>  🚫 Exclude — [default: node_modules]
      --Pattern <PATTERN>  🔍 Pattern — [default: .git]
  -O, --Omit <OMIT>        🚫 Omit — [default: Documentation]
  -h, --help               Print help
  -V, --version            Print version
```

This command will generate summaries for all the `Git` tags inside the specified
repository.

## Options

The `Summary` tool can be used with various options:

#### --Exclude or -E:

Exclude certain files or directories (defailt is `node_modules`).

#### --Omit or -O:

Specify regex patterns to omit files from processing (default is
"Documentation").

#### --Parallel or -P:

Run processing in parallel (default is `sequential`):

#### --Pattern:

Specify a custom pattern for matching (defailt is `.git`).

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):

```sh
Summary -R D:\Developer .git git fetch upstream
```
For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
flags and configuration options. [Pieces OS]

#### --Parallel or -P:
## Examples

Summary commands in `parallel` (default is `sequential`):
Analyze the current directory:

```sh
Summary -P -R D:\Developer .git git fetch upstream
Summary
```

#### --Exclude:
Analyze a specific directory in parallel:

Exclude certain files or directories (defailt is
`node_modules target dist vendor`)
```sh
Summary -P -R D:\Developer
```

#### --Pattern:
Exclude additional directories:

Specify a custom pattern for matching (defailt is `.git`)
```sh
Summary -E "node_modules target dist vendor"
```

#### --Separator:
Omit specific file patterns:

Define a custom separator
```sh
Summary -O "\.md$" -O "\.txt$"
```

## Dependencies

`Summary` relies on several Rust crates to provide its functionality:

-   `clap` - Parses command-line arguments
-   `rayon` - Enables parallel processing
-   `tokio` - Provides an asynchronous runtime
-   `walkdir` - Facilitates efficient filesystem traversal
-   `clap` - For parsing command-line arguments.
-   `futures` - For asynchronous programming abstractions.
-   `git2` - For `Git` repository operations.
-   `num_cpus` - For determining the number of CPUs for parallel processing.
-   `rayon` - For parallel processing.
-   `regex` - For pattern matching and text manipulation.
-   `tokio` - For asynchronous runtime.
-   `walkdir` - For efficient filesystem traversal.

[Pieces OS] For extended functionality and system integration.

[Summary]: HTTPS://crates.io/crates/psummary
[Pieces OS]: HTTPS://Pieces.App

## Changelog

diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index c590122..4cfbb9f 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -16,6 +16,27 @@ pub fn Fn() -> ArgMatches {
		.version(env!("CARGO_PKG_VERSION"))
		.author("🖋️ Source — 👐🏻 Open — <Source/Open@PlayForm.Cloud>")
		.about("🗣️ Summary —")
		.arg(
			Arg::new("Exclude")
				.short('E')
				.long("Exclude")
				.display_order(4)
				.value_name("EXCLUDE")
				.required(false)
				.help("🚫 Exclude —")
				.default_value("node_modules"),
		)
		.arg(
			Arg::new("Omit")
				.short('O')
				.long("Omit")
				.display_order(6)
				.value_name("OMIT")
				.required(false)
				.help("🚫 Omit —")
				.action(clap::ArgAction::Append)
				.default_value("Documentation"),
		)
		.arg(
			Arg::new("Parallel")
				.short('P')
@@ -26,6 +47,15 @@ pub fn Fn() -> ArgMatches {
				.required(false)
				.help("⏩ Parallel —"),
		)
		.arg(
			Arg::new("Pattern")
				.long("Pattern")
				.display_order(5)
				.value_name("PATTERN")
				.required(false)
				.help("🔍 Pattern —")
				.default_value(".git"),
		)
		.arg(
			Arg::new("Root")
				.short('R')
@@ -36,24 +66,6 @@ pub fn Fn() -> ArgMatches {
				.help("📂 Root —")
				.default_value("."),
		)
		.arg(
			Arg::new("Exclude")
				.short('E')
				.long("Exclude")
				.display_order(4)
				.value_name("EXCLUDE")
				.required(false)
				.help("🚫 Exclude —")
				.default_value("node_modules"),
		)
		.arg(
			Arg::new("Pattern")
				.display_order(5)
				.value_name("PATTERN")
				.required(false)
				.help("🔍 Pattern —")
				.default_value(".git"),
		)
		.get_matches()
}

diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 37a5949..22654c0 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -17,8 +17,8 @@
/// };
/// Fn(option).await;
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
	let Queue: Vec<_> = stream::iter(
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	futures::stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
@@ -29,22 +29,27 @@ pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
			})
			.collect::<Vec<String>>(),
	)
	.map(|Entry| async move {
		match crate::Fn::Summary::Fn(&Entry).await {
			Ok(summary) => Ok(summary),
			Err(e) => Err(format!("Error generating summary for {}: {}", Entry, e)),
	.map(|Entry| {
		let Omit = Omit.clone();

		async move {
			match crate::Fn::Summary::Fn(
				&Entry,
				&crate::Struct::Summary::Difference::Struct { Omit },
			)
			.await
			{
				Ok(Summary) => Ok(Summary),
				Err(_Error) => Err(format!("Error generating summary for {}: {}", Entry, _Error)),
			}
		}
	})
	.buffer_unordered(num_cpus::get())
	.collect()
	.collect::<Vec<_>>()
	.await;

	Queue.par_iter().for_each(|Output| match Output {
		Ok(Summary) => println!("Summary: {:?}", Summary),
		Err(Error) => eprintln!("Error: {}", Error),
	});
}

use futures::stream::StreamExt;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::Struct::Binary::Command::Entry::Struct as Option;
use futures::stream::{self, StreamExt};
use rayon::prelude::*;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index 63923f7..7ffeb0f 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -17,7 +17,8 @@
/// };
/// Fn(option);
/// ```
pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
	futures::future::join_all(
		Entry
			.into_iter()
			.filter_map(|Entry| {
@@ -26,9 +27,26 @@ pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
		.for_each(|_Entry| {
			// TODO: GENERATE SUMMARY
			.map(|Entry| {
				let Omit = Omit.clone();

				async move {
					match crate::Fn::Summary::Fn(
						&Entry,
						&crate::Struct::Summary::Difference::Struct { Omit },
					)
					.await
					{
						Ok(Summary) => Ok(Summary),
						Err(_Error) => {
							Err(format!("Error generating summary for {}: {}", Entry, _Error))
						}
					}
				}
			})
			.collect::<Vec<_>>(),
	)
	.await;
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index eb2d2ce..ae3e650 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -16,29 +16,28 @@
/// };
/// let summary = Fn(&option);
/// ```
pub async fn Fn(Entry: &str) -> Result<(), Box<dyn std::error::Error>> {
	let Repository = Repository::open(Entry)?;

	let Tag = Repository.tag_names(None)?;
	let mut Start = None;

	let Summary = "Summary";
	fs::create_dir_all(Summary)?;

	for i in 0..Tag.len() {
		if let Some(Tag) = Tag.get(i) {
			if let Some(Start) = Start {
				let Difference = crate::Fn::Summary::Difference::Fn(&Repository, Start, Tag)?;

				File::create(&format!("{}/Difference_{}_{}.txt", Summary, Start, Tag))?.write_all(
					crate::Fn::Summary::Difference::Fn(&Repository, Start, Tag)?.as_bytes(),
				)?;

				File::create(&format!("{}/Release_{}_{}.txt", Summary, Start, Tag))?
					.write_all(crate::Fn::Summary::Release::Fn(&Difference).as_bytes())?;
pub async fn Fn(
	Entry: &str,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<(), Box<dyn std::error::Error>> {
	match Repository::open(Entry) {
		Ok(Repository) => {
			let Name = Repository.tag_names(None)?;

			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			for (Index, &Current) in Tag.iter().enumerate() {
				for (_, &Next) in Tag.iter().enumerate().skip(Index + 1) {
					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Current, Next, Option)?
					);
				}

			Start = Some(Tag);
			}
		}
		Err(_Error) => {
			println!("Failed to open repository: {}", _Error);
			return Err(_Error.into());
		}
	}

@@ -46,10 +45,5 @@ pub async fn Fn(Entry: &str) -> Result<(), Box<dyn std::error::Error>> {
}

use git2::Repository;
use std::{
	fs::{self, File},
	io::Write,
};

pub mod Difference;
pub mod Release;
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
index 8d7badf..cc25057 100644
--- a/Source/Fn/Summary/Difference.rs
+++ b/Source/Fn/Summary/Difference.rs
@@ -16,18 +16,107 @@
/// };
/// let difference = Fn(&option);
/// ```
pub fn Fn(Repo: &git2::Repository, Start: &str, End: &str) -> Result<String, git2::Error> {
pub fn Fn(
	Repository: &git2::Repository,
	Start: &str,
	End: &str,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<String, git2::Error> {
	let mut Omit = vec![
		r"\.pdf$",
		r"\.exe$",
		r"\.dll$",
		r"\.so$",
		r"\.dylib$",
		r"\.zip$",
		r"\.tar$",
		r"\.gz$",
		r"\.7z$",
		r"\.rar$",
		r"\.jpg$",
		r"\.jpeg$",
		r"\.png$",
		r"\.gif$",
		r"\.bmp$",
		r"\.tiff$",
		r"\.ico$",
		r"\.svg$",
		r"\.webp$",
		r"\.heic$",
		r"\.mp3$",
		r"\.wav$",
		r"\.ogg$",
		r"\.flac$",
		r"\.m4a$",
		r"\.mp4$",
		r"\.avi$",
		r"\.mov$",
		r"\.mkv$",
		r"\.wmv$",
		r"\.doc$",
		r"\.docx$",
		r"\.xls$",
		r"\.xlsx$",
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

	Repo.diff_tree_to_tree(
		Some(&Repo.revparse_single(Start)?.peel_to_commit()?.tree()?),
		Some(&Repo.revparse_single(End)?.peel_to_commit()?.tree()?),
		Some(&mut git2::DiffOptions::new()),
	Repository
		.diff_tree_to_tree(
			Some(&Repository.revparse_single(Start)?.peel_to_commit()?.tree()?),
			Some(&Repository.revparse_single(End)?.peel_to_commit()?.tree()?),
			Some(&mut Options),
		)?
	.print(git2::DiffFormat::Patch, |_, _, line| {
		Difference.push_str(std::str::from_utf8(line.content()).unwrap());
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
diff --git a/Source/Fn/Summary/Release.rs b/Source/Fn/Summary/Release.rs
deleted file mode 100644
index 611f1eb..0000000
--- a/Source/Fn/Summary/Release.rs
+++ /dev/null
@@ -1,33 +0,0 @@
/// Generates a release summary.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for generating the release summary.
///
/// # Returns
///
/// * `Return` - The generated release summary.
///
/// # Example
///
/// ```
/// let option = Option {
///     // Fields needed for release summary generation
/// };
/// let release_summary = Fn(&option);
/// ```
pub fn Fn(Difference: &str) -> String {
	let mut Release = String::new();

	Release.push_str("Release Notes:\n");

	for Difference in Difference.lines() {
		if Difference.starts_with("+") && !Difference.starts_with("+++") {
			Release.push_str(&format!("Added: {}\n", &Difference[1..]));
		} else if Difference.starts_with("-") && !Difference.starts_with("---") {
			Release.push_str(&format!("Removed: {}\n", &Difference[1..]));
		}
	}

	Release
}
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
index da03f34..4a356ae 100644
--- a/Source/Struct/Binary/Command.rs
+++ b/Source/Struct/Binary/Command.rs
@@ -19,7 +19,7 @@ impl Struct {
							Parallel::Fn(Option).await;
						}
						false => {
							Sequential::Fn(Option);
							Sequential::Fn(Option).await;
						}
					};
				})
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
index c0c6b89..45d3e41 100644
--- a/Source/Struct/Binary/Command/Entry.rs
+++ b/Source/Struct/Binary/Command/Entry.rs
@@ -11,12 +11,16 @@ pub struct Struct {

	/// The path separator character.
	pub Separator: Separator,

	/// List of items to omit from processing.
	pub Omit: Omit,
}

impl Struct {
	pub fn Fn(Option: &Option) -> Self {
		Self {
			Entry: crate::Fn::Binary::Command::Entry::Fn(Option),
			Omit: Option.Omit.clone(),
			Parallel: Option.Parallel,
			Pattern: Option.Pattern.clone(),
			Separator: Option.Separator,
@@ -24,7 +28,9 @@ impl Struct {
	}
}

use crate::Struct::Binary::Command::Option::{Parallel, Pattern, Separator, Struct as Option};
use crate::Struct::Binary::Command::Option::{
	Omit, Parallel, Pattern, Separator, Struct as Option,
};

/// Defines a type alias for a vector of vectors of strings.
pub type Type = Vec<Vec<String>>;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index 8453d33..8d32f31 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
@@ -3,6 +3,9 @@ pub struct Struct {
	/// List of patterns to exclude from processing.
	pub Exclude: Vec<String>,

	/// List of items to omit from processing.
	pub Omit: Vec<String>,

	/// Flag indicating whether to use parallel processing.
	pub Parallel: Parallel,

@@ -20,23 +23,29 @@ impl Struct {
	/// Creates a new Struct instance from the provided Option.
	pub fn Fn(Option { Separator, .. }: Option) -> Self {
		Self {
			Exclude: Fn()
			Exclude: Command()
				.get_one::<String>("Exclude")
				.expect("Cannot Exclude.")
				.split(" ")
				.map(|Command| Command.to_string())
				.map(|Exclude| Exclude.to_string())
				.collect::<Vec<_>>(),
			Parallel: Fn().get_flag("Parallel"),
			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Fn().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Parallel: Command().get_flag("Parallel"),
			Pattern: Command().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Command().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Separator,
			Omit: Command()
				.get_many::<String>("Omit")
				.expect("Cannot Omit.")
				.map(|Omit| Omit.to_string())
				.collect(),
		}
	}
}

use crate::{Fn::Binary::Command::Fn, Struct::Binary::Command::Struct as Option};
use crate::{Fn::Binary::Command::Fn as Command, Struct::Binary::Command::Struct as Option};

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;
pub type Omit = Vec<String>;
diff --git a/Source/Struct/mod.rs b/Source/Struct/mod.rs
index a56e8ce..4ca5f2b 100644
--- a/Source/Struct/mod.rs
+++ b/Source/Struct/mod.rs
@@ -1 +1,2 @@
pub mod Binary;
pub mod Summary;
diff --git a/Source/Struct/Summary/Difference.rs b/Source/Struct/Summary/Difference.rs
new file mode 100644
index 0000000..7583a0a
--- /dev/null
+++ b/Source/Struct/Summary/Difference.rs
@@ -0,0 +1,3 @@
pub struct Struct {
	pub Omit: Vec<String>,
}
diff --git a/Source/Struct/Summary/mod.rs b/Source/Struct/Summary/mod.rs
new file mode 100644
index 0000000..7241509
--- /dev/null
+++ b/Source/Struct/Summary/mod.rs
@@ -0,0 +1 @@
pub mod Difference;

diff --git a/Cargo.toml b/Cargo.toml
index c10016a..3d3159a 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -35,5 +35,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.3"
version = "0.0.4"
edition = "2021"
diff --git a/CHANGELOG.md b/CHANGELOG.md
index 2821f26..0060a33 100644
--- a/CHANGELOG.md
+++ b/CHANGELOG.md
@@ -1,3 +1,7 @@
## 0.0.4

-   Cleanup

## 0.0.3

-   Cleanup
diff --git a/README.md b/README.md
index 018729b..49bf9ea 100644
--- a/README.md
+++ b/README.md
@@ -1,32 +1,32 @@
# 🗣️ [Summary] —

`Summary` is a powerful command-line tool designed for efficient file processing
and summarization. It offers both sequential and parallel processing
`Summary` is a powerful command-line tool designed for efficient Git repository
analysis and summarization. It offers both sequential and parallel processing
capabilities, along with flexible file filtering options.

[Summary]: HTTPS://crates.io/crates/psummary

## Feature
## Features

-   Directory traversal and file filtering
-   Parallel and sequential processing modes
-   Customizable file pattern matching
-   Diff generation between `Git` tags
-   Directory traversal and file filtering
-   Exclusion of specified files or directories
-   Integration with Pieces OS for enhanced functionality
-   `Git` repository analysis
-   Integration with [Pieces OS] for enhanced functionality
-   Parallel and sequential processing modes

## Pieces OS Integration
## [Pieces OS] Integration

The `Summary` CLI supports Pieces OS, essentially acting as a plugin that can
rewrite the whole system. This integration allows for:
The `Summary` CLI supports [Pieces OS], allowing it to:

-   Enhanced code analysis and summarization.
-   Improved context-aware processing.
-   Seamless integration with other Pieces OS-compatible tools.
-   Potential for AI-driven insights and optimizations.
-   Generate comprehensive diff logs and release notes automatically.
-   Provide AI-driven code analysis and insights.
-   Offer improved context-aware processing of repository changes.
-   Seamlessly interact with other [Pieces OS]-compatible development tools.

By leveraging Pieces OS, `Summary` can tap into a broader ecosystem of
development tools and services, significantly expanding its capabilities beyond
basic file processing.
By leveraging [Pieces OS], `Summary` can tap into a broader ecosystem of development
tools and services, significantly expanding its capabilities beyond basic file processing.

## Installation

@@ -36,73 +36,97 @@ cargo install psummary

## Usage

The Summary tool can be used with various options:

```
🗣️ Summary —

Usage: Summary [OPTIONS]

Options:
  -P, --Parallel           ⏩ Parallel —
  -R, --Root <ROOT>        📂 Root — [default: .]
  -E, --Exclude <EXCLUDE>  🚫 Exclude — [default: node_modules]
      --Pattern <PATTERN>  🔍 Pattern — [default: .git]
  -O, --Omit <OMIT>        🚫 Omit — [default: Documentation]
  -h, --help               Print help
  -V, --version            Print version
```

This command will generate summaries for all the Git tags inside the specified
repository.

## Options

The `Summary` tool can be used with various options:

-   `--Root` or `-R`: Set the current working directory
-   `--Parallel` or `-P`: Run commands in parallel
-   `--Exclude`: Exclude certain files or directories
-   `--Pattern`: Specify a custom pattern for matching
-   `--Separator`: Define a custom separator
#### --Exclude or -E:

For Pieces OS integration, refer to the Pieces OS documentation for specific
flags and configuration options.
[Pieces](HTTPS://GitHub.Com/PlayForm/Pieces.git)
Exclude certain files or directories (defailt is `node_modules`).

```sh
Summary
```
#### --Omit or -O:

This command will fetch from upstream for all `.git` repositories inside the
current directory. It essentially replaces the following command:
Specify regex patterns to omit files from processing (default is
"Documentation").

```sh
find -iname .git -type d -execdir git fetch upstream \;
```
#### --Parallel or -P:

## Options
Run processing in parallel (default is `sequential`):

#### --Pattern:

Specify a custom pattern for matching (defailt is `.git`).

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):

```sh
Summary -R D:\Developer .git git fetch upstream
```
For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
flags and configuration options. [Pieces OS]

#### --Parallel or -P:
## Examples

Summary commands in `parallel` (default is `sequential`):
Analyze the current directory:

```sh
Summary -P -R D:\Developer .git git fetch upstream
Summary
```

#### --Exclude:
Analyze a specific directory in parallel:

Exclude certain files or directories (defailt is
`node_modules target dist vendor`)
```sh
Summary -P -R D:\Developer
```

#### --Pattern:
Exclude additional directories:

Specify a custom pattern for matching (defailt is `.git`)
```sh
Summary -E "node_modules target dist vendor"
```

#### --Separator:
Omit specific file patterns:

Define a custom separator
```sh
Summary -O "\.md$" -O "\.txt$"
```

## Dependencies

`Summary` relies on several Rust crates to provide its functionality:

-   `clap` - For parsing command-line arguments.
-   `futures` - For asynchronous programming abstractions.
-   `git2` - For Git repository operations.
-   `num_cpus` - For determining the number of CPUs for parallel processing.
-   `rayon` - For parallel processing.
-   `regex` - For pattern matching and text manipulation.
-   `tokio` - For asynchronous runtime.
-   `walkdir` - For efficient filesystem traversal.

[Pieces OS](HTTPS://Pieces.App): For extended functionality and system
integration.
[Pieces OS] For extended functionality and system integration.

[Summary]: HTTPS://crates.io/crates/psummary
[Pieces OS]: HTTPS://Pieces.App

## Changelog

diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
index 4951c08..d96794d 100644
--- a/Source/Fn/Summary/Difference.rs
+++ b/Source/Fn/Summary/Difference.rs
@@ -25,16 +25,6 @@ pub fn Fn(
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

diff --git a/Cargo.toml b/Cargo.toml
index c10016a..3615257 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -35,5 +35,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.3"
version = "0.0.5"
edition = "2021"
diff --git a/CHANGELOG.md b/CHANGELOG.md
index 2821f26..56f65a8 100644
--- a/CHANGELOG.md
+++ b/CHANGELOG.md
@@ -1,3 +1,11 @@
## 0.0.5

-   Cleanup

## 0.0.4

-   Cleanup

## 0.0.3

-   Cleanup
diff --git a/README.md b/README.md
index 018729b..4449a1b 100644
--- a/README.md
+++ b/README.md
@@ -1,32 +1,32 @@
# 🗣️ [Summary] —

`Summary` is a powerful command-line tool designed for efficient file processing
and summarization. It offers both sequential and parallel processing
capabilities, along with flexible file filtering options.
`Summary` is a powerful command-line tool designed for efficient `Git`
repository analysis and summarization. It offers both sequential and parallel
processing capabilities, along with flexible file filtering options.

[Summary]: HTTPS://crates.io/crates/psummary

## Feature
## Features

-   Directory traversal and file filtering
-   Parallel and sequential processing modes
-   Customizable file pattern matching
-   Exclusion of specified files or directories
-   Integration with Pieces OS for enhanced functionality
-   Customizable file pattern matching.
-   Diff generation between `Git` tags.
-   Directory traversal and file filtering.
-   Exclusion of specified files or directories.
-   `Git` repository analysis.
-   Integration with [Pieces OS] for enhanced functionality.
-   Parallel and sequential processing modes.

## Pieces OS Integration
## [Pieces OS] Integration

The `Summary` CLI supports Pieces OS, essentially acting as a plugin that can
rewrite the whole system. This integration allows for:
The `Summary` CLI supports [Pieces OS], allowing it to:

-   Enhanced code analysis and summarization.
-   Improved context-aware processing.
-   Seamless integration with other Pieces OS-compatible tools.
-   Potential for AI-driven insights and optimizations.
-   Generate comprehensive diff logs and release notes automatically.
-   Provide AI-driven code analysis and insights.
-   Offer improved context-aware processing of repository changes.
-   Seamlessly interact with other [Pieces OS]-compatible development tools.

By leveraging Pieces OS, `Summary` can tap into a broader ecosystem of
development tools and services, significantly expanding its capabilities beyond
basic file processing.
By leveraging [Pieces OS], `Summary` can tap into a broader ecosystem of development
tools and services, significantly expanding its capabilities beyond basic file processing.

## Installation

@@ -36,73 +36,97 @@ cargo install psummary

## Usage

The Summary tool can be used with various options:

```
🗣️ Summary —

Usage: Summary [OPTIONS]

Options:
  -P, --Parallel           ⏩ Parallel —
  -R, --Root <ROOT>        📂 Root — [default: .]
  -E, --Exclude <EXCLUDE>  🚫 Exclude — [default: node_modules]
      --Pattern <PATTERN>  🔍 Pattern — [default: .git]
  -O, --Omit <OMIT>        🚫 Omit — [default: Documentation]
  -h, --help               Print help
  -V, --version            Print version
```

This command will generate summaries for all the `Git` tags inside the specified
repository.

## Options

The `Summary` tool can be used with various options:

-   `--Root` or `-R`: Set the current working directory
-   `--Parallel` or `-P`: Run commands in parallel
-   `--Exclude`: Exclude certain files or directories
-   `--Pattern`: Specify a custom pattern for matching
-   `--Separator`: Define a custom separator
#### --Exclude or -E:

For Pieces OS integration, refer to the Pieces OS documentation for specific
flags and configuration options.
[Pieces](HTTPS://GitHub.Com/PlayForm/Pieces.git)
Exclude certain files or directories (defailt is `node_modules`).

```sh
Summary
```
#### --Omit or -O:

This command will fetch from upstream for all `.git` repositories inside the
current directory. It essentially replaces the following command:
Specify regex patterns to omit files from processing (default is
"Documentation").

```sh
find -iname .git -type d -execdir git fetch upstream \;
```
#### --Parallel or -P:

## Options
Run processing in parallel (default is `sequential`):

#### --Pattern:

Specify a custom pattern for matching (defailt is `.git`).

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):

```sh
Summary -R D:\Developer .git git fetch upstream
```
For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
flags and configuration options. [Pieces OS]

#### --Parallel or -P:
## Examples

Summary commands in `parallel` (default is `sequential`):
Analyze the current directory:

```sh
Summary -P -R D:\Developer .git git fetch upstream
Summary
```

#### --Exclude:
Analyze a specific directory in parallel:

Exclude certain files or directories (defailt is
`node_modules target dist vendor`)
```sh
Summary -P -R D:\Developer
```

#### --Pattern:
Exclude additional directories:

Specify a custom pattern for matching (defailt is `.git`)
```sh
Summary -E "node_modules target dist vendor"
```

#### --Separator:
Omit specific file patterns:

Define a custom separator
```sh
Summary -O "\.md$" -O "\.txt$"
```

## Dependencies

`Summary` relies on several Rust crates to provide its functionality:

-   `clap` - For parsing command-line arguments.
-   `futures` - For asynchronous programming abstractions.
-   `git2` - For `Git` repository operations.
-   `num_cpus` - For determining the number of CPUs for parallel processing.
-   `rayon` - For parallel processing.
-   `regex` - For pattern matching and text manipulation.
-   `tokio` - For asynchronous runtime.
-   `walkdir` - For efficient filesystem traversal.

[Pieces OS](HTTPS://Pieces.App): For extended functionality and system
integration.
[Pieces OS] For extended functionality and system integration.

[Summary]: HTTPS://crates.io/crates/psummary
[Pieces OS]: HTTPS://Pieces.App

## Changelog

diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 463f890..22654c0 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -18,7 +18,7 @@
/// Fn(option).await;
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	stream::iter(
	futures::stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
@@ -33,7 +33,10 @@ pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
		let Omit = Omit.clone();

		async move {
			match crate::Fn::Summary::Fn(&Entry, &crate::Fn::Summary::Difference::Option { Omit })
			match crate::Fn::Summary::Fn(
				&Entry,
				&crate::Struct::Summary::Difference::Struct { Omit },
			)
			.await
			{
				Ok(Summary) => Ok(Summary),
@@ -46,6 +49,7 @@ pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	.await;
}

use futures::stream::StreamExt;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::Struct::Binary::Command::Entry::Struct as Option;
use futures::stream::{self, StreamExt};
use rayon::prelude::*;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index c19801b..7ffeb0f 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -33,7 +33,7 @@ pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
				async move {
					match crate::Fn::Summary::Fn(
						&Entry,
						&crate::Fn::Summary::Difference::Option { Omit },
						&crate::Struct::Summary::Difference::Struct { Omit },
					)
					.await
					{
@@ -45,7 +45,8 @@ pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
				}
			})
			.collect::<Vec<_>>(),
	).await;
	)
	.await;
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index f6825a8..ae3e650 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -18,16 +18,16 @@
/// ```
pub async fn Fn(
	Entry: &str,
	Option: &crate::Fn::Summary::Difference::Option,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<(), Box<dyn std::error::Error>> {
	match Repository::open(Entry) {
		Ok(Repository) => {
			let Tag = Repository.tag_names(None)?;
			let Name = Repository.tag_names(None)?;

			let Tags: Vec<_> = Tag.iter().filter_map(|Tag| Tag).collect();
			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			for (Index, &Current) in Tags.iter().enumerate() {
				for (_, &Next) in Tags.iter().enumerate().skip(Index + 1) {
			for (Index, &Current) in Tag.iter().enumerate() {
				for (_, &Next) in Tag.iter().enumerate().skip(Index + 1) {
					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Current, Next, Option)?
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
index 4951c08..cc25057 100644
--- a/Source/Fn/Summary/Difference.rs
+++ b/Source/Fn/Summary/Difference.rs
@@ -20,20 +20,66 @@ pub fn Fn(
	Repository: &git2::Repository,
	Start: &str,
	End: &str,
	Option: &Option,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<String, git2::Error> {
	let mut Difference = String::new();
	let mut Options = git2::DiffOptions::new();
	let mut Omit = vec![
		r"\.pdf$",
		r"\.exe$",
		r"\.dll$",
		r"\.so$",
		r"\.dylib$",
		r"\.zip$",
		r"\.tar$",
		r"\.gz$",
		r"\.7z$",
		r"\.rar$",
		r"\.jpg$",
		r"\.jpeg$",
		r"\.png$",
		r"\.gif$",
		r"\.bmp$",
		r"\.tiff$",
		r"\.ico$",
		r"\.svg$",
		r"\.webp$",
		r"\.heic$",
		r"\.mp3$",
		r"\.wav$",
		r"\.ogg$",
		r"\.flac$",
		r"\.m4a$",
		r"\.mp4$",
		r"\.avi$",
		r"\.mov$",
		r"\.mkv$",
		r"\.wmv$",
		r"\.doc$",
		r"\.docx$",
		r"\.xls$",
		r"\.xlsx$",
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
	];

	Omit.extend(Option.Omit.iter().map(|Omit| Omit.as_str()));

	// Options.pathspec(
	// 	std::ffi::CString::new(
	// 		std::iter::once("*".to_string())
	// 			.chain(Option.Omit.iter().map(|Omit| format!("{}", Omit)))
	// 			.collect::<Vec<String>>()
	// 			.join("\0"),
	// 	)
	// 	.expect("Cannot create CString"),
	// );
	let Regex = Omit.into_par_iter().filter_map(|Omit| Regex::new(Omit).ok()).collect::<Vec<_>>();

	let mut Options = git2::DiffOptions::new();

	Options.indent_heuristic(true);
	Options.minimal(true);
@@ -44,6 +90,10 @@ pub fn Fn(
	Options.ignore_whitespace(true);
	Options.ignore_whitespace_change(true);
	Options.ignore_whitespace_eol(true);
	Options.show_binary(false);
	Options.force_binary(false);

	let mut Difference = String::new();

	Repository
		.diff_tree_to_tree(
@@ -52,17 +102,14 @@ pub fn Fn(
			Some(&mut Options),
		)?
		.print(git2::DiffFormat::Patch, |Delta, _, Line| {
			if !Option
				.Omit
				.iter()
				.map(|Omit: &String| regex::Regex::new(Omit).expect("Cannot Regex."))
				.collect::<Vec<_>>()
				.iter()
				.any(|Omit| {
			if !Regex.iter().any(|Omit| {
				Omit.is_match(&Delta.old_file().path().unwrap().display().to_string())
					|| Omit.is_match(&Delta.new_file().path().unwrap().display().to_string())
			}) {
				Difference.push_str(std::str::from_utf8(Line.content()).unwrap());
				match std::str::from_utf8(Line.content()) {
					Ok(Line) => Difference.push_str(Line),
					Err(_) => (),
				}
			};

			true
@@ -71,6 +118,5 @@ pub fn Fn(
	Ok(Difference)
}

pub struct Option {
	pub Omit: Vec<String>,
}
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
diff --git a/Source/Struct/mod.rs b/Source/Struct/mod.rs
index a56e8ce..4ca5f2b 100644
--- a/Source/Struct/mod.rs
+++ b/Source/Struct/mod.rs
@@ -1 +1,2 @@
pub mod Binary;
pub mod Summary;
diff --git a/Source/Struct/Summary/Difference.rs b/Source/Struct/Summary/Difference.rs
new file mode 100644
index 0000000..7583a0a
--- /dev/null
+++ b/Source/Struct/Summary/Difference.rs
@@ -0,0 +1,3 @@
pub struct Struct {
	pub Omit: Vec<String>,
}
diff --git a/Source/Struct/Summary/mod.rs b/Source/Struct/Summary/mod.rs
new file mode 100644
index 0000000..7241509
--- /dev/null
+++ b/Source/Struct/Summary/mod.rs
@@ -0,0 +1 @@
pub mod Difference;

diff --git a/Cargo.toml b/Cargo.toml
index 3d3159a..3615257 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -35,5 +35,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.4"
version = "0.0.5"
edition = "2021"
diff --git a/CHANGELOG.md b/CHANGELOG.md
index 0060a33..56f65a8 100644
--- a/CHANGELOG.md
+++ b/CHANGELOG.md
@@ -1,3 +1,7 @@
## 0.0.5

-   Cleanup

## 0.0.4

-   Cleanup
diff --git a/README.md b/README.md
index 49bf9ea..4449a1b 100644
--- a/README.md
+++ b/README.md
@@ -1,20 +1,20 @@
# 🗣️ [Summary] —

`Summary` is a powerful command-line tool designed for efficient Git repository
analysis and summarization. It offers both sequential and parallel processing
capabilities, along with flexible file filtering options.
`Summary` is a powerful command-line tool designed for efficient `Git`
repository analysis and summarization. It offers both sequential and parallel
processing capabilities, along with flexible file filtering options.

[Summary]: HTTPS://crates.io/crates/psummary

## Features

-   Customizable file pattern matching
-   Diff generation between `Git` tags
-   Directory traversal and file filtering
-   Exclusion of specified files or directories
-   `Git` repository analysis
-   Integration with [Pieces OS] for enhanced functionality
-   Parallel and sequential processing modes
-   Customizable file pattern matching.
-   Diff generation between `Git` tags.
-   Directory traversal and file filtering.
-   Exclusion of specified files or directories.
-   `Git` repository analysis.
-   Integration with [Pieces OS] for enhanced functionality.
-   Parallel and sequential processing modes.

## [Pieces OS] Integration

@@ -53,7 +53,7 @@ Options:
  -V, --version            Print version
```

This command will generate summaries for all the Git tags inside the specified
This command will generate summaries for all the `Git` tags inside the specified
repository.

## Options
@@ -116,7 +116,7 @@ Summary -O "\.md$" -O "\.txt$"

-   `clap` - For parsing command-line arguments.
-   `futures` - For asynchronous programming abstractions.
-   `git2` - For Git repository operations.
-   `git2` - For `Git` repository operations.
-   `num_cpus` - For determining the number of CPUs for parallel processing.
-   `rayon` - For parallel processing.
-   `regex` - For pattern matching and text manipulation.
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 463f890..22654c0 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -18,7 +18,7 @@
/// Fn(option).await;
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	stream::iter(
	futures::stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
@@ -33,7 +33,10 @@ pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
		let Omit = Omit.clone();

		async move {
			match crate::Fn::Summary::Fn(&Entry, &crate::Fn::Summary::Difference::Option { Omit })
			match crate::Fn::Summary::Fn(
				&Entry,
				&crate::Struct::Summary::Difference::Struct { Omit },
			)
			.await
			{
				Ok(Summary) => Ok(Summary),
@@ -46,6 +49,7 @@ pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	.await;
}

use futures::stream::StreamExt;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::Struct::Binary::Command::Entry::Struct as Option;
use futures::stream::{self, StreamExt};
use rayon::prelude::*;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index c19801b..7ffeb0f 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -33,7 +33,7 @@ pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
				async move {
					match crate::Fn::Summary::Fn(
						&Entry,
						&crate::Fn::Summary::Difference::Option { Omit },
						&crate::Struct::Summary::Difference::Struct { Omit },
					)
					.await
					{
@@ -45,7 +45,8 @@ pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
				}
			})
			.collect::<Vec<_>>(),
	).await;
	)
	.await;
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index f6825a8..ae3e650 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -18,16 +18,16 @@
/// ```
pub async fn Fn(
	Entry: &str,
	Option: &crate::Fn::Summary::Difference::Option,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<(), Box<dyn std::error::Error>> {
	match Repository::open(Entry) {
		Ok(Repository) => {
			let Tag = Repository.tag_names(None)?;
			let Name = Repository.tag_names(None)?;

			let Tags: Vec<_> = Tag.iter().filter_map(|Tag| Tag).collect();
			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			for (Index, &Current) in Tags.iter().enumerate() {
				for (_, &Next) in Tags.iter().enumerate().skip(Index + 1) {
			for (Index, &Current) in Tag.iter().enumerate() {
				for (_, &Next) in Tag.iter().enumerate().skip(Index + 1) {
					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Current, Next, Option)?
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
index d96794d..cc25057 100644
--- a/Source/Fn/Summary/Difference.rs
+++ b/Source/Fn/Summary/Difference.rs
@@ -20,9 +20,65 @@ pub fn Fn(
	Repository: &git2::Repository,
	Start: &str,
	End: &str,
	Option: &Option,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<String, git2::Error> {
	let mut Difference = String::new();
	let mut Omit = vec![
		r"\.pdf$",
		r"\.exe$",
		r"\.dll$",
		r"\.so$",
		r"\.dylib$",
		r"\.zip$",
		r"\.tar$",
		r"\.gz$",
		r"\.7z$",
		r"\.rar$",
		r"\.jpg$",
		r"\.jpeg$",
		r"\.png$",
		r"\.gif$",
		r"\.bmp$",
		r"\.tiff$",
		r"\.ico$",
		r"\.svg$",
		r"\.webp$",
		r"\.heic$",
		r"\.mp3$",
		r"\.wav$",
		r"\.ogg$",
		r"\.flac$",
		r"\.m4a$",
		r"\.mp4$",
		r"\.avi$",
		r"\.mov$",
		r"\.mkv$",
		r"\.wmv$",
		r"\.doc$",
		r"\.docx$",
		r"\.xls$",
		r"\.xlsx$",
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
	];

	Omit.extend(Option.Omit.iter().map(|Omit| Omit.as_str()));

	let Regex = Omit.into_par_iter().filter_map(|Omit| Regex::new(Omit).ok()).collect::<Vec<_>>();

	let mut Options = git2::DiffOptions::new();

	Options.indent_heuristic(true);
@@ -34,6 +90,10 @@ pub fn Fn(
	Options.ignore_whitespace(true);
	Options.ignore_whitespace_change(true);
	Options.ignore_whitespace_eol(true);
	Options.show_binary(false);
	Options.force_binary(false);

	let mut Difference = String::new();

	Repository
		.diff_tree_to_tree(
@@ -42,17 +102,14 @@ pub fn Fn(
			Some(&mut Options),
		)?
		.print(git2::DiffFormat::Patch, |Delta, _, Line| {
			if !Option
				.Omit
				.iter()
				.map(|Omit: &String| regex::Regex::new(Omit).expect("Cannot Regex."))
				.collect::<Vec<_>>()
				.iter()
				.any(|Omit| {
			if !Regex.iter().any(|Omit| {
				Omit.is_match(&Delta.old_file().path().unwrap().display().to_string())
					|| Omit.is_match(&Delta.new_file().path().unwrap().display().to_string())
			}) {
				Difference.push_str(std::str::from_utf8(Line.content()).unwrap());
				match std::str::from_utf8(Line.content()) {
					Ok(Line) => Difference.push_str(Line),
					Err(_) => (),
				}
			};

			true
@@ -61,6 +118,5 @@ pub fn Fn(
	Ok(Difference)
}

pub struct Option {
	pub Omit: Vec<String>,
}
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
diff --git a/Source/Struct/mod.rs b/Source/Struct/mod.rs
index a56e8ce..4ca5f2b 100644
--- a/Source/Struct/mod.rs
+++ b/Source/Struct/mod.rs
@@ -1 +1,2 @@
pub mod Binary;
pub mod Summary;
diff --git a/Source/Struct/Summary/Difference.rs b/Source/Struct/Summary/Difference.rs
new file mode 100644
index 0000000..7583a0a
--- /dev/null
+++ b/Source/Struct/Summary/Difference.rs
@@ -0,0 +1,3 @@
pub struct Struct {
	pub Omit: Vec<String>,
}
diff --git a/Source/Struct/Summary/mod.rs b/Source/Struct/Summary/mod.rs
new file mode 100644
index 0000000..7241509
--- /dev/null
+++ b/Source/Struct/Summary/mod.rs
@@ -0,0 +1 @@
pub mod Difference;

