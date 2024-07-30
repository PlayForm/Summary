🗣️ Summary from Summary/v0.0.1 to Summary/v0.0.2 in .
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

🗣️ Summary from Summary/v0.0.2 to Summary/v0.0.3 in .
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

🗣️ Summary from Summary/v0.0.3 to Summary/v0.0.4 in .
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

🗣️ Summary from Summary/v0.0.4 to Summary/v0.0.5 in .
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

🗣️ Summary from Summary/v0.0.5 to Summary/v0.0.6 in .
diff --git a/Cargo.toml b/Cargo.toml
index 3615257..7c5b90e 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -18,7 +18,7 @@ regex = "1.10.5"

[build-dependencies]
serde = { version = "1.0.204", features = ["derive"] }
toml = "0.8.16"
toml = "0.8.17"

[lib]
crate-type = ["staticlib", "cdylib", "rlib"]
@@ -35,5 +35,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.5"
version = "0.0.6"
edition = "2021"
diff --git a/README.md b/README.md
index 4449a1b..d388d49 100644
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
Summary -P -O Target -O target -O Summary.md -O CHANGELOG.md > Summary.md
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

@@ -67,7 +74,7 @@ Exclude certain files or directories (defailt is `node_modules`).
#### --Omit or -O:

Specify regex patterns to omit files from processing (default is
"Documentation").
`Documentation`).

#### --Parallel or -P:

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
index ae3e650..2411611 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -1,21 +1,20 @@
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
/// Returns a Result containing () if successful, or a boxed dynamic error if an error occurs.
///
/// # Example
/// # Errors
///
/// ```
/// let option = Option {
///     // Fields needed for summary generation
/// };
/// let summary = Fn(&option);
/// ```
/// This function will return an error if the repository cannot be opened or if there are issues
/// generating the summary.
pub async fn Fn(
	Entry: &str,
	Option: &crate::Struct::Summary::Difference::Struct,
@@ -26,17 +25,52 @@ pub async fn Fn(

			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			for (Index, &Current) in Tag.iter().enumerate() {
				for (_, &Next) in Tag.iter().enumerate().skip(Index + 1) {
			let Head = Repository.head()?;

			let First = Repository.find_commit(First::Fn(&Repository)?)?.id().to_string();

			let Last = Head.peel_to_commit()?.id().to_string();

			if Tag.is_empty() {
				println!("🗣️ Summary from first commit to last commit:");

				println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Current, Next, Option)?
					"```\n{}\n```",
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option,)?
				);
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					println!("🗣️ Summary from tag: {} to tag: {}:", Start, End);

					println!(
						"```\n{}\n```",
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?
					);
				}

				if let Some(Latest) = Tag.last() {
					println!("🗣️ Summary from first commit to latest tag: {}:", Latest);

					println!(
						"```\n{}\n```",
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?
					);

					println!("🗣️ Summary from latest tag: {} to last commit:", Latest);

					println!(
						"```\n{}\n```",
						crate::Fn::Summary::Difference::Fn(&Repository, Latest, &Last, Option)?
					);
				}
			}
		}
		Err(_Error) => {
			println!("Failed to open repository: {}", _Error);
			println!("Cannot Repository: {}", _Error);

			return Err(_Error.into());
		}
	}
@@ -47,3 +81,4 @@ pub async fn Fn(
use git2::Repository;

pub mod Difference;
pub mod First;
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
new file mode 100644
index 0000000..a0c3df6
--- /dev/null
+++ b/Source/Fn/Summary/First.rs
@@ -0,0 +1,26 @@
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

🗣️ Summary from Summary/v0.0.6 to Summary/v0.0.7 in .
diff --git a/Cargo.toml b/Cargo.toml
index 7c5b90e..c907ec5 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -35,5 +35,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.6"
version = "0.0.7"
edition = "2021"
diff --git a/README.md b/README.md
index d388d49..c3006f1 100644
--- a/README.md
+++ b/README.md
@@ -7,11 +7,11 @@ processing capabilities, along with flexible file filtering options.
[Summary]: HTTPS://crates.io/crates/psummary

```sh
Summary -P -O Target -O target -O Summary.md -O CHANGELOG.md > Summary.md
Summary -P > SUMMARY.md
```

[Summary] will now generate the following [Summary.md](./Summary.md) for all the
commits and tags between the first and the latest commit.
[Summary] will now generate the following [SUMMARY.md](./SUMMARY.md) for all the
commits and tags between the first and the last commit.

## Features

@@ -69,24 +69,61 @@ The [Summary] tool can be used with various options:

#### --Exclude or -E:

Exclude certain files or directories (defailt is `node_modules`).
Exclude certain files or directories.

Default is:

```sh
Summary -E node_modules
```

#### --Omit or -O:

Specify regex patterns to omit files from processing (default is
`Documentation`).
Specify regex patterns to omit files from processing.

Default is:

```sh
Summary \
	--Omit "Target" \
	--Omit "target" \
	--Omit "Documentation" \
	--Omit "documentation" \
	--Omit "SUMMARY.md" \
	--Omit "CHANGELOG.md" \
	--Omit "summary.md" \
	--Omit "changelog.md"
```

#### --Parallel or -P:

Run processing in parallel (default is `sequential`):
Run processing in parallel.

Default is:

```sh
Summary
```

#### --Pattern:

Specify a custom pattern for matching (defailt is `.git`).
Specify a custom pattern for matching.

Default is:

```sh
Summary --Pattern .git
```

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):
Set the current working directory to a different folder.

Default is:

```sh
Summary --Root .
```

For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
flags and configuration options. [Pieces OS]
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index e355a33..fbdff71 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -32,7 +32,16 @@ pub fn Fn() -> ArgMatches {
				.required(false)
				.help("🚫 Omit —")
				.action(clap::ArgAction::Append)
				.default_values(["Target", "Documentation", r"Summary\.md$"]),
				.default_values([
					"Target",
					"target",
					"Documentation",
					"documentation",
					"SUMMARY.md",
					"CHANGELOG.md",
					"summary.md",
					"changelog.md",
				]),
		)
		.arg(
			Arg::new("Parallel")
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index 2411611..f199ca3 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -35,7 +35,7 @@ pub async fn Fn(
				println!("🗣️ Summary from first commit to last commit:");

				println!(
					"```\n{}\n```",
					"{}",
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option,)?
				);
			} else {
@@ -46,7 +46,7 @@ pub async fn Fn(
					println!("🗣️ Summary from tag: {} to tag: {}:", Start, End);

					println!(
						"```\n{}\n```",
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?
					);
				}
@@ -55,14 +55,14 @@ pub async fn Fn(
					println!("🗣️ Summary from first commit to latest tag: {}:", Latest);

					println!(
						"```\n{}\n```",
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?
					);

					println!("🗣️ Summary from latest tag: {} to last commit:", Latest);

					println!(
						"```\n{}\n```",
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Latest, &Last, Option)?
					);
				}

🗣️ Summary from Summary/v0.0.7 to Summary/v0.0.8 in .
diff --git a/Cargo.toml b/Cargo.toml
index c907ec5..13e711a 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -15,6 +15,7 @@ tokio = { version = "1.39.2", features = ["full"] }
git2 = { version = "0.19.0" }
num_cpus = "1.16.0"
regex = "1.10.5"
dashmap = "6.0.1"

[build-dependencies]
serde = { version = "1.0.204", features = ["derive"] }
@@ -35,5 +36,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.7"
version = "0.0.8"
edition = "2021"
diff --git a/README.md b/README.md
index c3006f1..1bf06be 100644
--- a/README.md
+++ b/README.md
@@ -7,10 +7,10 @@ processing capabilities, along with flexible file filtering options.
[Summary]: HTTPS://crates.io/crates/psummary

