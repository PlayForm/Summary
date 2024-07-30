🗣️ Summary from tag: Summary/v0.0.1 to tag: Summary/v0.0.2:
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

🗣️ Summary from tag: Summary/v0.0.2 to tag: Summary/v0.0.3:
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

🗣️ Summary from tag: Summary/v0.0.3 to tag: Summary/v0.0.4:
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

🗣️ Summary from tag: Summary/v0.0.4 to tag: Summary/v0.0.5:
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

🗣️ Summary from tag: Summary/v0.0.5 to tag: Summary/v0.0.6:
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
diff --git a/Summary.md b/Summary.md
new file mode 100644
index 0000000..f600aa6
--- /dev/null
+++ b/Summary.md
@@ -0,0 +1,2897 @@
🗣️ Summary from tag: Summary/v0.0.1 to tag: Summary/v0.0.2:
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

🗣️ Summary from tag: Summary/v0.0.2 to tag: Summary/v0.0.3:
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

🗣️ Summary from tag: Summary/v0.0.3 to tag: Summary/v0.0.4:
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

🗣️ Summary from tag: Summary/v0.0.4 to tag: Summary/v0.0.5:
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

🗣️ Summary from tag: Summary/v0.0.5 to tag: Summary/v0.0.6:
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
index 4449a1b..786f795 100644
--- a/README.md
+++ b/README.md
@@ -67,7 +67,7 @@ Exclude certain files or directories (defailt is `node_modules`).
#### --Omit or -O:

Specify regex patterns to omit files from processing (default is
"Documentation").
`Documentation`).

#### --Parallel or -P:

diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index ae3e650..0e302ee 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -11,10 +11,12 @@
/// # Example
///
/// ```
/// let option = Option {
/// let Option = Option {
///     // Fields needed for summary generation
/// };
/// let summary = Fn(&option);
///
/// let summary = Fn(&Option);
///
/// ```
pub async fn Fn(
	Entry: &str,
@@ -26,17 +28,52 @@ pub async fn Fn(

			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			for (Index, &Current) in Tag.iter().enumerate() {
				for (_, &Next) in Tag.iter().enumerate().skip(Index + 1) {
			let Head = Repository.head()?;

			let First = Repository.find_commit(First::Fn(&Repository)?)?.id().to_string();

			let Last = Head.peel_to_commit()?.id().to_string();

			if Tag.is_empty() {
				println!("🗣️ Summary from first commit: {} to last commit: {}:", First, Last);

				println!(
					"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Current, Next, Option)?
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option,)?
				);
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					println!("🗣️ Summary from tag: {} to tag: {}:", Start, End);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?
					);
				}

				if let Some(Latest) = Tag.last() {
					println!("🗣️ Summary from first commit: {} to latest tag: {}:", First, Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?
					);

					println!("🗣️ Summary from latest tag: {} to last commit: {}:", Latest, Last);

					println!(
						"{}",
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
@@ -47,3 +84,4 @@ pub async fn Fn(
use git2::Repository;

pub mod Difference;
pub mod First;
diff --git a/Source/Fn/Summary/First.rs b/Source/Fn/Summary/First.rs
new file mode 100644
index 0000000..7bfdeee
--- /dev/null
+++ b/Source/Fn/Summary/First.rs
@@ -0,0 +1,20 @@
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

🗣️ Summary from first commit to latest tag: Summary/v0.0.6:
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
index 3e65019..7c5b90e 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -11,13 +11,14 @@ clap = { features = ["derive"], version = "4.5.11" }
walkdir = "2.5.0"
futures = "0.3.30"
rayon = "1.10.0"
tokio = { version = "1.39.1", features = ["full"] }
tokio = { version = "1.39.2", features = ["full"] }
git2 = { version = "0.19.0" }
num_cpus = "1.16.0"
regex = "1.10.5"

[build-dependencies]
serde = { version = "1.0.204", features = ["derive"] }
toml = "0.8.16"
toml = "0.8.17"

[lib]
crate-type = ["staticlib", "cdylib", "rlib"]
@@ -34,5 +35,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.1"
version = "0.0.6"
edition = "2021"
diff --git a/README.md b/README.md
index 871c006..786f795 100644
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
@@ -14,54 +36,97 @@ cargo install psummary

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
`Documentation`).

#### --Parallel or -P:

Run processing in parallel (default is `sequential`):

#### --Pattern:

Specify a custom pattern for matching (defailt is `.git`).

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):

For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
flags and configuration options. [Pieces OS]

## Examples

Analyze the current directory:

```sh
Summary -R D:\Developer .git git fetch upstream
Summary
```

#### --Parallel or -P:

Summary commands in `parallel` (default is `sequential`):
Analyze a specific directory in parallel:

```sh
Summary -P -R D:\Developer .git git fetch upstream
Summary -P -R D:\Developer
```

#### --Exclude:
Exclude additional directories:

Exclude certain files or directories (defailt is
`node_modules target dist vendor`)
```sh
Summary -E "node_modules target dist vendor"
```

#### --Pattern:
Omit specific file patterns:

Specify a custom pattern for matching (defailt is `.git`)
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
index e69de29..4cfbb9f 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -0,0 +1,76 @@
/// Defines and configures command-line arguments for the "Summary" command.
///
/// # Returns
///
/// * `ArgMatches` - The parsed command-line arguments.
///
/// # Example
///
/// ```
/// let matches = Fn();
/// let parallel = matches.get_flag("Parallel");
/// let root = matches.get_one::<String>("Root").unwrap();
/// ```
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
				.default_value("Documentation"),
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
index 0000000..64351c7
--- /dev/null
+++ b/Source/Fn/Binary/Command/Entry.rs
@@ -0,0 +1,51 @@
/// Walks through a directory and filters files based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   * `Exclude`: Vec<String> - List of patterns to exclude
///   * `Pattern`: String - The pattern to match for inclusion
///   * `Root`: String - The root directory to start the walk from
///   * `Separator`: char - The path separator character
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
index 0000000..22654c0
--- /dev/null
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -0,0 +1,55 @@
/// Processes entries in parallel, filtering and executing commands based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   * `Entry`: Vec<Vec<String>> - List of entries to process
///   * `Separator`: char - The path separator character
///   * `Pattern`: String - The pattern to match for inclusion
///
/// # Example
///
/// ```
/// let option = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.txt".to_string()]],
///     Separator: '/',
///     Pattern: "file.txt".to_string(),
/// };
/// Fn(option).await;
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	futures::stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
				Entry
					.last()
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
			.collect::<Vec<String>>(),
	)
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
	.collect::<Vec<_>>()
	.await;
}

use futures::stream::StreamExt;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
new file mode 100644
index 0000000..7ffeb0f
--- /dev/null
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -0,0 +1,52 @@
/// Processes entries sequentially, filtering and executing commands based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   * `Entry`: Vec<Vec<String>> - List of entries to process
///   * `Pattern`: String - The pattern to match for inclusion
///   * `Separator`: char - The path separator character
///
/// # Example
///
/// ```
/// let option = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.txt".to_string()]],
///     Pattern: "file.txt".to_string(),
///     Separator: '/',
/// };
/// Fn(option);
/// ```
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
index 0000000..0e302ee
--- /dev/null
+++ b/Source/Fn/Summary.rs
@@ -0,0 +1,87 @@
/// Generates a summary based on the provided options.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for generating the summary.
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
///
/// let summary = Fn(&Option);
///
/// ```
pub async fn Fn(
	Entry: &str,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<(), Box<dyn std::error::Error>> {
	match Repository::open(Entry) {
		Ok(Repository) => {
			let Name = Repository.tag_names(None)?;

			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			let Head = Repository.head()?;

			let First = Repository.find_commit(First::Fn(&Repository)?)?.id().to_string();

			let Last = Head.peel_to_commit()?.id().to_string();

			if Tag.is_empty() {
				println!("🗣️ Summary from first commit: {} to last commit: {}:", First, Last);

				println!(
					"{}",
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option,)?
				);
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					println!("🗣️ Summary from tag: {} to tag: {}:", Start, End);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?
					);
				}

				if let Some(Latest) = Tag.last() {
					println!("🗣️ Summary from first commit: {} to latest tag: {}:", First, Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?
					);

					println!("🗣️ Summary from latest tag: {} to last commit: {}:", Latest, Last);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Latest, &Last, Option)?
					);
				}
			}
		}
		Err(_Error) => {
			println!("Cannot Repository: {}", _Error);

			return Err(_Error.into());
		}
	}

	Ok(())
}

use git2::Repository;

pub mod Difference;
pub mod First;
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
new file mode 100644
index 0000000..cc25057
--- /dev/null
+++ b/Source/Fn/Summary/Difference.rs
@@ -0,0 +1,122 @@
/// Calculates the difference between two summaries.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for calculating the difference.
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
diff --git a/Source/Fn/Summary/First.rs b/Source/Fn/Summary/First.rs
new file mode 100644
index 0000000..7bfdeee
--- /dev/null
+++ b/Source/Fn/Summary/First.rs
@@ -0,0 +1,20 @@
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
diff --git a/Source/Library.rs b/Source/Library.rs
index 62cfaff..6864249 100644
--- a/Source/Library.rs
+++ b/Source/Library.rs
@@ -1,10 +1,14 @@
#![allow(non_snake_case)]

mod Fn;
mod Struct;

/// The main entry point for the Summary application.
///
/// This function initializes the command structure and executes the appropriate
/// command based on the provided command-line arguments.
#[allow(dead_code)]
#[tokio::main]
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
index 0000000..4a356ae
--- /dev/null
+++ b/Source/Struct/Binary/Command.rs
@@ -0,0 +1,37 @@
/// Represents the main command structure for the Summary application.
pub struct Struct {
	/// The path separator character.
	pub Separator: Option::Separator,
	/// A boxed function that returns a pinned future.
	pub Fn: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static>,
}

