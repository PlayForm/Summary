# 🗣️ [Summary] —

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
cargo install psummary
```

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
Summary
```

Analyze a specific directory in parallel:

```sh
Summary -P -R D:\Developer
```

Exclude additional directories:

```sh
Summary -E "node_modules target dist vendor"
```

Omit specific file patterns:

```sh
Summary -O "\.md$" -O "\.txt$"
```

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

[Pieces OS] For extended functionality and system integration.

[Summary]: HTTPS://crates.io/crates/psummary
[Pieces OS]: HTTPS://Pieces.App

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a history of changes to this CLI.