```sh
Summary -P > SUMMARY.md
Summary -P > Summary.md
```

[Summary] will now generate the following [SUMMARY.md](./SUMMARY.md) for all the
[Summary] will now generate the following [Summary.md](./Summary.md) for all the
commits and tags between the first and the last commit.

## Features
@@ -74,7 +74,7 @@ Exclude certain files or directories.
Default is:

```sh
Summary -E node_modules
Summary -P -E node_modules
```

#### --Omit or -O:
@@ -84,15 +84,11 @@ Specify regex patterns to omit files from processing.
Default is:

```sh
Summary \
	--Omit "Target" \
	--Omit "target" \
	--Omit "Documentation" \
	--Omit "documentation" \
	--Omit "SUMMARY.md" \
	--Omit "CHANGELOG.md" \
	--Omit "summary.md" \
	--Omit "changelog.md"
Summary -P \
	--Omit "(?i)documentation" \
	--Omit "(?i)target" \
	--Omit "(?i)changelog\.md$" \
	--Omit "(?i)summary\.md$"
```

#### --Parallel or -P:
@@ -112,7 +108,7 @@ Specify a custom pattern for matching.
Default is:

```sh
Summary --Pattern .git
Summary -P --Pattern .git
```

#### --Root or -R:
@@ -122,7 +118,7 @@ Set the current working directory to a different folder.
Default is:

```sh
Summary --Root .
Summary -P --Root .
```

For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
@@ -145,13 +141,13 @@ Summary -P -R D:\Developer
Exclude additional directories:

```sh
Summary -E "node_modules target dist vendor"
Summary -P -E "node_modules target dist vendor"
```

Omit specific file patterns:

```sh
Summary -O "\.md$" -O "\.txt$"
Summary -P -O "\.md$" -O "\.txt$"
```

## Dependencies
diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index fbdff71..4686859 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -1,13 +1,39 @@
//! This module defines the command-line interface for the Summary application.