impl Struct {
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

use crate::Fn::Binary::Command::{Parallel, Sequential};

use futures::Future;
use std::pin::Pin;

pub mod Entry;
pub mod Option;
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
new file mode 100644
index 0000000..45d3e41
--- /dev/null
+++ b/Source/Struct/Binary/Command/Entry.rs
@@ -0,0 +1,36 @@
/// Represents the entry options for processing in the Summary command.
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

/// Defines a type alias for a vector of vectors of strings.
pub type Type = Vec<Vec<String>>;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
new file mode 100644
index 0000000..8d32f31
--- /dev/null
+++ b/Source/Struct/Binary/Command/Option.rs
@@ -0,0 +1,51 @@
/// Represents the configuration options for the Summary command.
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

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;
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

🗣️ Summary from latest tag: Summary/v0.0.6 to last commit:
diff --git a/README.md b/README.md
index 786f795..d388d49 100644
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
index 0e302ee..f199ca3 100644
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
@@ -35,7 +32,7 @@ pub async fn Fn(
			let Last = Head.peel_to_commit()?.id().to_string();

			if Tag.is_empty() {
				println!("🗣️ Summary from first commit: {} to last commit: {}:", First, Last);
				println!("🗣️ Summary from first commit to last commit:");

				println!(
					"{}",
@@ -55,14 +52,14 @@ pub async fn Fn(
				}

				if let Some(Latest) = Tag.last() {
					println!("🗣️ Summary from first commit: {} to latest tag: {}:", First, Latest);
					println!("🗣️ Summary from first commit to latest tag: {}:", Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?
					);

					println!("🗣️ Summary from latest tag: {} to last commit: {}:", Latest, Last);
					println!("🗣️ Summary from latest tag: {} to last commit:", Latest);

					println!(
						"{}",
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


🗣️ Summary from tag: Summary/v0.0.6 to tag: Summary/v0.0.7:
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
diff --git a/Summary.md b/Summary.md
index f600aa6..f74c21e 100644
--- a/Summary.md
+++ b/Summary.md
@@ -1,3 +1,1121 @@
🗣️ Summary from tag: Summary/v0.0.6 to tag: Summary/v0.0.7:
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

🗣️ Summary from tag: Summary/v0.0.7 to tag: Summary/v0.0.1:
diff --git a/build.rs b/build.rs
index 1f0de60..73ccc94 100644
--- a/build.rs
+++ b/build.rs
@@ -1,5 +1,8 @@
#![allow(non_snake_case)]

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Toml {
	package: Package,
@@ -21,6 +24,3 @@ fn main() {
		.version
	);
}

use serde::Deserialize;
use std::fs;
diff --git a/Cargo.toml b/Cargo.toml
index c907ec5..745ad03 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -14,11 +14,10 @@ rayon = "1.10.0"
tokio = { version = "1.39.2", features = ["full"] }
git2 = { version = "0.19.0" }
num_cpus = "1.16.0"
regex = "1.10.5"

[build-dependencies]
serde = { version = "1.0.204", features = ["derive"] }
toml = "0.8.17"
toml = "0.8.16"

[lib]
crate-type = ["staticlib", "cdylib", "rlib"]
@@ -35,5 +34,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.7"
version = "0.0.1"
edition = "2021"
diff --git a/README.md b/README.md
index d388d49..fd4bfe5 100644
--- a/README.md
+++ b/README.md
@@ -1,40 +1,11 @@
# 🗣️ [Summary] —

[Summary] is a powerful command-line tool designed for efficient `Git`
repository analysis and summarization. It offers both sequential and parallel
processing capabilities, along with flexible file filtering options.
`Summary` is a command-line tool that executes commands in multiple directories
simultaneously. It leverages parallel processing and concurrent `I/O` to
efficiently run tasks across directories.

[Summary]: HTTPS://crates.io/crates/psummary

```sh
Summary -P -O Target -O target -O Summary.md -O CHANGELOG.md > Summary.md
```

[Summary] will now generate the following [Summary.md](./Summary.md) for all the
commits and tags between the first and the latest commit.

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
@@ -43,97 +14,58 @@ cargo install psummary

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
```sh
Summary
```

This command will generate summaries for all the `Git` tags inside the specified
repository.

## Options

The [Summary] tool can be used with various options:

#### --Exclude or -E:

Exclude certain files or directories (defailt is `node_modules`).

#### --Omit or -O:
This command will fetch from upstream for all `.git` repositories inside the
current directory. It essentially replaces the following command:

Specify regex patterns to omit files from processing (default is
`Documentation`).

#### --Parallel or -P:

Run processing in parallel (default is `sequential`):

#### --Pattern:
```sh
find -iname .git -type d -execdir git fetch upstream \;
```

Specify a custom pattern for matching (defailt is `.git`).
## Options

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):

For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
flags and configuration options. [Pieces OS]
```sh
Summary -R D:\Developer .git git fetch upstream
```

## Examples
#### --Parallel or -P:

Analyze the current directory:
Summary commands in `parallel` (default is `sequential`):

```sh
Summary
Summary -P -R D:\Developer .git git fetch upstream
```

Analyze a specific directory in parallel:
#### --Exclude:

```sh
Summary -P -R D:\Developer
```
Exclude certain files or directories (defailt is
`node_modules target dist vendor`)

Exclude additional directories:
#### --Pattern:

```sh
Summary -E "node_modules target dist vendor"
```
Specify a custom pattern for matching (defailt is `.git`)

Omit specific file patterns:
#### --Separator:

```sh
Summary -O "\.md$" -O "\.txt$"
```
Define a custom separator

## Dependencies

[Summary] relies on several Rust crates to provide its functionality:

-   `clap` - For parsing command-line arguments.
-   `futures` - For asynchronous programming abstractions.
-   `git2` - For `Git` repository operations.
-   `num_cpus` - For determining the number of CPUs for parallel processing.
-   `rayon` - For parallel processing.
-   `regex` - For pattern matching and text manipulation.
-   `tokio` - For asynchronous runtime.
-   `walkdir` - For efficient filesystem traversal.
`Summary` relies on several Rust crates to provide its functionality:

[Pieces OS] For extended functionality and system integration.
-   `clap` - Parses command-line arguments
-   `rayon` - Enables parallel processing
-   `tokio` - Provides an asynchronous runtime
-   `walkdir` - Facilitates efficient filesystem traversal

[Summary]: HTTPS://crates.io/crates/psummary
[Pieces OS]: HTTPS://Pieces.App

## Changelog

diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index e355a33..c590122 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -1,39 +1,21 @@
//! This module defines the command-line interface for the Summary application.

/// Configures and returns the command-line argument matches for the Summary application.
///
/// This function sets up the command-line interface using the clap crate, defining
/// various arguments and their properties.
/// Defines and configures command-line arguments for the "Summary" command.
///
/// # Returns
///
/// Returns an `ArgMatches` struct containing the parsed command-line arguments.
/// * `ArgMatches` - The parsed command-line arguments.
///
/// # Example
///
/// ```
/// let matches = Fn();
/// let parallel = matches.get_flag("Parallel");
/// let root = matches.get_one::<String>("Root").unwrap();
/// ```
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
				.default_values(["Target", "Documentation", r"Summary\.md$"]),
		)
		.arg(
			Arg::new("Parallel")
				.short('P')
@@ -44,15 +26,6 @@ pub fn Fn() -> ArgMatches {
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
@@ -63,6 +36,24 @@ pub fn Fn() -> ArgMatches {
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

diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
index 387c0e4..64351c7 100644
--- a/Source/Fn/Binary/Command/Entry.rs
+++ b/Source/Fn/Binary/Command/Entry.rs
@@ -1,14 +1,29 @@
//! This module provides functionality for processing binary command entries.

/// Processes entries based on the provided options.
/// Walks through a directory and filters files based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A reference to an Option struct containing processing parameters.
/// * `Option` - A struct containing the following fields:
///   * `Exclude`: Vec<String> - List of patterns to exclude
///   * `Pattern`: String - The pattern to match for inclusion
///   * `Root`: String - The root directory to start the walk from
///   * `Separator`: char - The path separator character
///
/// # Returns
///
/// Returns a vector of processed entries.
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
pub fn Fn(Option { Exclude, Pattern, Root, Separator, .. }: &Option) -> Return {
	WalkDir::new(Root)
		.follow_links(false)
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 75ef690..37a5949 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -1,20 +1,24 @@
//! This module contains functions for parallel command execution in a binary context.

/// Executes a sequence of operations asynchronously in parallel based on the provided options.
/// Processes entries in parallel, filtering and executing commands based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing various options for execution, including:
///   - `Entry`: A collection of entries to process
///   - `Separator`: A separator used for joining entry parts
///   - `Pattern`: A pattern to match against the last element of each entry
///   - `Omit`: A collection of items to omit from processing
/// * `Option` - A struct containing the following fields:
///   * `Entry`: Vec<Vec<String>> - List of entries to process
///   * `Separator`: char - The path separator character
///   * `Pattern`: String - The pattern to match for inclusion
///
/// # Async
/// # Example
///
/// This function is asynchronous and returns a future.
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	futures::stream::iter(
/// ```
/// let option = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.txt".to_string()]],
///     Separator: '/',
///     Pattern: "file.txt".to_string(),
/// };
/// Fn(option).await;
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
	let Queue: Vec<_> = stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
@@ -25,27 +29,22 @@ pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
			})
			.collect::<Vec<String>>(),
	)
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
	.map(|Entry| async move {
		match crate::Fn::Summary::Fn(&Entry).await {
			Ok(summary) => Ok(summary),
			Err(e) => Err(format!("Error generating summary for {}: {}", Entry, e)),
		}
	})
	.buffer_unordered(num_cpus::get())
	.collect::<Vec<_>>()
	.collect()
	.await;
}

use futures::stream::StreamExt;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
	Queue.par_iter().for_each(|Output| match Output {
		Ok(Summary) => println!("Summary: {:?}", Summary),
		Err(Error) => eprintln!("Error: {}", Error),
	});
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
use futures::stream::{self, StreamExt};
use rayon::prelude::*;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index a84435b..63923f7 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -1,16 +1,23 @@
//! This module contains functions for sequential command execution in a binary context.

/// Executes a sequence of operations asynchronously based on the provided options.
/// Processes entries sequentially, filtering and executing commands based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing various options for execution.
/// * `Option` - A struct containing the following fields:
///   * `Entry`: Vec<Vec<String>> - List of entries to process
///   * `Pattern`: String - The pattern to match for inclusion
///   * `Separator`: char - The path separator character
///
/// # Async
/// # Example
///
/// This function is asynchronous and returns a future.
pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
	futures::future::join_all(
/// ```
/// let option = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.txt".to_string()]],
///     Pattern: "file.txt".to_string(),
///     Separator: '/',
/// };
/// Fn(option);
/// ```
pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
	Entry
		.into_iter()
		.filter_map(|Entry| {
@@ -19,26 +26,9 @@ pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
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
		.for_each(|_Entry| {
			// TODO: GENERATE SUMMARY
		})
			.collect::<Vec<_>>(),
	)
	.await;
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index f199ca3..eb2d2ce 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -1,77 +1,44 @@
//! This module provides functionality for generating summaries of git repositories.

/// Generates a summary for a given git repository entry.
/// Generates a summary based on the provided options.
///
/// # Arguments
///
/// * `Entry` - A string representing the repository path.
/// * `Option` - A reference to a struct containing summary options.
/// * `Option` - A struct containing the necessary information for generating the summary.
///
/// # Returns
///
/// Returns a Result containing () if successful, or a boxed dynamic error if an error occurs.
/// * `Return` - The generated summary.
///
/// # Errors
/// # Example
///
/// This function will return an error if the repository cannot be opened or if there are issues
/// generating the summary.
pub async fn Fn(
	Entry: &str,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<(), Box<dyn std::error::Error>> {
	match Repository::open(Entry) {
		Ok(Repository) => {
			let Name = Repository.tag_names(None)?;

			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			let Head = Repository.head()?;

			let First = Repository.find_commit(First::Fn(&Repository)?)?.id().to_string();

			let Last = Head.peel_to_commit()?.id().to_string();

			if Tag.is_empty() {
				println!("🗣️ Summary from first commit to last commit:");

				println!(
					"{}",
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option,)?
				);
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					println!("🗣️ Summary from tag: {} to tag: {}:", Start, End);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?
					);
				}

				if let Some(Latest) = Tag.last() {
					println!("🗣️ Summary from first commit to latest tag: {}:", Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?
					);

					println!("🗣️ Summary from latest tag: {} to last commit:", Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Latest, &Last, Option)?
					);
				}
			}
/// ```
/// let option = Option {
///     // Fields needed for summary generation
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
			}
		Err(_Error) => {
			println!("Cannot Repository: {}", _Error);

			return Err(_Error.into());
			Start = Some(Tag);
		}
	}

@@ -79,6 +46,10 @@ pub async fn Fn(
}

use git2::Repository;
use std::{
	fs::{self, File},
	io::Write,
};

pub mod Difference;
pub mod First;
pub mod Release;
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
index 544095b..8d7badf 100644
--- a/Source/Fn/Summary/Difference.rs
+++ b/Source/Fn/Summary/Difference.rs
@@ -1,122 +1,33 @@
//! This module provides functionality for generating difference summaries between git commits.

/// Generates a difference summary between two git commits.
/// Calculates the difference between two summaries.
///
/// # Arguments
///
/// * `Repository` - A reference to the git Repository.
/// * `Start` - The starting commit or reference.
/// * `End` - The ending commit or reference.
/// * `Option` - A reference to a struct containing difference options.
/// * `Option` - A struct containing the necessary information for calculating the difference.
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
pub fn Fn(Repo: &git2::Repository, Start: &str, End: &str) -> Result<String, git2::Error> {
	let mut Difference = String::new();

	Repository
		.diff_tree_to_tree(
			Some(&Repository.revparse_single(Start)?.peel_to_commit()?.tree()?),
			Some(&Repository.revparse_single(End)?.peel_to_commit()?.tree()?),
			Some(&mut Options),
	Repo.diff_tree_to_tree(
		Some(&Repo.revparse_single(Start)?.peel_to_commit()?.tree()?),
		Some(&Repo.revparse_single(End)?.peel_to_commit()?.tree()?),
		Some(&mut git2::DiffOptions::new()),
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

	.print(git2::DiffFormat::Patch, |_, _, line| {
		Difference.push_str(std::str::from_utf8(line.content()).unwrap());
		true
	})?;

	Ok(Difference)
}

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
diff --git a/Source/Fn/Summary/First.rs b/Source/Fn/Summary/First.rs
deleted file mode 100644
index a0c3df6..0000000
--- a/Source/Fn/Summary/First.rs
+++ /dev/null
@@ -1,26 +0,0 @@
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
diff --git a/Source/Fn/Summary/Release.rs b/Source/Fn/Summary/Release.rs
new file mode 100644
index 0000000..611f1eb
--- /dev/null
+++ b/Source/Fn/Summary/Release.rs
@@ -0,0 +1,33 @@
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
diff --git a/Source/Library.rs b/Source/Library.rs
index 7b297ec..6864249 100644
--- a/Source/Library.rs
+++ b/Source/Library.rs
@@ -1,13 +1,9 @@
//! The main entry point for the Summary application.

#![allow(non_snake_case)]

/// The main function that initializes and runs the `Summary` application.
///
/// # Errors
/// The main entry point for the Summary application.
///
/// This function will return an error if there are issues parsing arguments
/// or executing the requested commands.
/// This function initializes the command structure and executes the appropriate
/// command based on the provided command-line arguments.
#[allow(dead_code)]
#[tokio::main]
async fn main() {
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
index 7473837..da03f34 100644
--- a/Source/Struct/Binary/Command.rs
+++ b/Source/Struct/Binary/Command.rs
@@ -1,23 +1,12 @@
//! This module defines the main command structure and its implementation for the binary command execution.

/// Represents the main command structure for binary command execution.
/// Represents the main command structure for the Summary application.
pub struct Struct {
	/// The separator used for file paths.
	/// The path separator character.
	pub Separator: Option::Separator,

	/// A boxed function that returns a pinned future representing the command execution.
	/// A boxed function that returns a pinned future.
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
@@ -30,7 +19,7 @@ impl Struct {
							Parallel::Fn(Option).await;
						}
						false => {
							Sequential::Fn(Option).await;
							Sequential::Fn(Option);
						}
					};
				})
@@ -39,10 +28,10 @@ impl Struct {
	}
}

use crate::Fn::Binary::Command::{Parallel, Sequential};

use futures::Future;
use std::pin::Pin;

pub mod Entry;
pub mod Option;

use crate::Fn::Binary::Command::{Parallel, Sequential};
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
index ee24305..c0c6b89 100644
--- a/Source/Struct/Binary/Command/Entry.rs
+++ b/Source/Struct/Binary/Command/Entry.rs
@@ -1,32 +1,22 @@
//! This module defines structures and functions related to binary command entries.

/// Represents the structure for binary command entries.
/// Represents the entry options for processing in the Summary command.
pub struct Struct {
	/// The path.
	pub Entry: Type,

	/// Flag indicating whether to use parallel processing.
	pub Parallel: Parallel,

	/// The pattern to match for inclusion in processing.
	pub Pattern: Pattern,

	/// The path separator character.
	pub Separator: Separator,

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
			Omit: Option.Omit.clone(),
			Parallel: Option.Parallel,
			Pattern: Option.Pattern.clone(),
			Separator: Option.Separator,
@@ -34,8 +24,7 @@ impl Struct {
	}
}

use crate::Struct::Binary::Command::Option::{
	Omit, Parallel, Pattern, Separator, Struct as Option,
};
use crate::Struct::Binary::Command::Option::{Parallel, Pattern, Separator, Struct as Option};

/// Defines a type alias for a vector of vectors of strings.
pub type Type = Vec<Vec<String>>;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index 3f069ff..8453d33 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
@@ -1,55 +1,42 @@
//! This module defines structures and functions related to binary command options.

/// Represents the structure for binary command options.
/// Represents the configuration options for the Summary command.
pub struct Struct {
	/// List of patterns to exclude from processing.
	pub Exclude: Vec<String>,

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
	/// Creates a new Struct instance from the given options.
	///
	/// # Arguments
	///
	/// * `Option` - An Option struct containing initialization parameters.
	///
	/// # Returns
	///
	/// Returns a new instance of Struct.
	/// Creates a new Struct instance from the provided Option.
	pub fn Fn(Option { Separator, .. }: Option) -> Self {
		Self {
			Exclude: Command()
			Exclude: Fn()
				.get_one::<String>("Exclude")
				.expect("Cannot Exclude.")
				.split(" ")
				.map(|Exclude| Exclude.to_string())
				.map(|Command| Command.to_string())
				.collect::<Vec<_>>(),
			Parallel: Command().get_flag("Parallel"),
			Pattern: Command().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Command().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Parallel: Fn().get_flag("Parallel"),
			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Fn().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
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
use crate::{Fn::Binary::Command::Fn, Struct::Binary::Command::Struct as Option};

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;
pub type Omit = Vec<String>;
diff --git a/Source/Struct/mod.rs b/Source/Struct/mod.rs
index 4ca5f2b..a56e8ce 100644
--- a/Source/Struct/mod.rs
+++ b/Source/Struct/mod.rs
@@ -1,2 +1 @@
pub mod Binary;
pub mod Summary;
diff --git a/Source/Struct/Summary/Difference.rs b/Source/Struct/Summary/Difference.rs
deleted file mode 100644
index bb7ec8e..0000000
--- a/Source/Struct/Summary/Difference.rs
+++ /dev/null
@@ -1,6 +0,0 @@
//! This module defines structures related to git diff summary options.

/// Represents the options for generating a git diff summary.
pub struct Struct {
	pub Omit: Vec<String>,
}
diff --git a/Source/Struct/Summary/mod.rs b/Source/Struct/Summary/mod.rs
deleted file mode 100644
index 7241509..0000000
--- a/Source/Struct/Summary/mod.rs
+++ /dev/null
@@ -1 +0,0 @@
pub mod Difference;

🗣️ Summary from tag: Summary/v0.0.1 to tag: Summary/v0.0.2:
diff --git a/Cargo.toml b/Cargo.toml
index 745ad03..c769c35 100644
@@ -1067,149 +2185,7 @@ index 0000000..7241509
@@ -0,0 +1 @@
pub mod Difference;

🗣️ Summary from tag: Summary/v0.0.5 to tag: Summary/v0.0.6:
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
index 4449a1b..786f795 100644
--- a/README.md
+++ b/README.md
@@ -67,7 +67,7 @@ Exclude certain files or directories (defailt is `node_modules`).
#### --Omit or -O:

Specify regex patterns to omit files from processing (default is
"Documentation").
`Documentation`).

#### --Parallel or -P:

diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index ae3e650..0e302ee 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -11,10 +11,12 @@
/// # Example
///
/// ```
/// let option = Option {
/// let Option = Option {
///     // Fields needed for summary generation
/// };
/// let summary = Fn(&option);
///
/// let summary = Fn(&Option);
///
/// ```
pub async fn Fn(
	Entry: &str,
@@ -26,17 +28,52 @@ pub async fn Fn(

			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			for (Index, &Current) in Tag.iter().enumerate() {
				for (_, &Next) in Tag.iter().enumerate().skip(Index + 1) {
			let Head = Repository.head()?;

			let First = Repository.find_commit(First::Fn(&Repository)?)?.id().to_string();

			let Last = Head.peel_to_commit()?.id().to_string();

			if Tag.is_empty() {
				println!("🗣️ Summary from first commit: {} to last commit: {}:", First, Last);

				println!(
					"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Current, Next, Option)?
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option,)?
				);
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					println!("🗣️ Summary from tag: {} to tag: {}:", Start, End);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?
					);
				}

				if let Some(Latest) = Tag.last() {
					println!("🗣️ Summary from first commit: {} to latest tag: {}:", First, Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?
					);

					println!("🗣️ Summary from latest tag: {} to last commit: {}:", Latest, Last);

					println!(
						"{}",
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
@@ -47,3 +84,4 @@ pub async fn Fn(
use git2::Repository;

pub mod Difference;
pub mod First;
diff --git a/Source/Fn/Summary/First.rs b/Source/Fn/Summary/First.rs
new file mode 100644
index 0000000..7bfdeee
--- /dev/null
+++ b/Source/Fn/Summary/First.rs
@@ -0,0 +1,20 @@
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

🗣️ Summary from first commit to latest tag: Summary/v0.0.6:
🗣️ Summary from first commit to latest tag: Summary/v0.0.5:
diff --git a/.gitignore b/.gitignore
index 34f0334..619d2a9 100644
--- a/.gitignore
@@ -1243,10 +2219,10 @@ struct Toml {
use serde::Deserialize;
use std::fs;
diff --git a/Cargo.toml b/Cargo.toml
index 3e65019..7c5b90e 100644
index 3e65019..3615257 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -11,13 +11,14 @@ clap = { features = ["derive"], version = "4.5.11" }
@@ -11,9 +11,10 @@ clap = { features = ["derive"], version = "4.5.11" }
walkdir = "2.5.0"
futures = "0.3.30"
rayon = "1.10.0"
@@ -1258,20 +2234,15 @@ regex = "1.10.5"

[build-dependencies]
serde = { version = "1.0.204", features = ["derive"] }
toml = "0.8.16"
toml = "0.8.17"

[lib]
crate-type = ["staticlib", "cdylib", "rlib"]
@@ -34,5 +35,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.1"
version = "0.0.6"
version = "0.0.5"
edition = "2021"
diff --git a/README.md b/README.md
index 871c006..786f795 100644
index 871c006..4449a1b 100644
--- a/README.md
+++ b/README.md
@@ -1,11 +1,33 @@
@@ -1354,7 +2325,7 @@ Exclude certain files or directories (defailt is `node_modules`).
#### --Omit or -O:

Specify regex patterns to omit files from processing (default is
`Documentation`).
"Documentation").

#### --Parallel or -P:

@@ -1697,10 +2668,10 @@ pub mod Binary;
pub mod Summary;
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
new file mode 100644
index 0000000..0e302ee
index 0000000..ae3e650
--- /dev/null
+++ b/Source/Fn/Summary.rs
@@ -0,0 +1,87 @@
@@ -0,0 +1,49 @@
/// Generates a summary based on the provided options.
///
/// # Arguments
@@ -1714,12 +2685,10 @@ index 0000000..0e302ee
/// # Example
///
/// ```
/// let Option = Option {
/// let option = Option {
///     // Fields needed for summary generation
/// };
///
/// let summary = Fn(&Option);
///
/// let summary = Fn(&option);
/// ```
pub async fn Fn(
	Entry: &str,
@@ -1731,52 +2700,17 @@ pub async fn Fn(

			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			let Head = Repository.head()?;

			let First = Repository.find_commit(First::Fn(&Repository)?)?.id().to_string();

			let Last = Head.peel_to_commit()?.id().to_string();

			if Tag.is_empty() {
				println!("🗣️ Summary from first commit: {} to last commit: {}:", First, Last);

				println!(
					"{}",
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option,)?
				);
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					println!("🗣️ Summary from tag: {} to tag: {}:", Start, End);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?
					);
				}

				if let Some(Latest) = Tag.last() {
					println!("🗣️ Summary from first commit: {} to latest tag: {}:", First, Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?
					);

					println!("🗣️ Summary from latest tag: {} to last commit: {}:", Latest, Last);

			for (Index, &Current) in Tag.iter().enumerate() {
				for (_, &Next) in Tag.iter().enumerate().skip(Index + 1) {
					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Latest, &Last, Option)?
						crate::Fn::Summary::Difference::Fn(&Repository, Current, Next, Option)?
					);
				}
			}
		}
		Err(_Error) => {
			println!("Cannot Repository: {}", _Error);

			println!("Failed to open repository: {}", _Error);
			return Err(_Error.into());
		}
	}
@@ -1787,7 +2721,6 @@ pub async fn Fn(
use git2::Repository;

pub mod Difference;
pub mod First;
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
new file mode 100644
index 0000000..cc25057
@@ -1916,32 +2849,6 @@ pub fn Fn(

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
diff --git a/Source/Fn/Summary/First.rs b/Source/Fn/Summary/First.rs
new file mode 100644
index 0000000..7bfdeee
--- /dev/null
+++ b/Source/Fn/Summary/First.rs
@@ -0,0 +1,20 @@
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
diff --git a/Source/Library.rs b/Source/Library.rs
index 62cfaff..6864249 100644
--- a/Source/Library.rs
@@ -2302,9 +3209,29 @@ index 0000000..7241509
@@ -0,0 +1 @@
pub mod Difference;

🗣️ Summary from latest tag: Summary/v0.0.6 to last commit:
🗣️ Summary from latest tag: Summary/v0.0.5 to last commit:
diff --git a/Cargo.toml b/Cargo.toml
index 3615257..c907ec5 100644
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
version = "0.0.7"
edition = "2021"
diff --git a/README.md b/README.md
index 786f795..d388d49 100644
index 4449a1b..d388d49 100644
--- a/README.md
+++ b/README.md
@@ -1,11 +1,18 @@
@@ -2353,6 +3280,15 @@ The [Summary] tool can be used with various options:

#### --Exclude or -E:

@@ -67,7 +74,7 @@ Exclude certain files or directories (defailt is `node_modules`).
#### --Omit or -O:

Specify regex patterns to omit files from processing (default is
"Documentation").
`Documentation`).

#### --Parallel or -P:

@@ -112,7 +119,7 @@ Summary -O "\.md$" -O "\.txt$"

## Dependencies
@@ -2509,10 +3445,10 @@ pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
	futures::future::join_all(
		Entry
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index 0e302ee..f199ca3 100644
index ae3e650..f199ca3 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -1,23 +1,20 @@
@@ -1,21 +1,20 @@
/// Generates a summary based on the provided options.
//! This module provides functionality for generating summaries of git repositories.

@@ -2527,50 +3463,84 @@ index 0e302ee..f199ca3 100644
/// # Returns
///
/// * `Return` - The generated summary.
/// Returns a Result containing () if successful, or a boxed dynamic error if an error occurs.
///
/// # Example
/// # Errors
///
/// ```
/// let Option = Option {
/// let option = Option {
///     // Fields needed for summary generation
/// };
/// Returns a Result containing () if successful, or a boxed dynamic error if an error occurs.
///
/// let summary = Fn(&Option);
/// # Errors
///
/// let summary = Fn(&option);
/// ```
/// This function will return an error if the repository cannot be opened or if there are issues
/// generating the summary.
pub async fn Fn(
	Entry: &str,
	Option: &crate::Struct::Summary::Difference::Struct,
@@ -35,7 +32,7 @@ pub async fn Fn(
@@ -26,17 +25,52 @@ pub async fn Fn(

			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			for (Index, &Current) in Tag.iter().enumerate() {
				for (_, &Next) in Tag.iter().enumerate().skip(Index + 1) {
			let Head = Repository.head()?;

			let First = Repository.find_commit(First::Fn(&Repository)?)?.id().to_string();

			let Last = Head.peel_to_commit()?.id().to_string();

			if Tag.is_empty() {
				println!("🗣️ Summary from first commit: {} to last commit: {}:", First, Last);
				println!("🗣️ Summary from first commit to last commit:");

				println!(
					"{}",
@@ -55,14 +52,14 @@ pub async fn Fn(
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option,)?
				);
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					println!("🗣️ Summary from tag: {} to tag: {}:", Start, End);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?
					);
				}

				if let Some(Latest) = Tag.last() {
					println!("🗣️ Summary from first commit: {} to latest tag: {}:", First, Latest);
					println!("🗣️ Summary from first commit to latest tag: {}:", Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Current, Next, Option)?
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?
					);

					println!("🗣️ Summary from latest tag: {} to last commit: {}:", Latest, Last);
					println!("🗣️ Summary from latest tag: {} to last commit:", Latest);

					println!(
						"{}",
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
@@ -2707,29 +3677,37 @@ pub fn Fn(

	Omit.extend(Option.Omit.iter().map(|Omit| Omit.as_str()));
diff --git a/Source/Fn/Summary/First.rs b/Source/Fn/Summary/First.rs
index 7bfdeee..a0c3df6 100644
--- a/Source/Fn/Summary/First.rs
new file mode 100644
index 0000000..a0c3df6
--- /dev/null
+++ b/Source/Fn/Summary/First.rs
@@ -1,10 +1,16 @@
/// Function to iterate over the commits in a repository in reverse topological order.
@@ -0,0 +1,26 @@
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

🗣️ Summary from first commit to latest tag: Summary/v0.0.7:
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
index 3e65019..c907ec5 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -11,13 +11,14 @@ clap = { features = ["derive"], version = "4.5.11" }
walkdir = "2.5.0"
futures = "0.3.30"
rayon = "1.10.0"
tokio = { version = "1.39.1", features = ["full"] }
tokio = { version = "1.39.2", features = ["full"] }
git2 = { version = "0.19.0" }
num_cpus = "1.16.0"
regex = "1.10.5"

[build-dependencies]
serde = { version = "1.0.204", features = ["derive"] }
toml = "0.8.16"
toml = "0.8.17"

[lib]
crate-type = ["staticlib", "cdylib", "rlib"]
@@ -34,5 +35,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.1"
version = "0.0.7"
edition = "2021"
diff --git a/README.md b/README.md
index 871c006..d388d49 100644
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
Summary -P -O Target -O target -O Summary.md -O CHANGELOG.md > Summary.md
```

[Summary] will now generate the following [Summary.md](./Summary.md) for all the
commits and tags between the first and the latest commit.

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
@@ -14,54 +43,97 @@ cargo install psummary

## Usage

```sh
Summary
```
The Summary tool can be used with various options:

This command will fetch from upstream for all `.git` repositories inside the
current directory. It essentially replaces the following command:

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

The [Summary] tool can be used with various options:

#### --Exclude or -E:

Exclude certain files or directories (defailt is `node_modules`).

#### --Omit or -O:

Specify regex patterns to omit files from processing (default is
`Documentation`).

#### --Parallel or -P:

Run processing in parallel (default is `sequential`):

#### --Pattern:

Specify a custom pattern for matching (defailt is `.git`).

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):

For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
flags and configuration options. [Pieces OS]

## Examples

Analyze the current directory:

```sh
Summary -R D:\Developer .git git fetch upstream
Summary
```

#### --Parallel or -P:

Summary commands in `parallel` (default is `sequential`):
Analyze a specific directory in parallel:

```sh
Summary -P -R D:\Developer .git git fetch upstream
Summary -P -R D:\Developer
```

#### --Exclude:
Exclude additional directories:

Exclude certain files or directories (defailt is
`node_modules target dist vendor`)
```sh
Summary -E "node_modules target dist vendor"
```

#### --Pattern:
Omit specific file patterns:

Specify a custom pattern for matching (defailt is `.git`)
```sh
Summary -O "\.md$" -O "\.txt$"
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
index e69de29..e355a33 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -0,0 +1,73 @@
//! This module defines the command-line interface for the Summary application.

/// Configures and returns the command-line argument matches for the Summary application.
///
/// This function sets up the command-line interface using the clap crate, defining
/// various arguments and their properties.
///
/// # Returns
///
/// Returns an `ArgMatches` struct containing the parsed command-line arguments.
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
				.default_values(["Target", "Documentation", r"Summary\.md$"]),
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
index 0000000..387c0e4
--- /dev/null
+++ b/Source/Fn/Binary/Command/Entry.rs
@@ -0,0 +1,36 @@
//! This module provides functionality for processing binary command entries.

/// Processes entries based on the provided options.
///
/// # Arguments
///
/// * `Option` - A reference to an Option struct containing processing parameters.
///
/// # Returns
///
/// Returns a vector of processed entries.
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
index 0000000..75ef690
--- /dev/null
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -0,0 +1,51 @@
//! This module contains functions for parallel command execution in a binary context.

/// Executes a sequence of operations asynchronously in parallel based on the provided options.
///
/// # Arguments
///
/// * `Option` - A struct containing various options for execution, including:
///   - `Entry`: A collection of entries to process
///   - `Separator`: A separator used for joining entry parts
///   - `Pattern`: A pattern to match against the last element of each entry
///   - `Omit`: A collection of items to omit from processing
///
/// # Async
///
/// This function is asynchronous and returns a future.
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	futures::stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
				Entry
					.last()
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
			.collect::<Vec<String>>(),
	)
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
	.collect::<Vec<_>>()
	.await;
}

use futures::stream::StreamExt;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
new file mode 100644
index 0000000..a84435b
--- /dev/null
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -0,0 +1,44 @@
//! This module contains functions for sequential command execution in a binary context.

/// Executes a sequence of operations asynchronously based on the provided options.
///
/// # Arguments
///
/// * `Option` - A struct containing various options for execution.
///
/// # Async
///
/// This function is asynchronous and returns a future.
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
index 0000000..f199ca3
--- /dev/null
+++ b/Source/Fn/Summary.rs
@@ -0,0 +1,84 @@
//! This module provides functionality for generating summaries of git repositories.

/// Generates a summary for a given git repository entry.
///
/// # Arguments
///
/// * `Entry` - A string representing the repository path.
/// * `Option` - A reference to a struct containing summary options.
///
/// # Returns
///
/// Returns a Result containing () if successful, or a boxed dynamic error if an error occurs.
///
/// # Errors
///
/// This function will return an error if the repository cannot be opened or if there are issues
/// generating the summary.
pub async fn Fn(
	Entry: &str,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<(), Box<dyn std::error::Error>> {
	match Repository::open(Entry) {
		Ok(Repository) => {
			let Name = Repository.tag_names(None)?;

			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			let Head = Repository.head()?;

			let First = Repository.find_commit(First::Fn(&Repository)?)?.id().to_string();

			let Last = Head.peel_to_commit()?.id().to_string();

			if Tag.is_empty() {
				println!("🗣️ Summary from first commit to last commit:");

				println!(
					"{}",
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option,)?
				);
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					println!("🗣️ Summary from tag: {} to tag: {}:", Start, End);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?
					);
				}

				if let Some(Latest) = Tag.last() {
					println!("🗣️ Summary from first commit to latest tag: {}:", Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?
					);

					println!("🗣️ Summary from latest tag: {} to last commit:", Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Latest, &Last, Option)?
					);
				}
			}
		}
		Err(_Error) => {
			println!("Cannot Repository: {}", _Error);

			return Err(_Error.into());
		}
	}

	Ok(())
}

use git2::Repository;

pub mod Difference;
pub mod First;
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
new file mode 100644
index 0000000..544095b
--- /dev/null
+++ b/Source/Fn/Summary/Difference.rs
@@ -0,0 +1,122 @@
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
index 62cfaff..7b297ec 100644
--- a/Source/Library.rs
+++ b/Source/Library.rs
@@ -1,10 +1,18 @@
#![allow(non_snake_case)]
//! The main entry point for the Summary application.

mod Fn;
mod Struct;
#![allow(non_snake_case)]

/// The main function that initializes and runs the `Summary` application.
///
/// # Errors
///
/// This function will return an error if there are issues parsing arguments
/// or executing the requested commands.
#[allow(dead_code)]
#[tokio::main]
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
index 0000000..7473837
--- /dev/null
+++ b/Source/Struct/Binary/Command.rs
@@ -0,0 +1,48 @@
//! This module defines the main command structure and its implementation for the binary command execution.

/// Represents the main command structure for binary command execution.
pub struct Struct {
	/// The separator used for file paths.
	pub Separator: Option::Separator,

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
index 0000000..ee24305
--- /dev/null
+++ b/Source/Struct/Binary/Command/Entry.rs
@@ -0,0 +1,41 @@
//! This module defines structures and functions related to binary command entries.

/// Represents the structure for binary command entries.
pub struct Struct {
	pub Entry: Type,

	pub Parallel: Parallel,

	pub Pattern: Pattern,

	pub Separator: Separator,

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

pub type Type = Vec<Vec<String>>;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
new file mode 100644
index 0000000..3f069ff
--- /dev/null
+++ b/Source/Struct/Binary/Command/Option.rs
@@ -0,0 +1,55 @@
//! This module defines structures and functions related to binary command options.

/// Represents the structure for binary command options.
pub struct Struct {
	pub Exclude: Vec<String>,

	pub Omit: Vec<String>,

	pub Parallel: Parallel,

	pub Pattern: Pattern,

	pub Root: String,

	pub Separator: Separator,
}

impl Struct {
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

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;
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
index 0000000..bb7ec8e
--- /dev/null
+++ b/Source/Struct/Summary/Difference.rs
@@ -0,0 +1,6 @@
//! This module defines structures related to git diff summary options.

/// Represents the options for generating a git diff summary.
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
diff --git a/Summary.md b/Summary.md
new file mode 100644
index 0000000..f74c21e
--- /dev/null
+++ b/Summary.md
@@ -0,0 +1,3875 @@
🗣️ Summary from tag: Summary/v0.0.6 to tag: Summary/v0.0.7:
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

🗣️ Summary from tag: Summary/v0.0.7 to tag: Summary/v0.0.1:
diff --git a/build.rs b/build.rs
index 1f0de60..73ccc94 100644
--- a/build.rs
+++ b/build.rs
@@ -1,5 +1,8 @@
#![allow(non_snake_case)]

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Toml {
	package: Package,
@@ -21,6 +24,3 @@ fn main() {
		.version
	);
}

use serde::Deserialize;
use std::fs;
diff --git a/Cargo.toml b/Cargo.toml
index c907ec5..745ad03 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -14,11 +14,10 @@ rayon = "1.10.0"
tokio = { version = "1.39.2", features = ["full"] }
git2 = { version = "0.19.0" }
num_cpus = "1.16.0"
regex = "1.10.5"

[build-dependencies]
serde = { version = "1.0.204", features = ["derive"] }
toml = "0.8.17"
toml = "0.8.16"

[lib]
crate-type = ["staticlib", "cdylib", "rlib"]
@@ -35,5 +34,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.7"
version = "0.0.1"
edition = "2021"
diff --git a/README.md b/README.md
index d388d49..fd4bfe5 100644
--- a/README.md
+++ b/README.md
@@ -1,40 +1,11 @@
# 🗣️ [Summary] —

[Summary] is a powerful command-line tool designed for efficient `Git`
repository analysis and summarization. It offers both sequential and parallel
processing capabilities, along with flexible file filtering options.
`Summary` is a command-line tool that executes commands in multiple directories
simultaneously. It leverages parallel processing and concurrent `I/O` to
efficiently run tasks across directories.

[Summary]: HTTPS://crates.io/crates/psummary

```sh
Summary -P -O Target -O target -O Summary.md -O CHANGELOG.md > Summary.md
```

[Summary] will now generate the following [Summary.md](./Summary.md) for all the
commits and tags between the first and the latest commit.

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
@@ -43,97 +14,58 @@ cargo install psummary

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
```sh
Summary
```

This command will generate summaries for all the `Git` tags inside the specified
repository.

## Options

The [Summary] tool can be used with various options:

#### --Exclude or -E:

Exclude certain files or directories (defailt is `node_modules`).

#### --Omit or -O:
This command will fetch from upstream for all `.git` repositories inside the
current directory. It essentially replaces the following command:

Specify regex patterns to omit files from processing (default is
`Documentation`).

#### --Parallel or -P:

Run processing in parallel (default is `sequential`):

#### --Pattern:
```sh
find -iname .git -type d -execdir git fetch upstream \;
```

Specify a custom pattern for matching (defailt is `.git`).
## Options

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):

For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
flags and configuration options. [Pieces OS]
```sh
Summary -R D:\Developer .git git fetch upstream
```

## Examples
#### --Parallel or -P:

Analyze the current directory:
Summary commands in `parallel` (default is `sequential`):

```sh
Summary
Summary -P -R D:\Developer .git git fetch upstream
```

Analyze a specific directory in parallel:
#### --Exclude:

```sh
Summary -P -R D:\Developer
```
Exclude certain files or directories (defailt is
`node_modules target dist vendor`)

Exclude additional directories:
#### --Pattern:

```sh
Summary -E "node_modules target dist vendor"
```
Specify a custom pattern for matching (defailt is `.git`)

Omit specific file patterns:
#### --Separator:

```sh
Summary -O "\.md$" -O "\.txt$"
```
Define a custom separator

## Dependencies

[Summary] relies on several Rust crates to provide its functionality:

-   `clap` - For parsing command-line arguments.
-   `futures` - For asynchronous programming abstractions.
-   `git2` - For `Git` repository operations.
-   `num_cpus` - For determining the number of CPUs for parallel processing.
-   `rayon` - For parallel processing.
-   `regex` - For pattern matching and text manipulation.
-   `tokio` - For asynchronous runtime.
-   `walkdir` - For efficient filesystem traversal.
`Summary` relies on several Rust crates to provide its functionality:

[Pieces OS] For extended functionality and system integration.
-   `clap` - Parses command-line arguments
-   `rayon` - Enables parallel processing
-   `tokio` - Provides an asynchronous runtime
-   `walkdir` - Facilitates efficient filesystem traversal

[Summary]: HTTPS://crates.io/crates/psummary
[Pieces OS]: HTTPS://Pieces.App

## Changelog

diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index e355a33..c590122 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -1,39 +1,21 @@
//! This module defines the command-line interface for the Summary application.

/// Configures and returns the command-line argument matches for the Summary application.
///
/// This function sets up the command-line interface using the clap crate, defining
/// various arguments and their properties.
/// Defines and configures command-line arguments for the "Summary" command.
///
/// # Returns
///
/// Returns an `ArgMatches` struct containing the parsed command-line arguments.
/// * `ArgMatches` - The parsed command-line arguments.
///
/// # Example
///
/// ```
/// let matches = Fn();
/// let parallel = matches.get_flag("Parallel");
/// let root = matches.get_one::<String>("Root").unwrap();
/// ```
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
				.default_values(["Target", "Documentation", r"Summary\.md$"]),
		)
		.arg(
			Arg::new("Parallel")
				.short('P')
@@ -44,15 +26,6 @@ pub fn Fn() -> ArgMatches {
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
@@ -63,6 +36,24 @@ pub fn Fn() -> ArgMatches {
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

diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
index 387c0e4..64351c7 100644
--- a/Source/Fn/Binary/Command/Entry.rs
+++ b/Source/Fn/Binary/Command/Entry.rs
@@ -1,14 +1,29 @@
//! This module provides functionality for processing binary command entries.

/// Processes entries based on the provided options.
/// Walks through a directory and filters files based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A reference to an Option struct containing processing parameters.
/// * `Option` - A struct containing the following fields:
///   * `Exclude`: Vec<String> - List of patterns to exclude
///   * `Pattern`: String - The pattern to match for inclusion
///   * `Root`: String - The root directory to start the walk from
///   * `Separator`: char - The path separator character
///
/// # Returns
///
/// Returns a vector of processed entries.
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
pub fn Fn(Option { Exclude, Pattern, Root, Separator, .. }: &Option) -> Return {
	WalkDir::new(Root)
		.follow_links(false)
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 75ef690..37a5949 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -1,20 +1,24 @@
//! This module contains functions for parallel command execution in a binary context.

/// Executes a sequence of operations asynchronously in parallel based on the provided options.
/// Processes entries in parallel, filtering and executing commands based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing various options for execution, including:
///   - `Entry`: A collection of entries to process
///   - `Separator`: A separator used for joining entry parts
///   - `Pattern`: A pattern to match against the last element of each entry
///   - `Omit`: A collection of items to omit from processing
/// * `Option` - A struct containing the following fields:
///   * `Entry`: Vec<Vec<String>> - List of entries to process
///   * `Separator`: char - The path separator character
///   * `Pattern`: String - The pattern to match for inclusion
///
/// # Async
/// # Example
///
/// This function is asynchronous and returns a future.
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	futures::stream::iter(
/// ```
/// let option = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.txt".to_string()]],
///     Separator: '/',
///     Pattern: "file.txt".to_string(),
/// };
/// Fn(option).await;
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
	let Queue: Vec<_> = stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
@@ -25,27 +29,22 @@ pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
			})
			.collect::<Vec<String>>(),
	)
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
	.map(|Entry| async move {
		match crate::Fn::Summary::Fn(&Entry).await {
			Ok(summary) => Ok(summary),
			Err(e) => Err(format!("Error generating summary for {}: {}", Entry, e)),
		}
	})
	.buffer_unordered(num_cpus::get())
	.collect::<Vec<_>>()
	.collect()
	.await;
}

use futures::stream::StreamExt;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
	Queue.par_iter().for_each(|Output| match Output {
		Ok(Summary) => println!("Summary: {:?}", Summary),
		Err(Error) => eprintln!("Error: {}", Error),
	});
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
use futures::stream::{self, StreamExt};
use rayon::prelude::*;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index a84435b..63923f7 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -1,16 +1,23 @@
//! This module contains functions for sequential command execution in a binary context.

/// Executes a sequence of operations asynchronously based on the provided options.
/// Processes entries sequentially, filtering and executing commands based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing various options for execution.
/// * `Option` - A struct containing the following fields:
///   * `Entry`: Vec<Vec<String>> - List of entries to process
///   * `Pattern`: String - The pattern to match for inclusion
///   * `Separator`: char - The path separator character
///
/// # Async
/// # Example
///
/// This function is asynchronous and returns a future.
pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
	futures::future::join_all(
/// ```
/// let option = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.txt".to_string()]],
///     Pattern: "file.txt".to_string(),
///     Separator: '/',
/// };
/// Fn(option);
/// ```
pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
	Entry
		.into_iter()
		.filter_map(|Entry| {
@@ -19,26 +26,9 @@ pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
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
		.for_each(|_Entry| {
			// TODO: GENERATE SUMMARY
		})
			.collect::<Vec<_>>(),
	)
	.await;
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index f199ca3..eb2d2ce 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -1,77 +1,44 @@
//! This module provides functionality for generating summaries of git repositories.

/// Generates a summary for a given git repository entry.
/// Generates a summary based on the provided options.
///
/// # Arguments
///
/// * `Entry` - A string representing the repository path.
/// * `Option` - A reference to a struct containing summary options.
/// * `Option` - A struct containing the necessary information for generating the summary.
///
/// # Returns
///
/// Returns a Result containing () if successful, or a boxed dynamic error if an error occurs.
/// * `Return` - The generated summary.
///
/// # Errors
/// # Example
///
/// This function will return an error if the repository cannot be opened or if there are issues
/// generating the summary.
pub async fn Fn(
	Entry: &str,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<(), Box<dyn std::error::Error>> {
	match Repository::open(Entry) {
		Ok(Repository) => {
			let Name = Repository.tag_names(None)?;

			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			let Head = Repository.head()?;

			let First = Repository.find_commit(First::Fn(&Repository)?)?.id().to_string();

			let Last = Head.peel_to_commit()?.id().to_string();

			if Tag.is_empty() {
				println!("🗣️ Summary from first commit to last commit:");

				println!(
					"{}",
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option,)?
				);
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					println!("🗣️ Summary from tag: {} to tag: {}:", Start, End);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?
					);
				}

				if let Some(Latest) = Tag.last() {
					println!("🗣️ Summary from first commit to latest tag: {}:", Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?
					);

					println!("🗣️ Summary from latest tag: {} to last commit:", Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Latest, &Last, Option)?
					);
				}
			}
/// ```
/// let option = Option {
///     // Fields needed for summary generation
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
			}
		Err(_Error) => {
			println!("Cannot Repository: {}", _Error);

			return Err(_Error.into());
			Start = Some(Tag);
		}
	}

@@ -79,6 +46,10 @@ pub async fn Fn(
}

use git2::Repository;
use std::{
	fs::{self, File},
	io::Write,
};

pub mod Difference;
pub mod First;
pub mod Release;
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
index 544095b..8d7badf 100644
--- a/Source/Fn/Summary/Difference.rs
+++ b/Source/Fn/Summary/Difference.rs
@@ -1,122 +1,33 @@
//! This module provides functionality for generating difference summaries between git commits.

/// Generates a difference summary between two git commits.
/// Calculates the difference between two summaries.
///
/// # Arguments
///
/// * `Repository` - A reference to the git Repository.
/// * `Start` - The starting commit or reference.
/// * `End` - The ending commit or reference.
/// * `Option` - A reference to a struct containing difference options.
/// * `Option` - A struct containing the necessary information for calculating the difference.
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
pub fn Fn(Repo: &git2::Repository, Start: &str, End: &str) -> Result<String, git2::Error> {
	let mut Difference = String::new();

	Repository
		.diff_tree_to_tree(
			Some(&Repository.revparse_single(Start)?.peel_to_commit()?.tree()?),
			Some(&Repository.revparse_single(End)?.peel_to_commit()?.tree()?),
			Some(&mut Options),
	Repo.diff_tree_to_tree(
		Some(&Repo.revparse_single(Start)?.peel_to_commit()?.tree()?),
		Some(&Repo.revparse_single(End)?.peel_to_commit()?.tree()?),
		Some(&mut git2::DiffOptions::new()),
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

	.print(git2::DiffFormat::Patch, |_, _, line| {
		Difference.push_str(std::str::from_utf8(line.content()).unwrap());
		true
	})?;

	Ok(Difference)
}

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
diff --git a/Source/Fn/Summary/First.rs b/Source/Fn/Summary/First.rs
deleted file mode 100644
index a0c3df6..0000000
--- a/Source/Fn/Summary/First.rs
+++ /dev/null
@@ -1,26 +0,0 @@
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
diff --git a/Source/Fn/Summary/Release.rs b/Source/Fn/Summary/Release.rs
new file mode 100644
index 0000000..611f1eb
--- /dev/null
+++ b/Source/Fn/Summary/Release.rs
@@ -0,0 +1,33 @@
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
diff --git a/Source/Library.rs b/Source/Library.rs
index 7b297ec..6864249 100644
--- a/Source/Library.rs
+++ b/Source/Library.rs
@@ -1,13 +1,9 @@
//! The main entry point for the Summary application.

#![allow(non_snake_case)]

/// The main function that initializes and runs the `Summary` application.
///
/// # Errors
/// The main entry point for the Summary application.
///
/// This function will return an error if there are issues parsing arguments
/// or executing the requested commands.
/// This function initializes the command structure and executes the appropriate
/// command based on the provided command-line arguments.
#[allow(dead_code)]
#[tokio::main]
async fn main() {
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
index 7473837..da03f34 100644
--- a/Source/Struct/Binary/Command.rs
+++ b/Source/Struct/Binary/Command.rs
@@ -1,23 +1,12 @@
//! This module defines the main command structure and its implementation for the binary command execution.

/// Represents the main command structure for binary command execution.
/// Represents the main command structure for the Summary application.
pub struct Struct {
	/// The separator used for file paths.
	/// The path separator character.
	pub Separator: Option::Separator,

	/// A boxed function that returns a pinned future representing the command execution.
	/// A boxed function that returns a pinned future.
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
@@ -30,7 +19,7 @@ impl Struct {
							Parallel::Fn(Option).await;
						}
						false => {
							Sequential::Fn(Option).await;
							Sequential::Fn(Option);
						}
					};
				})
@@ -39,10 +28,10 @@ impl Struct {
	}
}

use crate::Fn::Binary::Command::{Parallel, Sequential};

use futures::Future;
use std::pin::Pin;

pub mod Entry;
pub mod Option;

use crate::Fn::Binary::Command::{Parallel, Sequential};
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
index ee24305..c0c6b89 100644
--- a/Source/Struct/Binary/Command/Entry.rs
+++ b/Source/Struct/Binary/Command/Entry.rs
@@ -1,32 +1,22 @@
//! This module defines structures and functions related to binary command entries.

/// Represents the structure for binary command entries.
/// Represents the entry options for processing in the Summary command.
pub struct Struct {
	/// The path.
	pub Entry: Type,

	/// Flag indicating whether to use parallel processing.
	pub Parallel: Parallel,

	/// The pattern to match for inclusion in processing.
	pub Pattern: Pattern,

	/// The path separator character.
	pub Separator: Separator,

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
			Omit: Option.Omit.clone(),
			Parallel: Option.Parallel,
			Pattern: Option.Pattern.clone(),
			Separator: Option.Separator,
@@ -34,8 +24,7 @@ impl Struct {
	}
}

use crate::Struct::Binary::Command::Option::{
	Omit, Parallel, Pattern, Separator, Struct as Option,
};
use crate::Struct::Binary::Command::Option::{Parallel, Pattern, Separator, Struct as Option};

/// Defines a type alias for a vector of vectors of strings.
pub type Type = Vec<Vec<String>>;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index 3f069ff..8453d33 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
@@ -1,55 +1,42 @@
//! This module defines structures and functions related to binary command options.

/// Represents the structure for binary command options.
/// Represents the configuration options for the Summary command.
pub struct Struct {
	/// List of patterns to exclude from processing.
	pub Exclude: Vec<String>,

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
	/// Creates a new Struct instance from the given options.
	///
	/// # Arguments
	///
	/// * `Option` - An Option struct containing initialization parameters.
	///
	/// # Returns
	///
	/// Returns a new instance of Struct.
	/// Creates a new Struct instance from the provided Option.
	pub fn Fn(Option { Separator, .. }: Option) -> Self {
		Self {
			Exclude: Command()
			Exclude: Fn()
				.get_one::<String>("Exclude")
				.expect("Cannot Exclude.")
				.split(" ")
				.map(|Exclude| Exclude.to_string())
				.map(|Command| Command.to_string())
				.collect::<Vec<_>>(),
			Parallel: Command().get_flag("Parallel"),
			Pattern: Command().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Command().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Parallel: Fn().get_flag("Parallel"),
			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Fn().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
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
use crate::{Fn::Binary::Command::Fn, Struct::Binary::Command::Struct as Option};

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;
pub type Omit = Vec<String>;
diff --git a/Source/Struct/mod.rs b/Source/Struct/mod.rs
index 4ca5f2b..a56e8ce 100644
--- a/Source/Struct/mod.rs
+++ b/Source/Struct/mod.rs
@@ -1,2 +1 @@
pub mod Binary;
pub mod Summary;
diff --git a/Source/Struct/Summary/Difference.rs b/Source/Struct/Summary/Difference.rs
deleted file mode 100644
index bb7ec8e..0000000
--- a/Source/Struct/Summary/Difference.rs
+++ /dev/null
@@ -1,6 +0,0 @@
//! This module defines structures related to git diff summary options.

/// Represents the options for generating a git diff summary.
pub struct Struct {
	pub Omit: Vec<String>,
}
diff --git a/Source/Struct/Summary/mod.rs b/Source/Struct/Summary/mod.rs
deleted file mode 100644
index 7241509..0000000
--- a/Source/Struct/Summary/mod.rs
+++ /dev/null
@@ -1 +0,0 @@
pub mod Difference;

🗣️ Summary from tag: Summary/v0.0.1 to tag: Summary/v0.0.2:
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

🗣️ Summary from tag: Summary/v0.0.2 to tag: Summary/v0.0.3:
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

🗣️ Summary from tag: Summary/v0.0.3 to tag: Summary/v0.0.4:
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

🗣️ Summary from tag: Summary/v0.0.4 to tag: Summary/v0.0.5:
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

🗣️ Summary from first commit to latest tag: Summary/v0.0.5:
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
index 3e65019..3615257 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -11,9 +11,10 @@ clap = { features = ["derive"], version = "4.5.11" }
walkdir = "2.5.0"
futures = "0.3.30"
rayon = "1.10.0"
tokio = { version = "1.39.1", features = ["full"] }
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
diff --git a/README.md b/README.md
index 871c006..4449a1b 100644
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
@@ -14,54 +36,97 @@ cargo install psummary

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

For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
flags and configuration options. [Pieces OS]

## Examples

Analyze the current directory:

```sh
Summary -R D:\Developer .git git fetch upstream
Summary
```

#### --Parallel or -P:

Summary commands in `parallel` (default is `sequential`):
Analyze a specific directory in parallel:

```sh
Summary -P -R D:\Developer .git git fetch upstream
Summary -P -R D:\Developer
```

#### --Exclude:
Exclude additional directories:

Exclude certain files or directories (defailt is
`node_modules target dist vendor`)
```sh
Summary -E "node_modules target dist vendor"
```

#### --Pattern:
Omit specific file patterns:

Specify a custom pattern for matching (defailt is `.git`)
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
index e69de29..4cfbb9f 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -0,0 +1,76 @@
/// Defines and configures command-line arguments for the "Summary" command.
///
/// # Returns
///
/// * `ArgMatches` - The parsed command-line arguments.
///
/// # Example
///
/// ```
/// let matches = Fn();
/// let parallel = matches.get_flag("Parallel");
/// let root = matches.get_one::<String>("Root").unwrap();
/// ```
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
				.default_value("Documentation"),
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
index 0000000..64351c7
--- /dev/null
+++ b/Source/Fn/Binary/Command/Entry.rs
@@ -0,0 +1,51 @@
/// Walks through a directory and filters files based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   * `Exclude`: Vec<String> - List of patterns to exclude
///   * `Pattern`: String - The pattern to match for inclusion
///   * `Root`: String - The root directory to start the walk from
///   * `Separator`: char - The path separator character
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
index 0000000..22654c0
--- /dev/null
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -0,0 +1,55 @@
/// Processes entries in parallel, filtering and executing commands based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   * `Entry`: Vec<Vec<String>> - List of entries to process
///   * `Separator`: char - The path separator character
///   * `Pattern`: String - The pattern to match for inclusion
///
/// # Example
///
/// ```
/// let option = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.txt".to_string()]],
///     Separator: '/',
///     Pattern: "file.txt".to_string(),
/// };
/// Fn(option).await;
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	futures::stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
				Entry
					.last()
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
			.collect::<Vec<String>>(),
	)
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
	.collect::<Vec<_>>()
	.await;
}

use futures::stream::StreamExt;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
new file mode 100644
index 0000000..7ffeb0f
--- /dev/null
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -0,0 +1,52 @@
/// Processes entries sequentially, filtering and executing commands based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   * `Entry`: Vec<Vec<String>> - List of entries to process
///   * `Pattern`: String - The pattern to match for inclusion
///   * `Separator`: char - The path separator character
///
/// # Example
///
/// ```
/// let option = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.txt".to_string()]],
///     Pattern: "file.txt".to_string(),
///     Separator: '/',
/// };
/// Fn(option);
/// ```
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
index 0000000..ae3e650
--- /dev/null
+++ b/Source/Fn/Summary.rs
@@ -0,0 +1,49 @@
/// Generates a summary based on the provided options.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for generating the summary.
///
/// # Returns
///
/// * `Return` - The generated summary.
///
/// # Example
///
/// ```
/// let option = Option {
///     // Fields needed for summary generation
/// };
/// let summary = Fn(&option);
/// ```
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
			}
		}
		Err(_Error) => {
			println!("Failed to open repository: {}", _Error);
			return Err(_Error.into());
		}
	}

	Ok(())
}

use git2::Repository;

pub mod Difference;
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
new file mode 100644
index 0000000..cc25057
--- /dev/null
+++ b/Source/Fn/Summary/Difference.rs
@@ -0,0 +1,122 @@
/// Calculates the difference between two summaries.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for calculating the difference.
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
diff --git a/Source/Library.rs b/Source/Library.rs
index 62cfaff..6864249 100644
--- a/Source/Library.rs
+++ b/Source/Library.rs
@@ -1,10 +1,14 @@
#![allow(non_snake_case)]

mod Fn;
mod Struct;

/// The main entry point for the Summary application.
///
/// This function initializes the command structure and executes the appropriate
/// command based on the provided command-line arguments.
#[allow(dead_code)]
#[tokio::main]
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
index 0000000..4a356ae
--- /dev/null
+++ b/Source/Struct/Binary/Command.rs
@@ -0,0 +1,37 @@
/// Represents the main command structure for the Summary application.
pub struct Struct {
	/// The path separator character.
	pub Separator: Option::Separator,
	/// A boxed function that returns a pinned future.
	pub Fn: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static>,
}

impl Struct {
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

use crate::Fn::Binary::Command::{Parallel, Sequential};

use futures::Future;
use std::pin::Pin;

pub mod Entry;
pub mod Option;
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
new file mode 100644
index 0000000..45d3e41
--- /dev/null
+++ b/Source/Struct/Binary/Command/Entry.rs
@@ -0,0 +1,36 @@
/// Represents the entry options for processing in the Summary command.
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

/// Defines a type alias for a vector of vectors of strings.
pub type Type = Vec<Vec<String>>;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
new file mode 100644
index 0000000..8d32f31
--- /dev/null
+++ b/Source/Struct/Binary/Command/Option.rs
@@ -0,0 +1,51 @@
/// Represents the configuration options for the Summary command.
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

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;
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

🗣️ Summary from latest tag: Summary/v0.0.5 to last commit:
diff --git a/Cargo.toml b/Cargo.toml
index 3615257..c907ec5 100644
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
version = "0.0.7"
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
index ae3e650..f199ca3 100644
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
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option,)?
				);
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					println!("🗣️ Summary from tag: {} to tag: {}:", Start, End);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?
					);
				}

				if let Some(Latest) = Tag.last() {
					println!("🗣️ Summary from first commit to latest tag: {}:", Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Current, Next, Option)?
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?
					);

					println!("🗣️ Summary from latest tag: {} to last commit:", Latest);

					println!(
						"{}",
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


🗣️ Summary from latest tag: Summary/v0.0.7 to last commit:
diff --git a/README.md b/README.md
index d388d49..22a46bb 100644
--- a/README.md
+++ b/README.md
@@ -7,11 +7,11 @@ processing capabilities, along with flexible file filtering options.
[Summary]: HTTPS://crates.io/crates/psummary

```sh
Summary -P -O Target -O target -O Summary.md -O CHANGELOG.md > Summary.md
Summary -P -O Target -O target -O SUMMARY.md -O CHANGELOG.md > SUMMARY.md
```

[Summary] will now generate the following [Summary.md](./Summary.md) for all the
commits and tags between the first and the latest commit.
[Summary] will now generate the following [SUMMARY.md](./SUMMARY.md) for all the
commits and tags between the first and the last commit.

## Features

diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index e355a33..1324482 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -32,7 +32,14 @@ pub fn Fn() -> ArgMatches {
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
				]),
		)
		.arg(
			Arg::new("Parallel")
diff --git a/Summary.md b/Summary.md
deleted file mode 100644
index f74c21e..0000000
--- a/Summary.md
+++ /dev/null
@@ -1,3875 +0,0 @@
🗣️ Summary from tag: Summary/v0.0.6 to tag: Summary/v0.0.7:
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

🗣️ Summary from tag: Summary/v0.0.7 to tag: Summary/v0.0.1:
diff --git a/build.rs b/build.rs
index 1f0de60..73ccc94 100644
--- a/build.rs
+++ b/build.rs
@@ -1,5 +1,8 @@
#![allow(non_snake_case)]

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Toml {
	package: Package,
@@ -21,6 +24,3 @@ fn main() {
		.version
	);
}

use serde::Deserialize;
use std::fs;
diff --git a/Cargo.toml b/Cargo.toml
index c907ec5..745ad03 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -14,11 +14,10 @@ rayon = "1.10.0"
tokio = { version = "1.39.2", features = ["full"] }
git2 = { version = "0.19.0" }
num_cpus = "1.16.0"
regex = "1.10.5"

[build-dependencies]
serde = { version = "1.0.204", features = ["derive"] }
toml = "0.8.17"
toml = "0.8.16"

[lib]
crate-type = ["staticlib", "cdylib", "rlib"]
@@ -35,5 +34,5 @@ description = "🗣️ Summary —"
license = "MIT"
name = "psummary"
repository = "https://github.com/PlayForm/Summary.git"
version = "0.0.7"
version = "0.0.1"
edition = "2021"
diff --git a/README.md b/README.md
index d388d49..fd4bfe5 100644
--- a/README.md
+++ b/README.md
@@ -1,40 +1,11 @@
# 🗣️ [Summary] —

[Summary] is a powerful command-line tool designed for efficient `Git`
repository analysis and summarization. It offers both sequential and parallel
processing capabilities, along with flexible file filtering options.
`Summary` is a command-line tool that executes commands in multiple directories
simultaneously. It leverages parallel processing and concurrent `I/O` to
efficiently run tasks across directories.

[Summary]: HTTPS://crates.io/crates/psummary

```sh
Summary -P -O Target -O target -O Summary.md -O CHANGELOG.md > Summary.md
```

[Summary] will now generate the following [Summary.md](./Summary.md) for all the
commits and tags between the first and the latest commit.

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
@@ -43,97 +14,58 @@ cargo install psummary

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
```sh
Summary
```

This command will generate summaries for all the `Git` tags inside the specified
repository.

## Options

The [Summary] tool can be used with various options:

#### --Exclude or -E:

Exclude certain files or directories (defailt is `node_modules`).

#### --Omit or -O:
This command will fetch from upstream for all `.git` repositories inside the
current directory. It essentially replaces the following command:

Specify regex patterns to omit files from processing (default is
`Documentation`).

#### --Parallel or -P:

Run processing in parallel (default is `sequential`):

#### --Pattern:
```sh
find -iname .git -type d -execdir git fetch upstream \;
```

Specify a custom pattern for matching (defailt is `.git`).
## Options

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):

For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
flags and configuration options. [Pieces OS]
```sh
Summary -R D:\Developer .git git fetch upstream
```

## Examples
#### --Parallel or -P:

Analyze the current directory:
Summary commands in `parallel` (default is `sequential`):

```sh
Summary
Summary -P -R D:\Developer .git git fetch upstream
```

Analyze a specific directory in parallel:
#### --Exclude:

```sh
Summary -P -R D:\Developer
```
Exclude certain files or directories (defailt is
`node_modules target dist vendor`)

Exclude additional directories:
#### --Pattern:

```sh
Summary -E "node_modules target dist vendor"
```
Specify a custom pattern for matching (defailt is `.git`)

Omit specific file patterns:
#### --Separator:

```sh
Summary -O "\.md$" -O "\.txt$"
```
Define a custom separator

## Dependencies

[Summary] relies on several Rust crates to provide its functionality:

-   `clap` - For parsing command-line arguments.
-   `futures` - For asynchronous programming abstractions.
-   `git2` - For `Git` repository operations.
-   `num_cpus` - For determining the number of CPUs for parallel processing.
-   `rayon` - For parallel processing.
-   `regex` - For pattern matching and text manipulation.
-   `tokio` - For asynchronous runtime.
-   `walkdir` - For efficient filesystem traversal.
`Summary` relies on several Rust crates to provide its functionality:

[Pieces OS] For extended functionality and system integration.
-   `clap` - Parses command-line arguments
-   `rayon` - Enables parallel processing
-   `tokio` - Provides an asynchronous runtime
-   `walkdir` - Facilitates efficient filesystem traversal

[Summary]: HTTPS://crates.io/crates/psummary
[Pieces OS]: HTTPS://Pieces.App

## Changelog

diff --git a/Source/Fn/Binary/Command.rs b/Source/Fn/Binary/Command.rs
index e355a33..c590122 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -1,39 +1,21 @@
//! This module defines the command-line interface for the Summary application.

/// Configures and returns the command-line argument matches for the Summary application.
///
/// This function sets up the command-line interface using the clap crate, defining
/// various arguments and their properties.
/// Defines and configures command-line arguments for the "Summary" command.
///
/// # Returns
///
/// Returns an `ArgMatches` struct containing the parsed command-line arguments.
/// * `ArgMatches` - The parsed command-line arguments.
///
/// # Example
///
/// ```
/// let matches = Fn();
/// let parallel = matches.get_flag("Parallel");
/// let root = matches.get_one::<String>("Root").unwrap();
/// ```
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
				.default_values(["Target", "Documentation", r"Summary\.md$"]),
		)
		.arg(
			Arg::new("Parallel")
				.short('P')
@@ -44,15 +26,6 @@ pub fn Fn() -> ArgMatches {
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
@@ -63,6 +36,24 @@ pub fn Fn() -> ArgMatches {
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

diff --git a/Source/Fn/Binary/Command/Entry.rs b/Source/Fn/Binary/Command/Entry.rs
index 387c0e4..64351c7 100644
--- a/Source/Fn/Binary/Command/Entry.rs
+++ b/Source/Fn/Binary/Command/Entry.rs
@@ -1,14 +1,29 @@
//! This module provides functionality for processing binary command entries.

/// Processes entries based on the provided options.
/// Walks through a directory and filters files based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A reference to an Option struct containing processing parameters.
/// * `Option` - A struct containing the following fields:
///   * `Exclude`: Vec<String> - List of patterns to exclude
///   * `Pattern`: String - The pattern to match for inclusion
///   * `Root`: String - The root directory to start the walk from
///   * `Separator`: char - The path separator character
///
/// # Returns
///
/// Returns a vector of processed entries.
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
pub fn Fn(Option { Exclude, Pattern, Root, Separator, .. }: &Option) -> Return {
	WalkDir::new(Root)
		.follow_links(false)
diff --git a/Source/Fn/Binary/Command/Parallel.rs b/Source/Fn/Binary/Command/Parallel.rs
index 75ef690..37a5949 100644
--- a/Source/Fn/Binary/Command/Parallel.rs
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -1,20 +1,24 @@
//! This module contains functions for parallel command execution in a binary context.

/// Executes a sequence of operations asynchronously in parallel based on the provided options.
/// Processes entries in parallel, filtering and executing commands based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing various options for execution, including:
///   - `Entry`: A collection of entries to process
///   - `Separator`: A separator used for joining entry parts
///   - `Pattern`: A pattern to match against the last element of each entry
///   - `Omit`: A collection of items to omit from processing
/// * `Option` - A struct containing the following fields:
///   * `Entry`: Vec<Vec<String>> - List of entries to process
///   * `Separator`: char - The path separator character
///   * `Pattern`: String - The pattern to match for inclusion
///
/// # Async
/// # Example
///
/// This function is asynchronous and returns a future.
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	futures::stream::iter(
/// ```
/// let option = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.txt".to_string()]],
///     Separator: '/',
///     Pattern: "file.txt".to_string(),
/// };
/// Fn(option).await;
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, .. }: Option) {
	let Queue: Vec<_> = stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
@@ -25,27 +29,22 @@ pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
			})
			.collect::<Vec<String>>(),
	)
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
	.map(|Entry| async move {
		match crate::Fn::Summary::Fn(&Entry).await {
			Ok(summary) => Ok(summary),
			Err(e) => Err(format!("Error generating summary for {}: {}", Entry, e)),
		}
	})
	.buffer_unordered(num_cpus::get())
	.collect::<Vec<_>>()
	.collect()
	.await;
}

use futures::stream::StreamExt;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
	Queue.par_iter().for_each(|Output| match Output {
		Ok(Summary) => println!("Summary: {:?}", Summary),
		Err(Error) => eprintln!("Error: {}", Error),
	});
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
use futures::stream::{self, StreamExt};
use rayon::prelude::*;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
index a84435b..63923f7 100644
--- a/Source/Fn/Binary/Command/Sequential.rs
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -1,16 +1,23 @@
//! This module contains functions for sequential command execution in a binary context.

/// Executes a sequence of operations asynchronously based on the provided options.
/// Processes entries sequentially, filtering and executing commands based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing various options for execution.
/// * `Option` - A struct containing the following fields:
///   * `Entry`: Vec<Vec<String>> - List of entries to process
///   * `Pattern`: String - The pattern to match for inclusion
///   * `Separator`: char - The path separator character
///
/// # Async
/// # Example
///
/// This function is asynchronous and returns a future.
pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
	futures::future::join_all(
/// ```
/// let option = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.txt".to_string()]],
///     Pattern: "file.txt".to_string(),
///     Separator: '/',
/// };
/// Fn(option);
/// ```
pub fn Fn(Option { Entry, Pattern, Separator, .. }: Option) {
	Entry
		.into_iter()
		.filter_map(|Entry| {
@@ -19,26 +26,9 @@ pub async fn Fn(Option { Entry, Pattern, Separator, Omit, .. }: Option) {
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
		.for_each(|_Entry| {
			// TODO: GENERATE SUMMARY
		})
			.collect::<Vec<_>>(),
	)
	.await;
}

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Summary.rs b/Source/Fn/Summary.rs
index f199ca3..eb2d2ce 100644
--- a/Source/Fn/Summary.rs
+++ b/Source/Fn/Summary.rs
@@ -1,77 +1,44 @@
//! This module provides functionality for generating summaries of git repositories.

/// Generates a summary for a given git repository entry.
/// Generates a summary based on the provided options.
///
/// # Arguments
///
/// * `Entry` - A string representing the repository path.
/// * `Option` - A reference to a struct containing summary options.
/// * `Option` - A struct containing the necessary information for generating the summary.
///
/// # Returns
///
/// Returns a Result containing () if successful, or a boxed dynamic error if an error occurs.
/// * `Return` - The generated summary.
///
/// # Errors
/// # Example
///
/// This function will return an error if the repository cannot be opened or if there are issues
/// generating the summary.
pub async fn Fn(
	Entry: &str,
	Option: &crate::Struct::Summary::Difference::Struct,
) -> Result<(), Box<dyn std::error::Error>> {
	match Repository::open(Entry) {
		Ok(Repository) => {
			let Name = Repository.tag_names(None)?;

			let Tag: Vec<_> = Name.iter().filter_map(|Tag| Tag).collect();

			let Head = Repository.head()?;

			let First = Repository.find_commit(First::Fn(&Repository)?)?.id().to_string();

			let Last = Head.peel_to_commit()?.id().to_string();

			if Tag.is_empty() {
				println!("🗣️ Summary from first commit to last commit:");

				println!(
					"{}",
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option,)?
				);
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					println!("🗣️ Summary from tag: {} to tag: {}:", Start, End);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?
					);
				}

				if let Some(Latest) = Tag.last() {
					println!("🗣️ Summary from first commit to latest tag: {}:", Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?
					);

					println!("🗣️ Summary from latest tag: {} to last commit:", Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Latest, &Last, Option)?
					);
				}
			}
/// ```
/// let option = Option {
///     // Fields needed for summary generation
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
			}
		Err(_Error) => {
			println!("Cannot Repository: {}", _Error);

			return Err(_Error.into());
			Start = Some(Tag);
		}
	}

@@ -79,6 +46,10 @@ pub async fn Fn(
}

use git2::Repository;
use std::{
	fs::{self, File},
	io::Write,
};

pub mod Difference;
pub mod First;
pub mod Release;
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
index 544095b..8d7badf 100644
--- a/Source/Fn/Summary/Difference.rs
+++ b/Source/Fn/Summary/Difference.rs
@@ -1,122 +1,33 @@
//! This module provides functionality for generating difference summaries between git commits.

/// Generates a difference summary between two git commits.
/// Calculates the difference between two summaries.
///
/// # Arguments
///
/// * `Repository` - A reference to the git Repository.
/// * `Start` - The starting commit or reference.
/// * `End` - The ending commit or reference.
/// * `Option` - A reference to a struct containing difference options.
/// * `Option` - A struct containing the necessary information for calculating the difference.
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
pub fn Fn(Repo: &git2::Repository, Start: &str, End: &str) -> Result<String, git2::Error> {
	let mut Difference = String::new();

	Repository
		.diff_tree_to_tree(
			Some(&Repository.revparse_single(Start)?.peel_to_commit()?.tree()?),
			Some(&Repository.revparse_single(End)?.peel_to_commit()?.tree()?),
			Some(&mut Options),
	Repo.diff_tree_to_tree(
		Some(&Repo.revparse_single(Start)?.peel_to_commit()?.tree()?),
		Some(&Repo.revparse_single(End)?.peel_to_commit()?.tree()?),
		Some(&mut git2::DiffOptions::new()),
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

	.print(git2::DiffFormat::Patch, |_, _, line| {
		Difference.push_str(std::str::from_utf8(line.content()).unwrap());
		true
	})?;

	Ok(Difference)
}

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
diff --git a/Source/Fn/Summary/First.rs b/Source/Fn/Summary/First.rs
deleted file mode 100644
index a0c3df6..0000000
--- a/Source/Fn/Summary/First.rs
+++ /dev/null
@@ -1,26 +0,0 @@
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
diff --git a/Source/Fn/Summary/Release.rs b/Source/Fn/Summary/Release.rs
new file mode 100644
index 0000000..611f1eb
--- /dev/null
+++ b/Source/Fn/Summary/Release.rs
@@ -0,0 +1,33 @@
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
diff --git a/Source/Library.rs b/Source/Library.rs
index 7b297ec..6864249 100644
--- a/Source/Library.rs
+++ b/Source/Library.rs
@@ -1,13 +1,9 @@
//! The main entry point for the Summary application.

#![allow(non_snake_case)]

/// The main function that initializes and runs the `Summary` application.
///
/// # Errors
/// The main entry point for the Summary application.
///
/// This function will return an error if there are issues parsing arguments
/// or executing the requested commands.
/// This function initializes the command structure and executes the appropriate
/// command based on the provided command-line arguments.
#[allow(dead_code)]
#[tokio::main]
async fn main() {
diff --git a/Source/Struct/Binary/Command.rs b/Source/Struct/Binary/Command.rs
index 7473837..da03f34 100644
--- a/Source/Struct/Binary/Command.rs
+++ b/Source/Struct/Binary/Command.rs
@@ -1,23 +1,12 @@
//! This module defines the main command structure and its implementation for the binary command execution.

/// Represents the main command structure for binary command execution.
/// Represents the main command structure for the Summary application.
pub struct Struct {
	/// The separator used for file paths.
	/// The path separator character.
	pub Separator: Option::Separator,

	/// A boxed function that returns a pinned future representing the command execution.
	/// A boxed function that returns a pinned future.
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
@@ -30,7 +19,7 @@ impl Struct {
							Parallel::Fn(Option).await;
						}
						false => {
							Sequential::Fn(Option).await;
							Sequential::Fn(Option);
						}
					};
				})
@@ -39,10 +28,10 @@ impl Struct {
	}
}

use crate::Fn::Binary::Command::{Parallel, Sequential};

use futures::Future;
use std::pin::Pin;

pub mod Entry;
pub mod Option;

use crate::Fn::Binary::Command::{Parallel, Sequential};
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
index ee24305..c0c6b89 100644
--- a/Source/Struct/Binary/Command/Entry.rs
+++ b/Source/Struct/Binary/Command/Entry.rs
@@ -1,32 +1,22 @@
//! This module defines structures and functions related to binary command entries.

/// Represents the structure for binary command entries.
/// Represents the entry options for processing in the Summary command.
pub struct Struct {
	/// The path.
	pub Entry: Type,

	/// Flag indicating whether to use parallel processing.
	pub Parallel: Parallel,

	/// The pattern to match for inclusion in processing.
	pub Pattern: Pattern,

	/// The path separator character.
	pub Separator: Separator,

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
			Omit: Option.Omit.clone(),
			Parallel: Option.Parallel,
			Pattern: Option.Pattern.clone(),
			Separator: Option.Separator,
@@ -34,8 +24,7 @@ impl Struct {
	}
}

use crate::Struct::Binary::Command::Option::{
	Omit, Parallel, Pattern, Separator, Struct as Option,
};
use crate::Struct::Binary::Command::Option::{Parallel, Pattern, Separator, Struct as Option};

/// Defines a type alias for a vector of vectors of strings.
pub type Type = Vec<Vec<String>>;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
index 3f069ff..8453d33 100644
--- a/Source/Struct/Binary/Command/Option.rs
+++ b/Source/Struct/Binary/Command/Option.rs
@@ -1,55 +1,42 @@
//! This module defines structures and functions related to binary command options.

/// Represents the structure for binary command options.
/// Represents the configuration options for the Summary command.
pub struct Struct {
	/// List of patterns to exclude from processing.
	pub Exclude: Vec<String>,

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
	/// Creates a new Struct instance from the given options.
	///
	/// # Arguments
	///
	/// * `Option` - An Option struct containing initialization parameters.
	///
	/// # Returns
	///
	/// Returns a new instance of Struct.
	/// Creates a new Struct instance from the provided Option.
	pub fn Fn(Option { Separator, .. }: Option) -> Self {
		Self {
			Exclude: Command()
			Exclude: Fn()
				.get_one::<String>("Exclude")
				.expect("Cannot Exclude.")
				.split(" ")
				.map(|Exclude| Exclude.to_string())
				.map(|Command| Command.to_string())
				.collect::<Vec<_>>(),
			Parallel: Command().get_flag("Parallel"),
			Pattern: Command().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Command().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
			Parallel: Fn().get_flag("Parallel"),
			Pattern: Fn().get_one::<String>("Pattern").expect("Cannot Pattern.").to_owned(),
			Root: Fn().get_one::<String>("Root").expect("Cannot Root.").to_owned(),
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
use crate::{Fn::Binary::Command::Fn, Struct::Binary::Command::Struct as Option};

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;
pub type Omit = Vec<String>;
diff --git a/Source/Struct/mod.rs b/Source/Struct/mod.rs
index 4ca5f2b..a56e8ce 100644
--- a/Source/Struct/mod.rs
+++ b/Source/Struct/mod.rs
@@ -1,2 +1 @@
pub mod Binary;
pub mod Summary;
diff --git a/Source/Struct/Summary/Difference.rs b/Source/Struct/Summary/Difference.rs
deleted file mode 100644
index bb7ec8e..0000000
--- a/Source/Struct/Summary/Difference.rs
+++ /dev/null
@@ -1,6 +0,0 @@
//! This module defines structures related to git diff summary options.

/// Represents the options for generating a git diff summary.
pub struct Struct {
	pub Omit: Vec<String>,
}
diff --git a/Source/Struct/Summary/mod.rs b/Source/Struct/Summary/mod.rs
deleted file mode 100644
index 7241509..0000000
--- a/Source/Struct/Summary/mod.rs
+++ /dev/null
@@ -1 +0,0 @@
pub mod Difference;

🗣️ Summary from tag: Summary/v0.0.1 to tag: Summary/v0.0.2:
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

🗣️ Summary from tag: Summary/v0.0.2 to tag: Summary/v0.0.3:
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

🗣️ Summary from tag: Summary/v0.0.3 to tag: Summary/v0.0.4:
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

🗣️ Summary from tag: Summary/v0.0.4 to tag: Summary/v0.0.5:
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

🗣️ Summary from first commit to latest tag: Summary/v0.0.5:
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
index 3e65019..3615257 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -11,9 +11,10 @@ clap = { features = ["derive"], version = "4.5.11" }
walkdir = "2.5.0"
futures = "0.3.30"
rayon = "1.10.0"
tokio = { version = "1.39.1", features = ["full"] }
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
diff --git a/README.md b/README.md
index 871c006..4449a1b 100644
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
@@ -14,54 +36,97 @@ cargo install psummary

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

For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
flags and configuration options. [Pieces OS]

## Examples

Analyze the current directory:

```sh
Summary -R D:\Developer .git git fetch upstream
Summary
```

#### --Parallel or -P:

Summary commands in `parallel` (default is `sequential`):
Analyze a specific directory in parallel:

```sh
Summary -P -R D:\Developer .git git fetch upstream
Summary -P -R D:\Developer
```

#### --Exclude:
Exclude additional directories:

Exclude certain files or directories (defailt is
`node_modules target dist vendor`)
```sh
Summary -E "node_modules target dist vendor"
```

#### --Pattern:
Omit specific file patterns:

Specify a custom pattern for matching (defailt is `.git`)
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
index e69de29..4cfbb9f 100644
--- a/Source/Fn/Binary/Command.rs
+++ b/Source/Fn/Binary/Command.rs
@@ -0,0 +1,76 @@
/// Defines and configures command-line arguments for the "Summary" command.
///
/// # Returns
///
/// * `ArgMatches` - The parsed command-line arguments.
///
/// # Example
///
/// ```
/// let matches = Fn();
/// let parallel = matches.get_flag("Parallel");
/// let root = matches.get_one::<String>("Root").unwrap();
/// ```
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
				.default_value("Documentation"),
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
index 0000000..64351c7
--- /dev/null
+++ b/Source/Fn/Binary/Command/Entry.rs
@@ -0,0 +1,51 @@
/// Walks through a directory and filters files based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   * `Exclude`: Vec<String> - List of patterns to exclude
///   * `Pattern`: String - The pattern to match for inclusion
///   * `Root`: String - The root directory to start the walk from
///   * `Separator`: char - The path separator character
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
index 0000000..22654c0
--- /dev/null
+++ b/Source/Fn/Binary/Command/Parallel.rs
@@ -0,0 +1,55 @@
/// Processes entries in parallel, filtering and executing commands based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   * `Entry`: Vec<Vec<String>> - List of entries to process
///   * `Separator`: char - The path separator character
///   * `Pattern`: String - The pattern to match for inclusion
///
/// # Example
///
/// ```
/// let option = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.txt".to_string()]],
///     Separator: '/',
///     Pattern: "file.txt".to_string(),
/// };
/// Fn(option).await;
/// ```
pub async fn Fn(Option { Entry, Separator, Pattern, Omit, .. }: Option) {
	futures::stream::iter(
		Entry
			.into_par_iter()
			.filter_map(|Entry| {
				Entry
					.last()
					.filter(|Last| *Last == &Pattern)
					.map(|_| Entry[0..Entry.len() - 1].join(&Separator.to_string()))
			})
			.collect::<Vec<String>>(),
	)
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
	.collect::<Vec<_>>()
	.await;
}

use futures::stream::StreamExt;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::Struct::Binary::Command::Entry::Struct as Option;
diff --git a/Source/Fn/Binary/Command/Sequential.rs b/Source/Fn/Binary/Command/Sequential.rs
new file mode 100644
index 0000000..7ffeb0f
--- /dev/null
+++ b/Source/Fn/Binary/Command/Sequential.rs
@@ -0,0 +1,52 @@
/// Processes entries sequentially, filtering and executing commands based on specified criteria.
///
/// # Arguments
///
/// * `Option` - A struct containing the following fields:
///   * `Entry`: Vec<Vec<String>> - List of entries to process
///   * `Pattern`: String - The pattern to match for inclusion
///   * `Separator`: char - The path separator character
///
/// # Example
///
/// ```
/// let option = Option {
///     Entry: vec![vec!["path".to_string(), "to".to_string(), "file.txt".to_string()]],
///     Pattern: "file.txt".to_string(),
///     Separator: '/',
/// };
/// Fn(option);
/// ```
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
index 0000000..ae3e650
--- /dev/null
+++ b/Source/Fn/Summary.rs
@@ -0,0 +1,49 @@
/// Generates a summary based on the provided options.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for generating the summary.
///
/// # Returns
///
/// * `Return` - The generated summary.
///
/// # Example
///
/// ```
/// let option = Option {
///     // Fields needed for summary generation
/// };
/// let summary = Fn(&option);
/// ```
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
			}
		}
		Err(_Error) => {
			println!("Failed to open repository: {}", _Error);
			return Err(_Error.into());
		}
	}

	Ok(())
}

use git2::Repository;

pub mod Difference;
diff --git a/Source/Fn/Summary/Difference.rs b/Source/Fn/Summary/Difference.rs
new file mode 100644
index 0000000..cc25057
--- /dev/null
+++ b/Source/Fn/Summary/Difference.rs
@@ -0,0 +1,122 @@
/// Calculates the difference between two summaries.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for calculating the difference.
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
diff --git a/Source/Library.rs b/Source/Library.rs
index 62cfaff..6864249 100644
--- a/Source/Library.rs
+++ b/Source/Library.rs
@@ -1,10 +1,14 @@
#![allow(non_snake_case)]

mod Fn;
mod Struct;

/// The main entry point for the Summary application.
///
/// This function initializes the command structure and executes the appropriate
/// command based on the provided command-line arguments.
#[allow(dead_code)]
#[tokio::main]
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
index 0000000..4a356ae
--- /dev/null
+++ b/Source/Struct/Binary/Command.rs
@@ -0,0 +1,37 @@
/// Represents the main command structure for the Summary application.
pub struct Struct {
	/// The path separator character.
	pub Separator: Option::Separator,
	/// A boxed function that returns a pinned future.
	pub Fn: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static>,
}

impl Struct {
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

use crate::Fn::Binary::Command::{Parallel, Sequential};

use futures::Future;
use std::pin::Pin;

pub mod Entry;
pub mod Option;
diff --git a/Source/Struct/Binary/Command/Entry.rs b/Source/Struct/Binary/Command/Entry.rs
new file mode 100644
index 0000000..45d3e41
--- /dev/null
+++ b/Source/Struct/Binary/Command/Entry.rs
@@ -0,0 +1,36 @@
/// Represents the entry options for processing in the Summary command.
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

/// Defines a type alias for a vector of vectors of strings.
pub type Type = Vec<Vec<String>>;
diff --git a/Source/Struct/Binary/Command/Option.rs b/Source/Struct/Binary/Command/Option.rs
new file mode 100644
index 0000000..8d32f31
--- /dev/null
+++ b/Source/Struct/Binary/Command/Option.rs
@@ -0,0 +1,51 @@
/// Represents the configuration options for the Summary command.
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

pub type Command = Vec<String>;
pub type Parallel = bool;
pub type Pattern = String;
pub type Separator = char;
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

🗣️ Summary from latest tag: Summary/v0.0.5 to last commit:
diff --git a/Cargo.toml b/Cargo.toml
index 3615257..c907ec5 100644
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
version = "0.0.7"
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
index ae3e650..f199ca3 100644
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
					crate::Fn::Summary::Difference::Fn(&Repository, &First, &Last, Option,)?
				);
			} else {
				for Window in Tag.windows(2) {
					let Start = Window[0];
					let End = Window[1];

					println!("🗣️ Summary from tag: {} to tag: {}:", Start, End);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Start, End, Option)?
					);
				}

				if let Some(Latest) = Tag.last() {
					println!("🗣️ Summary from first commit to latest tag: {}:", Latest);

					println!(
						"{}",
						crate::Fn::Summary::Difference::Fn(&Repository, Current, Next, Option)?
						crate::Fn::Summary::Difference::Fn(&Repository, &First, Latest, Option)?
					);

					println!("🗣️ Summary from latest tag: {} to last commit:", Latest);

					println!(
						"{}",
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