/// Configures and returns the command-line argument matches for the Summary application.
/// Creates and returns the command-line argument matches for the `Summary` application.
///
/// This function sets up the command-line interface using the clap crate, defining
/// various arguments and their properties.
/// This function sets up the command-line interface using the `clap` crate, defining various
/// arguments and their properties such as short and long names, help messages, default values,
/// and whether they are required.
///
/// # Returns
///
/// Returns an `ArgMatches` struct containing the parsed command-line arguments.
/// Returns an `ArgMatches` instance containing the parsed command-line arguments.
///
/// # Arguments
///
/// * `Exclude` - An optional argument to specify patterns to exclude. Default is "node_modules".
/// * `Omit` - An optional argument to specify patterns to omit. Default values are:
///   - "(?i)documentation"
///   - "(?i)target"
///   - "(?i)changelog\.md$"
///   - "(?i)summary\.md$"
/// * `Parallel` - An optional flag to enable parallel processing.
/// * `Pattern` - An optional argument to specify a pattern to match. Default is ".git".
/// * `Root` - An optional argument to specify the root directory. Default is ".".
///
/// # Example
///
/// ```rust
/// let matches = Fn();
/// let exclude = matches.value_of("Exclude").unwrap_or("node_modules");
/// let omit = matches.values_of("Omit").unwrap_or_default().collect::<Vec<_>>();
/// let parallel = matches.is_present("Parallel");
/// let pattern = matches.value_of("Pattern").unwrap_or(".git");
/// let root = matches.value_of("Root").unwrap_or(".");
/// ```
///
/// # Errors
///
/// This function will panic if there are issues with the argument definitions or parsing.
pub fn Fn() -> ArgMatches {
	Command::new("Summary")
		.version(env!("CARGO_PKG_VERSION"))
@@ -33,14 +59,10 @@ pub fn Fn() -> ArgMatches {
				.help("🚫 Omit —")
				.action(clap::ArgAction::Append)
				.default_values([
					"Target",
					"target",
					"Documentation",
					"documentation",
					"SUMMARY.md",
					"CHANGELOG.md",
					"summary.md",
					"changelog.md",
					"(?i)documentation",
					"(?i)target",
					r"(?i)changelog\.md$",
					r"(?i)summary\.md$",
				]),
		)
		.arg(
diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
index 387c0e4..8918e6f 100644
--- a/Source/Fn/Binary/Command/Entry.rs
+++ b/Source/Fn/Binary/Command/Entry.rs
@@ -1,14 +1,37 @@
//! This module provides functionality for processing binary command entries.

/// Processes entries based on the provided options.
/// Generates a list of file paths from the specified root directory, excluding paths that match
/// any of the specified exclude patterns.
///
/// # Arguments
///
/// * `Option` - A reference to an Option struct containing processing parameters.
/// * `Option` - A reference to an `Option` struct containing the following fields:
///   - `Exclude`: A vector of strings representing patterns to exclude.
///   - `Pattern`: A string pattern to match against the last element of each entry.
///   - `Root`: The root directory to start the walk from.
///   - `Separator`: The separator used for splitting file paths.
///
/// # Returns
///
/// Returns a vector of processed entries.
/// Returns a vector of vectors, where each inner vector contains the components of a file path
/// split by the specified separator.
///
/// # Panics
///
/// This function will panic if it encounters an error while reading a directory entry.
///
/// # Example
///
/// ```
/// let options = Option {
///     Exclude: vec!["node_modules".to_string(), "target".to_string()],
///     Pattern: ".git".to_string(),
///     Root: ".".to_string(),
///     Separator: '/',
/// };
/// let paths = Fn(&options);
/// for path in paths {
///     println!("{:?}", path);
/// }
/// ```
pub fn Fn(Option { Exclude, Pattern, Root, Separator, .. }: &Option) -> Return {
	WalkDir::new(Root)
		.follow_links(false)
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 75ef690..02dbcc4 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -1,21 +1,37 @@
//! This module contains functions for parallel command execution in a binary context.

/// Executes a sequence of operations asynchronously in parallel based on the provided options.
/// Asynchronously processes entries to generate summaries and outputs the results.
///
/// This function performs the following steps:
/// 1. Filters and processes the provided entries based on the given pattern and separator.
/// 2. Spawns asynchronous tasks to generate summaries for each entry.
/// 3. Collects the results and outputs them.
///
/// # Arguments
///
/// * `Option` - A struct containing various options for execution, including:
///   - `Entry`: A collection of entries to process
///   - `Separator`: A separator used for joining entry parts
///   - `Pattern`: A pattern to match against the last element of each entry
///   - `Omit`: A collection of items to omit from processing
/// * `Option` - A struct containing the following fields:
///   - `Entry`: A vector of vectors, where each inner vector contains the components of a file path.
///   - `Separator`: A character used to join the components of the file path.
///   - `Pattern`: A string pattern to match against the last element of each entry.
///   - `Omit`: A vector of strings representing patterns to omit.
///
/// # Example
///
/// # Async
/// ```rust
/// let options = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.git".to_string()]],
///     Separator: '/',
///     Pattern: ".git".to_string(),
///     Omit: vec!["target".to_string()],
/// };
/// Fn(options).await;
/// ```
///
/// This function is asynchronous and returns a future.
/// # Errors
///
/// This function will log errors if it fails to generate summaries or send results.
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	futures::stream::iter(
		Entry
	let (Approval, mut Receipt) = tokio::sync::mpsc::unbounded_channel();

	let Entry = Entry
		.into_par_iter()
		.filter_map(|Entry| {
			Entry
@@ -23,29 +39,65 @@ pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
				.filter(|Last| *Last == &Pattern)
				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
		})
			.collect::<Vec<String>>(),
	)
	.map(|Entry| {
		.collect::<Vec<String>>();

	let Queue = FuturesUnordered::new();

	for Entry in Entry {
		let Omit = Omit.clone();
		let Approval = Approval.clone();

		async move {
		Queue.push(tokio::spawn(async move {
			match crate::Fn::Summary::Fn(
				&Entry,
				&crate::Struct::Summary::Difference::Struct { Omit },
			)
			.await
			{
				Ok(Summary) => Ok(Summary),
				Err(_Error) => Err(format!("Error generating summary for {}: {}", Entry, _Error)),
				Ok(Summary) => {
					if let Err(_Error) = Approval.send((Entry, Summary)) {
						eprintln!("Failed to send result: {}", _Error);
					}
				}
				Err(_Error) => eprintln!("Error generating summary for {}: {}", Entry, _Error),
			}
		}));
	}

	tokio::spawn(async move {
		Queue.collect::<Vec<_>>().await;
		drop(Approval);
	});

	let Output = DashMap::new();

	while let Some((Entry, Summary)) = Receipt.recv().await {
		for (_, (Difference, Message)) in Summary.into_iter() {
			Output
				.entry(Message + " in " + &Entry)
				.and_modify(|Existing: &mut HashSet<String>| {
					Existing.insert(Difference.clone());
				})
	.buffer_unordered(num_cpus::get())
	.collect::<Vec<_>>()
	.await;
				.or_insert_with(|| {
					let mut Set = HashSet::new();
					Set.insert(Difference);
					Set
				});
		}
	}

	Output.into_iter().for_each(|(Message, Difference)| {
		println!("{}", Message);

		for Difference in Difference {
			println!("{}", Difference);
		}
	});
}

use futures::stream::StreamExt;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use dashmap::DashMap;
use futures::stream::{FuturesUnordered, StreamExt};
use rayon::prelude::*;
use std::collections::HashSet;

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index a84435b..666e377 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -1,14 +1,33 @@
//! This module contains functions for sequential command execution in a binary context.

/// Executes a sequence of operations asynchronously based on the provided options.
/// Asynchronously processes entries to generate summaries and outputs the results sequentially.
///
/// This function performs the following steps:
/// 1. Filters and processes the provided entries based on the given pattern and separator.
/// 2. Spawns asynchronous tasks to generate summaries for each entry.
/// 3. Collects the results and outputs them.
///
/// # Arguments
///
/// * `Option` - A struct containing various options for execution.
/// * `Option` - A struct containing the following fields:
///   - `Entry`: A vector of vectors, where each inner vector contains the components of a file path.
///   - `Separator`: A character used to join the components of the file path.
///   - `Pattern`: A string pattern to match against the last element of each entry.
///   - `Omit`: A vector of strings representing patterns to omit.
///
/// # Example
///
/// ```rust
/// let options = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.git".to_string()]],
///     Separator: '/',
///     Pattern: ".git".to_string(),
///     Omit: vec!["target".to_string()],
/// };
/// Fn(options).await;
/// ```
///
/// # Async
/// # Errors
///
/// This function is asynchronous and returns a future.
/// This function will log errors if it fails to generate summaries or send results.
pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
	futures::future::join_all(
		Entry
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index f199ca3..5332dfa 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -1,29 +1,54 @@
//! This module provides functionality for generating summaries of git repositories.

/// Generates a summary for a given git repository entry.
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
/// * `Entry` - A string representing the repository path.
/// * `Option` - A reference to a struct containing summary options.
/// * `Entry` - A string slice representing the path to the git repository.
/// * `Option` - A reference to a struct containing options for generating the diff summary.
///
/// # Returns
///
/// Returns a Result containing () if successful, or a boxed dynamic error if an error occurs.
/// Returns a `Result` containing a DashMap with the generated summaries if successful, or a boxed `dyn std::error::Error` if an error occurs.
///
/// # Errors
///
/// This function will return an error if the repository cannot be opened or if there are issues
/// generating the summary.
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
) -> Result<(), Box<dyn std::error::Error>> {
) -> Result<DashMap<u64, (String, String)>, Box<dyn std::error::Error>> {
	let Summary = DashMap::new();

	match Repository::open(Entry) {
		Ok(Repository) => {
			let Name = Repository.tag_names(None)?;

			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();
			let mut Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			Tag.sort();
			Tag.dedup();

			let Head = Repository.head()?;

@@ -32,38 +57,34 @@ pub async fn Fn(
			let Last = Head.peel_to_commit()?.id().to_string();

			if Tag.is_empty() {
				println!("🗣️ Summary from first commit to last commit:");

				println!(
					"{}",
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option,)?
				);
				Insert::Fn(
					&Summary,
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option)?,
					format!("🗣️ Summary from first commit to last commit"),
				)
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					println!("🗣️ Summary from tag: {} to tag: {}:", Start, End);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?
					Insert::Fn(
						&Summary,
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?,
						format!("🗣️ Summary from {} to {}", Start, End),
					);
				}

				if let Some(Latest) = Tag.last() {
					println!("🗣️ Summary from first commit to latest tag: {}:", Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?
					Insert::Fn(
						&Summary,
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?,
						format!("🗣️ Summary from first commit to latest {}", Latest),
					);

					println!("🗣️ Summary from latest tag: {} to last commit:", Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Latest, &Last, Option)?
					Insert::Fn(
						&Summary,
						crate::Fn::Summary::Difference::Fn(&Repository, Latest, &Last, Option)?,
						format!("🗣️ Summary from latest {} to last commit", Latest),
					);
				}
			}
@@ -75,10 +96,12 @@ pub async fn Fn(
		}
	}

	Ok(())
	Ok(Summary)
}

use dashmap::DashMap;
use git2::Repository;

pub mod Difference;
pub mod First;
pub mod Insert;
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
index 544095b..4364657 100644
--- a/Source/Fn/Summary/Difference.rs
+++ b/Source/Fn/Summary/Difference.rs
@@ -1,83 +1,107 @@
//! This module provides functionality for generating difference summaries between git commits.

/// Generates a difference summary between two git commits.
/// Generates a diff summary between two commits in a git repository.
///
/// This function computes the differences between two specified commits in a git repository,
/// while filtering out changes to files that match a set of predefined patterns or user-specified
/// patterns to omit. The resulting diff is returned as a string.
///
/// # Arguments
///
/// * `Repository` - A reference to the git Repository.
/// * `Start` - The starting commit or reference.
/// * `End` - The ending commit or reference.
/// * `Option` - A reference to a struct containing difference options.
/// * `Repository` - A reference to the git repository.
/// * `Start` - A string slice representing the starting commit hash.
/// * `End` - A string slice representing the ending commit hash.
/// * `Option` - A reference to a struct containing options for generating the diff summary.
///
/// # Returns
///
/// Returns a Result containing a String with the difference summary if successful,
/// or a boxed dynamic error if an error occurs.
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

	Omit.extend(Option.Omit.iter().map(|Omit| Omit.as_str()));
	Common.extend(Option.Omit.iter().map(|Omit| Omit.as_str()));

	let Regex = Omit.into_par_iter().filter_map(|Omit| Regex::new(Omit).ok()).collect::<Vec<_>>();
	let Regex = regex::RegexSet::new(Common).expect("Cannot RegexSet.");

	let mut Options = git2::DiffOptions::new();

@@ -93,7 +117,7 @@ pub fn Fn(
	Options.show_binary(false);
	Options.force_binary(false);

	let mut Difference = String::new();
	let mut Output = String::new();

	Repository
		.diff_tree_to_tree(
@@ -102,21 +126,17 @@ pub fn Fn(
			Some(&mut Options),
		)?
		.print(git2::DiffFormat::Patch, |Delta, _, Line| {
			if !Regex.iter().any(|Omit| {
				Omit.is_match(&Delta.old_file().path().unwrap().display().to_string())
					|| Omit.is_match(&Delta.new_file().path().unwrap().display().to_string())
			}) {
			if !Regex.is_match(&Delta.old_file().path().unwrap().display().to_string())
				&& !Regex.is_match(&Delta.new_file().path().unwrap().display().to_string())
			{
				match std::str::from_utf8(Line.content()) {
					Ok(Line) => Difference.push_str(Line),
					Ok(Line) => Output.push_str(Line),
					Err(_) => (),
				}
			};
			}

			true
		})?;

	Ok(Difference)
	Ok(Output)
}

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
diff --git a/Source/Fn/Summary/First.rs b/Source/Fn/Summary/First.rs
index a0c3df6..f25a2ba 100644
--- a/Source/Fn/Summary/First.rs
+++ b/Source/Fn/Summary/First.rs
@@ -1,16 +1,33 @@
//! This module provides functionality for generating summaries of the first commit in a git repository.

/// Generates a summary of the first commit in a git repository.
/// Retrieves the OID of the first commit in the repository.
///
/// This function initializes a revwalk on the given repository, pushes the HEAD reference onto the
/// revwalk, and sets the sorting mode to topological and reverse. It then retrieves the first commit
/// in the revwalk, which corresponds to the first commit in the repository.
///
/// # Arguments
///
/// * `Repository` - A reference to the git Repository.
/// * `Option` - A reference to a struct containing summary options.
/// * `Repository` - A reference to the git repository.
///
/// # Returns
///
/// Returns a Result containing a String with the summary if successful,
/// or a boxed dynamic error if an error occurs.
/// Returns a `Result` containing the OID of the first commit if successful, or a `git2::Error` if an error occurs.
///
/// # Errors
///
/// This function will return an error if there are issues with initializing the revwalk, pushing the HEAD reference,
/// setting the sorting mode, or retrieving the first commit.
///
/// # Example
///
/// ```rust
/// let repo = git2::Repository::open("/path/to/repo").expect("Cannot open repository.");
/// let first_commit_oid = Fn(&repo).expect("Cannot retrieve first commit.");
/// println!("First commit OID: {}", first_commit_oid);
/// ```
///
/// # Panics
///
/// This function does not panic.
pub fn Fn(Repository: &Repository) -> Result<Oid, git2::Error> {
	let mut Walk = Repository.revwalk()?;
	Walk.push_head()?;
diff --git a/Source/Fn/Summary/Insert.rs b/Source/Fn/Summary/Insert.rs
new file mode 100644
index 0000000..9ae2a0b
--- /dev/null
+++ b/Source/Fn/Summary/Insert.rs
@@ -0,0 +1,35 @@
/// Inserts a difference summary into the provided DashMap.
///
/// This function computes the hash of the given difference string and inserts it into the DashMap
/// along with the associated message. The hash is used as the key, and the value is a tuple
/// containing the difference string and the message.
///
/// # Arguments
///
/// * `Summary` - A reference to a DashMap where the difference summary will be inserted.
/// * `Difference` - A string representing the difference to be summarized.
/// * `Message` - A string representing the message associated with the difference.
///
/// # Example
///
/// ```rust
/// let summary = DashMap::new();
/// let difference = "diff content".to_string();
/// let message = "Summary message".to_string();
/// Fn(&summary, difference, message);
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// This function does not return errors.
pub fn Fn(Summary: &DashMap<u64, (String, String)>, Difference: String, Message: String) {
	Summary.insert(Hash::Fn(&Difference), (Difference, Message));
}

use dashmap::DashMap;

pub mod Hash;
diff --git a/Source/Fn/Summary/Insert/Hash.rs b/Source/Fn/Summary/Insert/Hash.rs
new file mode 100644
index 0000000..abf2d05
--- /dev/null
+++ b/Source/Fn/Summary/Insert/Hash.rs
@@ -0,0 +1,41 @@
/// Computes the hash of the given input using the `DefaultHasher`.
///
/// This function takes any input that implements the `Hash` trait and computes its hash value
/// using the `DefaultHasher` from the standard library. The resulting hash is returned as a `u64`.
///
/// # Arguments
///
/// * `Input` - A reference to the input value to be hashed. The input must implement the `Hash` trait.
///
/// # Returns
///
/// Returns a `u64` representing the hash value of the input.
///
/// # Example
///
/// ```rust
/// use std::collections::hash_map::DefaultHasher;
/// use std::hash::{Hash, Hasher};
///
/// let value = "example";
/// let hash = Fn(&value);
/// println!("Hash value: {}", hash);
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// This function does not return errors.
pub fn Fn<T: Hash>(Input: &T) -> u64 {
	let mut Output = DefaultHasher::new();
	Input.hash(&mut Output);
	Output.finish()
}

use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
};
diff --git a/Source/Library.rs b/Source/Library.rs
index 7b297ec..bd54253 100644
--- a/Source/Library.rs
+++ b/Source/Library.rs
@@ -1,15 +1,25 @@
//! The main entry point for the Summary application.

#![allow(non_snake_case)]

/// The main function that initializes and runs the `Summary` application.
///
/// # Errors
///
/// This function will return an error if there are issues parsing arguments
/// or executing the requested commands.
#[allow(dead_code)]
#[tokio::main]
/// The main entry point for the application.
///
/// This function initializes the command structure and executes the asynchronous function
/// defined within it. The function is marked with the `#[tokio::main]` attribute to enable
/// asynchronous execution using the Tokio runtime.
///
/// # Panics
///
/// This function does not panic.
///
/// # Example
///
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     (Struct::Binary::Command::Struct::Fn().Fn)().await
/// }
/// ```
async fn main() {
	(Struct::Binary::Command::Struct::Fn().Fn)().await
}
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
index 7473837..2c5edaf 100644
--- a/Source/Struct/Binary/Command.rs
+++ b/Source/Struct/Binary/Command.rs
@@ -1,19 +1,21 @@
//! This module defines the main command structure and its implementation for the binary command execution.

/// Represents the main command structure for binary command execution.
/// Represents the structure for binary command execution.
///
/// This struct holds various fields related to the command execution, including the separator for file paths
/// and a function to execute the command asynchronously.
pub struct Struct {
	/// The separator used for file paths.
	pub Separator: Option::Separator,

	/// A boxed function that returns a pinned future representing the command execution.
	/// A boxed asynchronous function that returns a pinned future.
	pub Fn: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static>,
}

impl Struct {
	/// Creates a new instance of the Struct.
	///
	/// This function initializes the Struct with the system's main separator
	/// and a boxed async function that handles command execution.
	/// This function initializes the Struct with the default file path separator and an asynchronous function
	/// that executes the command based on the provided options. The function determines whether to execute
	/// the command in parallel or sequentially based on the `Parallel` flag in the options.
	///
	/// # Returns
	///
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
index ee24305..ff0469d 100644
--- a/Source/Struct/Binary/Command/Entry.rs
+++ b/Source/Struct/Binary/Command/Entry.rs
@@ -1,20 +1,29 @@
//! This module defines structures and functions related to binary command entries.

/// Represents the structure for binary command entries.
///
/// This struct holds various fields related to the command entries, including the entry paths,
/// parallel execution flag, pattern to match, separator for file paths, and omit patterns.
pub struct Struct {
	/// A vector of vectors, where each inner vector contains the components of a file path.
	pub Entry: Type,

	/// A flag indicating whether to execute commands in parallel.
	pub Parallel: Parallel,

	/// A string pattern to match against the last element of each entry.
	pub Pattern: Pattern,

	/// The separator used for file paths.
	pub Separator: Separator,

	/// A vector of strings representing patterns to omit.
	pub Omit: Omit,
}

impl Struct {
	/// Creates a new Struct instance from the given options.
	/// Creates a new instance of the Struct.
	///
	/// This function initializes the Struct with the provided options, generating the entry paths
	/// and cloning the omit patterns, parallel flag, pattern, and separator from the options.
	///
	/// # Arguments
	///
@@ -38,4 +47,5 @@ use crate::Struct::Binary::Command::Option::{
	Omit, Parallel, Pattern, Separator, Struct as Option,
};

/// Type alias for a vector of vectors, where each inner vector contains the components of a file path.
pub type Type = Vec<Vec<String>>;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index 3f069ff..2a8f091 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
@@ -1,26 +1,36 @@
//! This module defines structures and functions related to binary command options.

/// Represents the structure for binary command options.
/// Represents the options for binary command execution.
///
/// This struct holds various fields related to the command options, including exclude patterns,
/// omit patterns, parallel execution flag, pattern to match, root directory, and separator for file paths.
pub struct Struct {
	/// A vector of strings representing patterns to exclude.
	pub Exclude: Vec<String>,

	/// A vector of strings representing patterns to omit.
	pub Omit: Vec<String>,

	/// A flag indicating whether to execute commands in parallel.
	pub Parallel: Parallel,

	/// A string pattern to match against the last element of each entry.
	pub Pattern: Pattern,

	/// The root directory to start the walk from.
	pub Root: String,

	/// The separator used for file paths.
	pub Separator: Separator,
}

impl Struct {
	/// Creates a new Struct instance from the given options.
	/// Creates a new instance of the Struct.
	///
	/// This function initializes the Struct with the provided options, generating the exclude patterns,
	/// omit patterns, parallel flag, pattern, root directory, and separator from the options.
	///
	/// # Arguments
	///
	/// * `Option` - An Option struct containing initialization parameters.
	/// * `Option` - A reference to an Option struct containing initialization parameters.
	///
	/// # Returns
	///
@@ -48,8 +58,17 @@ impl Struct {

use crate::{Fn::Binary::Command::Fn as Command, Struct::Binary::Command::Struct as Option};

/// Type alias for a vector of strings representing command options.
pub type Command = Vec<String>;

/// Type alias for a boolean flag indicating parallel execution.
pub type Parallel = bool;

/// Type alias for a string pattern to match.
pub type Pattern = String;

/// Type alias for a character used as a separator for file paths.
pub type Separator = char;

/// Type alias for a vector of strings representing patterns to omit.
pub type Omit = Vec<String>;
diff --git a/Source/Struct/Summary/Difference.rs b/Source/Struct/Summary/Difference.rs
index bb7ec8e..efe9627 100644
--- a/Source/Struct/Summary/Difference.rs
+++ b/Source/Struct/Summary/Difference.rs
@@ -1,6 +1,19 @@
//! This module defines structures related to git diff summary options.

/// Represents the options for generating a git diff summary.
/// Represents a structure containing omit patterns.
///
/// This struct holds a vector of strings representing patterns to omit.
///
/// # Fields
///
/// * `Omit` - A vector of strings representing patterns to omit.
///
/// # Example
///
/// ```rust
/// let omit_patterns = Struct {
///     Omit: vec!["pattern1".to_string(), "pattern2".to_string()],
/// };
/// ```
pub struct Struct {
	/// A vector of strings representing patterns to omit.
	pub Omit: Vec<String>,
}

🗣️ Summary from Summary/v0.0.8 to Summary/v0.0.9 in .
diff --git a/Cargo.toml b/Cargo.toml
index 13e711a..0f0c9c7 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -36,5 +36,13 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.8"
version = "0.0.9"
edition = "2021"
include = [
	"Source/**/*",
	"LICENSE",
	"README.md",
	"CHANGELOG.md",
	"build.rs",
	"Cargo.toml",
]

🗣️ Summary from Summary/v0.0.9 to last commit in .
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index 5332dfa..240a227 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -78,13 +78,13 @@ pub async fn Fn(
					Insert::Fn(
						&Summary,
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?,
						format!("🗣️ Summary from first commit to latest {}", Latest),
						format!("🗣️ Summary from first commit to {}", Latest),
					);

					Insert::Fn(
						&Summary,
						crate::Fn::Summary::Difference::Fn(&Repository, Latest, &Last, Option)?,
						format!("🗣️ Summary from latest {} to last commit", Latest),
						format!("🗣️ Summary from {} to last commit", Latest),
					);
				}
			}

🗣️ Summary from first commit to Summary/v0.0.9 in .
diff --git a/.gitignore b/.gitignore
index 34f0334..619d2a9 100644
--- a/.gitignore
+++ b/.gitignore
@@ -5,5 +5,5 @@

!/Target/release/*.deb
!/Target/release/*.exe
!/Target/release/PRun
!/Target/release/Run
!/Target/release/PSummary
!/Target/release/Summary
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
index 3e65019..0f0c9c7 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -11,13 +11,15 @@ clap = { features = ["derive"], version = "4.5.11" }
walkdir = "2.5.0"
futures = "0.3.30"
rayon = "1.10.0"
tokio = { version = "1.39.1", features = ["full"] }
tokio = { version = "1.39.2", features = ["full"] }
git2 = { version = "0.19.0" }
num_cpus = "1.16.0"
regex = "1.10.5"
dashmap = "6.0.1"

[build-dependencies]
serde = { version = "1.0.204", features = ["derive"] }
toml = "0.8.16"
toml = "0.8.17"

[lib]
crate-type = ["staticlib", "cdylib", "rlib"]
@@ -34,5 +36,13 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.1"
version = "0.0.9"
edition = "2021"
include = [
	"Source/**/*",
	"LICENSE",
	"README.md",
	"CHANGELOG.md",
	"build.rs",
	"Cargo.toml",
]
diff --git a/README.md b/README.md
index 871c006..1bf06be 100644
--- a/README.md
+++ b/README.md
@@ -1,11 +1,40 @@
# 🗣️ [Summary] —

`Summary` is a command-line tool that executes commands in multiple directories
simultaneously. It leverages parallel processing and concurrent `I/O` to
efficiently run tasks across directories.
[Summary] is a powerful command-line tool designed for efficient `Git`
repository analysis and summarization. It offers both sequential and parallel
processing capabilities, along with flexible file filtering options.

[Summary]: HTTPS://crates.io/crates/psummary

```sh
Summary -P > Summary.md
```

[Summary] will now generate the following [Summary.md](./Summary.md) for all the
commits and tags between the first and the last commit.

## Features

-   Customizable file pattern matching.
-   Diff generation between `Git` tags.
-   Directory traversal and file filtering.
-   Exclusion of specified files or directories.
-   `Git` repository analysis.
-   Integration with [Pieces OS] for enhanced functionality.
-   Parallel and sequential processing modes.

## [Pieces OS] Integration

The [Summary] CLI supports [Pieces OS], allowing it to:

-   Generate comprehensive diff logs and release notes automatically.
-   Provide AI-driven code analysis and insights.
-   Offer improved context-aware processing of repository changes.
-   Seamlessly interact with other [Pieces OS]-compatible development tools.

By leveraging [Pieces OS], [Summary] can tap into a broader ecosystem of development
tools and services, significantly expanding its capabilities beyond basic file processing.

## Installation

```sh
@@ -14,54 +43,130 @@ cargo install psummary

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

The [Summary] tool can be used with various options:

#### --Exclude or -E:

Exclude certain files or directories.

Default is:

```sh
Summary
Summary -P -E node_modules
```

This command will fetch from upstream for all `.git` repositories inside the
current directory. It essentially replaces the following command:
#### --Omit or -O:

Specify regex patterns to omit files from processing.

Default is:

```sh
find -iname .git -type d -execdir git fetch upstream \;
Summary -P \
	--Omit "(?i)documentation" \
	--Omit "(?i)target" \
	--Omit "(?i)changelog\.md$" \
	--Omit "(?i)summary\.md$"
```

## Options
#### --Parallel or -P:

Run processing in parallel.

Default is:

```sh
Summary
```

#### --Pattern:

Specify a custom pattern for matching.

Default is:

```sh
Summary -P --Pattern .git
```

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):
Set the current working directory to a different folder.

Default is:

```sh
Summary -R D:\Developer .git git fetch upstream
Summary -P --Root .
```

#### --Parallel or -P:
For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
flags and configuration options. [Pieces OS]

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
Summary -P -E "node_modules target dist vendor"
```

Omit specific file patterns:

```sh
Summary -P -O "\.md$" -O "\.txt$"
```

## Dependencies

`Summary` relies on several Rust crates to provide its functionality:
[Summary] relies on several Rust crates to provide its functionality:

-   `clap` - For parsing command-line arguments.
-   `futures` - For asynchronous programming abstractions.
-   `git2` - For `Git` repository operations.
-   `num_cpus` - For determining the number of CPUs for parallel processing.
-   `rayon` - For parallel processing.
-   `regex` - For pattern matching and text manipulation.
-   `tokio` - For asynchronous runtime.
-   `walkdir` - For efficient filesystem traversal.

-   `clap` - Parses command-line arguments
-   `rayon` - Enables parallel processing
-   `tokio` - Provides an asynchronous runtime
-   `walkdir` - Facilitates efficient filesystem traversal
[Pieces OS] For extended functionality and system integration.

[Summary]: HTTPS://crates.io/crates/psummary
[Pieces OS]: HTTPS://Pieces.App

## Changelog

diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index e69de29..4686859 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -0,0 +1,104 @@
/// Creates and returns the command-line argument matches for the `Summary` application.
///
/// This function sets up the command-line interface using the `clap` crate, defining various
/// arguments and their properties such as short and long names, help messages, default values,
/// and whether they are required.
///
/// # Returns
///
/// Returns an `ArgMatches` instance containing the parsed command-line arguments.
///
/// # Arguments
///
/// * `Exclude` - An optional argument to specify patterns to exclude. Default is "node_modules".
/// * `Omit` - An optional argument to specify patterns to omit. Default values are:
///   - "(?i)documentation"
///   - "(?i)target"
///   - "(?i)changelog\.md$"
///   - "(?i)summary\.md$"
/// * `Parallel` - An optional flag to enable parallel processing.
/// * `Pattern` - An optional argument to specify a pattern to match. Default is ".git".
/// * `Root` - An optional argument to specify the root directory. Default is ".".
///
/// # Example
///
/// ```rust
/// let matches = Fn();
/// let exclude = matches.value_of("Exclude").unwrap_or("node_modules");
/// let omit = matches.values_of("Omit").unwrap_or_default().collect::<Vec<_>>();
/// let parallel = matches.is_present("Parallel");
/// let pattern = matches.value_of("Pattern").unwrap_or(".git");
/// let root = matches.value_of("Root").unwrap_or(".");
/// ```
///
/// # Errors
///
/// This function will panic if there are issues with the argument definitions or parsing.
pub fn Fn() -> ArgMatches {
	Command::new("Summary")
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
				.default_values([
					"(?i)documentation",
					"(?i)target",
					r"(?i)changelog\.md$",
					r"(?i)summary\.md$",
				]),
		)
		.arg(
			Arg::new("Parallel")
				.short('P')
				.long("Parallel")
				.action(SetTrue)
				.display_order(2)
				.value_name("PARALLEL")
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
				.long("Root")
				.display_order(3)
				.value_name("ROOT")
				.required(false)
				.help("📂 Root —")
				.default_value("."),
		)
		.get_matches()
}

use clap::{Arg, ArgAction::SetTrue, ArgMatches, Command};

pub mod Entry;
pub mod Parallel;
pub mod Sequential;
diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
new file mode 100644
index 0000000..8918e6f
--- /dev/null
+++ b/Source/Fn/Binary/Command/Entry.rs
@@ -0,0 +1,59 @@
/// Generates a list of file paths from the specified root directory, excluding paths that match
/// any of the specified exclude patterns.
///
/// # Arguments
///
/// * `Option` - A reference to an `Option` struct containing the following fields:
///   - `Exclude`: A vector of strings representing patterns to exclude.
///   - `Pattern`: A string pattern to match against the last element of each entry.
///   - `Root`: The root directory to start the walk from.
///   - `Separator`: The separator used for splitting file paths.
///
/// # Returns
///
/// Returns a vector of vectors, where each inner vector contains the components of a file path
/// split by the specified separator.
///
/// # Panics
///
/// This function will panic if it encounters an error while reading a directory entry.
///
/// # Example
///
/// ```
/// let options = Option {
///     Exclude: vec!["node_modules".to_string(), "target".to_string()],
///     Pattern: ".git".to_string(),
///     Root: ".".to_string(),
///     Separator: '/',
/// };
/// let paths = Fn(&options);
/// for path in paths {
///     println!("{:?}", path);
/// }
/// ```
pub fn Fn(Option { Exclude, Pattern, Root, Separator, .. }: &Option) -> Return {
	WalkDir::new(Root)
		.follow_links(false)
		.into_iter()
		.filter_map(|Entry| {
			let Path = Entry.expect("Cannot Entry.").path().display().to_string();

			// TODO: Separate this into Entry/Exclude.rs
			if !Exclude
				.clone()
				.into_iter()
				.filter(|Exclude| *Pattern != *Exclude)
				.any(|Exclude| Path.contains(&Exclude))
			{
				Some(Path.split(*Separator).map(|Entry| Entry.to_string()).collect())
			} else {
				None
			}
		})
		.collect::<Vec<_>>()
}

use crate::Struct::Binary::Command::{Entry::Type as Return, Option::Struct as Option};

use walkdir::WalkDir;
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
new file mode 100644
index 0000000..02dbcc4
--- /dev/null
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -0,0 +1,103 @@
/// Asynchronously processes entries to generate summaries and outputs the results.
///
/// This function performs the following steps:
/// 1. Filters and processes the provided entries based on the given pattern and separator.
/// 2. Spawns asynchronous tasks to generate summaries for each entry.
/// 3. Collects the results and outputs them.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   - `Entry`: A vector of vectors, where each inner vector contains the components of a file path.
///   - `Separator`: A character used to join the components of the file path.
///   - `Pattern`: A string pattern to match against the last element of each entry.
///   - `Omit`: A vector of strings representing patterns to omit.
///
/// # Example
///
/// ```rust
/// let options = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.git".to_string()]],
///     Separator: '/',
///     Pattern: ".git".to_string(),
///     Omit: vec!["target".to_string()],
/// };
/// Fn(options).await;
/// ```
///
/// # Errors
///
/// This function will log errors if it fails to generate summaries or send results.
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	let (Approval, mut Receipt) = tokio::sync::mpsc::unbounded_channel();

	let Entry = Entry
		.into_par_iter()
		.filter_map(|Entry| {
			Entry
				.last()
				.filter(|Last| *Last == &Pattern)
				.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
		})
		.collect::<Vec<String>>();

	let Queue = FuturesUnordered::new();

	for Entry in Entry {
		let Omit = Omit.clone();
		let Approval = Approval.clone();

		Queue.push(tokio::spawn(async move {
			match crate::Fn::Summary::Fn(
				&Entry,
				&crate::Struct::Summary::Difference::Struct { Omit },
			)
			.await
			{
				Ok(Summary) => {
					if let Err(_Error) = Approval.send((Entry, Summary)) {
						eprintln!("Failed to send result: {}", _Error);
					}
				}
				Err(_Error) => eprintln!("Error generating summary for {}: {}", Entry, _Error),
			}
		}));
	}

	tokio::spawn(async move {
		Queue.collect::<Vec<_>>().await;
		drop(Approval);
	});

	let Output = DashMap::new();

	while let Some((Entry, Summary)) = Receipt.recv().await {
		for (_, (Difference, Message)) in Summary.into_iter() {
			Output
				.entry(Message + " in " + &Entry)
				.and_modify(|Existing: &mut HashSet<String>| {
					Existing.insert(Difference.clone());
				})
				.or_insert_with(|| {
					let mut Set = HashSet::new();
					Set.insert(Difference);
					Set
				});
		}
	}

	Output.into_iter().for_each(|(Message, Difference)| {
		println!("{}", Message);

		for Difference in Difference {
			println!("{}", Difference);
		}
	});
}

use dashmap::DashMap;
use futures::stream::{FuturesUnordered, StreamExt};
use rayon::prelude::*;
use std::collections::HashSet;

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
new file mode 100644
index 0000000..666e377
--- /dev/null
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -0,0 +1,63 @@
/// Asynchronously processes entries to generate summaries and outputs the results sequentially.
///
/// This function performs the following steps:
/// 1. Filters and processes the provided entries based on the given pattern and separator.
/// 2. Spawns asynchronous tasks to generate summaries for each entry.
/// 3. Collects the results and outputs them.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   - `Entry`: A vector of vectors, where each inner vector contains the components of a file path.
///   - `Separator`: A character used to join the components of the file path.
///   - `Pattern`: A string pattern to match against the last element of each entry.
///   - `Omit`: A vector of strings representing patterns to omit.
///
/// # Example
///
/// ```rust
/// let options = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.git".to_string()]],
///     Separator: '/',
///     Pattern: ".git".to_string(),
///     Omit: vec!["target".to_string()],
/// };
/// Fn(options).await;
/// ```
///
/// # Errors
///
/// This function will log errors if it fails to generate summaries or send results.
pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
	futures::future::join_all(
		Entry
			.into_iter()
			.filter_map(|Entry| {
				Entry
					.last()
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
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
diff --git a/Source/Fn/mod.rs b/Source/Fn/mod.rs
index a56e8ce..4ca5f2b 100644
--- a/Source/Fn/mod.rs
+++ b/Source/Fn/mod.rs
@@ -1 +1,2 @@
pub mod Binary;
pub mod Summary;
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
new file mode 100644
index 0000000..5332dfa
--- /dev/null
+++ b/Source/Fn/Summary.rs
@@ -0,0 +1,107 @@
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
/// * `Entry` - A string slice representing the path to the git repository.
/// * `Option` - A reference to a struct containing options for generating the diff summary.
///
/// # Returns
///
/// Returns a `Result` containing a DashMap with the generated summaries if successful, or a boxed `dyn std::error::Error` if an error occurs.
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

	match Repository::open(Entry) {
		Ok(Repository) => {
			let Name = Repository.tag_names(None)?;

			let mut Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			Tag.sort();
			Tag.dedup();

			let Head = Repository.head()?;

			let First = Repository.find_commit(First::Fn(&Repository)?)?.id().to_string();

			let Last = Head.peel_to_commit()?.id().to_string();

			if Tag.is_empty() {
				Insert::Fn(
					&Summary,
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option)?,
					format!("🗣️ Summary from first commit to last commit"),
				)
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					Insert::Fn(
						&Summary,
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?,
						format!("🗣️ Summary from {} to {}", Start, End),
					);
				}

				if let Some(Latest) = Tag.last() {
					Insert::Fn(
						&Summary,
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?,
						format!("🗣️ Summary from first commit to latest {}", Latest),
					);

					Insert::Fn(
						&Summary,
						crate::Fn::Summary::Difference::Fn(&Repository, Latest, &Last, Option)?,
						format!("🗣️ Summary from latest {} to last commit", Latest),
					);
				}
			}
		}
		Err(_Error) => {
			println!("Cannot Repository: {}", _Error);

			return Err(_Error.into());
		}
	}

	Ok(Summary)
}

use dashmap::DashMap;
use git2::Repository;

pub mod Difference;
pub mod First;
pub mod Insert;
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
new file mode 100644
index 0000000..4364657
--- /dev/null
+++ b/Source/Fn/Summary/Difference.rs
@@ -0,0 +1,142 @@
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
diff --git a/Source/Fn/Summary/First.rs b/Source/Fn/Summary/First.rs
new file mode 100644
index 0000000..f25a2ba
--- /dev/null
+++ b/Source/Fn/Summary/First.rs
@@ -0,0 +1,43 @@
/// Retrieves the OID of the first commit in the repository.
///
/// This function initializes a revwalk on the given repository, pushes the HEAD reference onto the
/// revwalk, and sets the sorting mode to topological and reverse. It then retrieves the first commit
/// in the revwalk, which corresponds to the first commit in the repository.
///
/// # Arguments
///
/// * `Repository` - A reference to the git repository.
///
/// # Returns
///
/// Returns a `Result` containing the OID of the first commit if successful, or a `git2::Error` if an error occurs.
///
/// # Errors
///
/// This function will return an error if there are issues with initializing the revwalk, pushing the HEAD reference,
/// setting the sorting mode, or retrieving the first commit.
///
/// # Example
///
/// ```rust
/// let repo = git2::Repository::open("/path/to/repo").expect("Cannot open repository.");
/// let first_commit_oid = Fn(&repo).expect("Cannot retrieve first commit.");
/// println!("First commit OID: {}", first_commit_oid);
/// ```
///
/// # Panics
///
/// This function does not panic.
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
diff --git a/Source/Fn/Summary/Insert.rs b/Source/Fn/Summary/Insert.rs
new file mode 100644
index 0000000..9ae2a0b
--- /dev/null
+++ b/Source/Fn/Summary/Insert.rs
@@ -0,0 +1,35 @@
/// Inserts a difference summary into the provided DashMap.
///
/// This function computes the hash of the given difference string and inserts it into the DashMap
/// along with the associated message. The hash is used as the key, and the value is a tuple
/// containing the difference string and the message.
///
/// # Arguments
///
/// * `Summary` - A reference to a DashMap where the difference summary will be inserted.
/// * `Difference` - A string representing the difference to be summarized.
/// * `Message` - A string representing the message associated with the difference.
///
/// # Example
///
/// ```rust
/// let summary = DashMap::new();
/// let difference = "diff content".to_string();
/// let message = "Summary message".to_string();
/// Fn(&summary, difference, message);
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// This function does not return errors.
pub fn Fn(Summary: &DashMap<u64, (String, String)>, Difference: String, Message: String) {
	Summary.insert(Hash::Fn(&Difference), (Difference, Message));
}

use dashmap::DashMap;

pub mod Hash;
diff --git a/Source/Fn/Summary/Insert/Hash.rs b/Source/Fn/Summary/Insert/Hash.rs
new file mode 100644
index 0000000..abf2d05
--- /dev/null
+++ b/Source/Fn/Summary/Insert/Hash.rs
@@ -0,0 +1,41 @@
/// Computes the hash of the given input using the `DefaultHasher`.
///
/// This function takes any input that implements the `Hash` trait and computes its hash value
/// using the `DefaultHasher` from the standard library. The resulting hash is returned as a `u64`.
///
/// # Arguments
///
/// * `Input` - A reference to the input value to be hashed. The input must implement the `Hash` trait.
///
/// # Returns
///
/// Returns a `u64` representing the hash value of the input.
///
/// # Example
///
/// ```rust
/// use std::collections::hash_map::DefaultHasher;
/// use std::hash::{Hash, Hasher};
///
/// let value = "example";
/// let hash = Fn(&value);
/// println!("Hash value: {}", hash);
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Errors
///
/// This function does not return errors.
pub fn Fn<T: Hash>(Input: &T) -> u64 {
	let mut Output = DefaultHasher::new();
	Input.hash(&mut Output);
	Output.finish()
}

use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
};
diff --git a/Source/Library.rs b/Source/Library.rs
index 62cfaff..bd54253 100644
--- a/Source/Library.rs
+++ b/Source/Library.rs
@@ -1,10 +1,28 @@
#![allow(non_snake_case)]

mod Fn;
mod Struct;

#[allow(dead_code)]
#[tokio::main]
/// The main entry point for the application.
///
/// This function initializes the command structure and executes the asynchronous function
/// defined within it. The function is marked with the `#[tokio::main]` attribute to enable
/// asynchronous execution using the Tokio runtime.
///
/// # Panics
///
/// This function does not panic.
///
/// # Example
///
/// ```rust
/// #[tokio::main]
/// async fn main() {
///     (Struct::Binary::Command::Struct::Fn().Fn)().await
/// }
/// ```
async fn main() {
	(Struct::Binary::Command::Struct::Fn().Fn)().await
}

pub mod Fn;
pub mod Struct;
diff --git a/Source/main.rs b/Source/main.rs
deleted file mode 100644
index 3f3ccf9..0000000
--- a/Source/main.rs
+++ /dev/null
@@ -1,158 +0,0 @@
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
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
new file mode 100644
index 0000000..2c5edaf
--- /dev/null
+++ b/Source/Struct/Binary/Command.rs
@@ -0,0 +1,50 @@
/// Represents the structure for binary command execution.
///
/// This struct holds various fields related to the command execution, including the separator for file paths
/// and a function to execute the command asynchronously.
pub struct Struct {
	/// The separator used for file paths.
	pub Separator: Option::Separator,

	/// A boxed asynchronous function that returns a pinned future.
	pub Fn: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static>,
}

impl Struct {
	/// Creates a new instance of the Struct.
	///
	/// This function initializes the Struct with the default file path separator and an asynchronous function
	/// that executes the command based on the provided options. The function determines whether to execute
	/// the command in parallel or sequentially based on the `Parallel` flag in the options.
	///
	/// # Returns
	///
	/// Returns a new instance of Struct.
	pub fn Fn() -> Self {
		Self {
			Separator: std::path::MAIN_SEPARATOR,
			Fn: Box::new(|| {
				Box::pin(async move {
					let Option = Entry::Struct::Fn(&Option::Struct::Fn(Struct::Fn()));

					match Option.Parallel {
						true => {
							Parallel::Fn(Option).await;
						}
						false => {
							Sequential::Fn(Option).await;
						}
					};
				})
			}),
		}
	}
}

use futures::Future;
use std::pin::Pin;

pub mod Entry;
pub mod Option;

use crate::Fn::Binary::Command::{Parallel, Sequential};
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
new file mode 100644
index 0000000..ff0469d
--- /dev/null
+++ b/Source/Struct/Binary/Command/Entry.rs
@@ -0,0 +1,51 @@
/// Represents the structure for binary command entries.
///
/// This struct holds various fields related to the command entries, including the entry paths,
/// parallel execution flag, pattern to match, separator for file paths, and omit patterns.
pub struct Struct {
	/// A vector of vectors, where each inner vector contains the components of a file path.
	pub Entry: Type,

	/// A flag indicating whether to execute commands in parallel.
	pub Parallel: Parallel,

	/// A string pattern to match against the last element of each entry.
	pub Pattern: Pattern,

	/// The separator used for file paths.
	pub Separator: Separator,

	/// A vector of strings representing patterns to omit.
	pub Omit: Omit,
}

impl Struct {
	/// Creates a new instance of the Struct.
	///
	/// This function initializes the Struct with the provided options, generating the entry paths
	/// and cloning the omit patterns, parallel flag, pattern, and separator from the options.
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
			Omit: Option.Omit.clone(),
			Parallel: Option.Parallel,
			Pattern: Option.Pattern.clone(),
			Separator: Option.Separator,
		}
	}
}

use crate::Struct::Binary::Command::Option::{
	Omit, Parallel, Pattern, Separator, Struct as Option,
};

/// Type alias for a vector of vectors, where each inner vector contains the components of a file path.
pub type Type = Vec<Vec<String>>;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
new file mode 100644
index 0000000..2a8f091
--- /dev/null
+++ b/Source/Struct/Binary/Command/Option.rs
@@ -0,0 +1,74 @@
/// Represents the options for binary command execution.
///
/// This struct holds various fields related to the command options, including exclude patterns,
/// omit patterns, parallel execution flag, pattern to match, root directory, and separator for file paths.
pub struct Struct {
	/// A vector of strings representing patterns to exclude.
	pub Exclude: Vec<String>,

	/// A vector of strings representing patterns to omit.
	pub Omit: Vec<String>,

	/// A flag indicating whether to execute commands in parallel.
	pub Parallel: Parallel,

	/// A string pattern to match against the last element of each entry.
	pub Pattern: Pattern,

	/// The root directory to start the walk from.
	pub Root: String,

	/// The separator used for file paths.
	pub Separator: Separator,
}

impl Struct {
	/// Creates a new instance of the Struct.
	///
	/// This function initializes the Struct with the provided options, generating the exclude patterns,
	/// omit patterns, parallel flag, pattern, root directory, and separator from the options.
	///
	/// # Arguments
	///
	/// * `Option` - A reference to an Option struct containing initialization parameters.
	///
	/// # Returns
	///
	/// Returns a new instance of Struct.
	pub fn Fn(Option { Separator, .. }: Option) -> Self {
		Self {
			Exclude: Command()
				.get_one::<String>("Exclude")
				.expect("Cannot Exclude.")
				.split(" ")
				.map(|Exclude| Exclude.to_string())
				.collect::<Vec<_>>(),
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

use crate::{Fn::Binary::Command::Fn as Command, Struct::Binary::Command::Struct as Option};

/// Type alias for a vector of strings representing command options.
pub type Command = Vec<String>;

/// Type alias for a boolean flag indicating parallel execution.
pub type Parallel = bool;

/// Type alias for a string pattern to match.
pub type Pattern = String;

/// Type alias for a character used as a separator for file paths.
pub type Separator = char;

/// Type alias for a vector of strings representing patterns to omit.
pub type Omit = Vec<String>;
diff --git a/Source/Struct/Binary/mod.rs b/Source/Struct/Binary/mod.rs
new file mode 100644
index 0000000..9da7843
--- /dev/null
+++ b/Source/Struct/Binary/mod.rs
@@ -0,0 +1 @@
pub mod Command;
diff --git a/Source/Struct/mod.rs b/Source/Struct/mod.rs
new file mode 100644
index 0000000..4ca5f2b
--- /dev/null
+++ b/Source/Struct/mod.rs
@@ -0,0 +1,2 @@
pub mod Binary;
pub mod Summary;
diff --git a/Source/Struct/Summary/Difference.rs b/Source/Struct/Summary/Difference.rs
new file mode 100644
index 0000000..efe9627
--- /dev/null
+++ b/Source/Struct/Summary/Difference.rs
@@ -0,0 +1,19 @@
/// Represents a structure containing omit patterns.
///
/// This struct holds a vector of strings representing patterns to omit.
///
/// # Fields
///
/// * `Omit` - A vector of strings representing patterns to omit.
///
/// # Example
///
/// ```rust
/// let omit_patterns = Struct {
///     Omit: vec!["pattern1".to_string(), "pattern2".to_string()],
/// };
/// ```
pub struct Struct {
	/// A vector of strings representing patterns to omit.
	pub Omit: Vec<String>,
}
diff --git a/Source/Struct/Summary/mod.rs b/Source/Struct/Summary/mod.rs
new file mode 100644
index 0000000..7241509
--- /dev/null
+++ b/Source/Struct/Summary/mod.rs
@@ -0,0 +1 @@
pub mod Difference;

