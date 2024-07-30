# 🗣️ [Summary] —

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

The `Summary` CLI supports [Pieces OS], essentially acting as a plugin that can rewrite
the whole system. This integration allows for:

-   Enhanced code analysis and summarization.
-   Improved context-aware processing.
-   Seamless integration with other [Pieces OS]-compatible tools.
-   Potential for AI-driven insights and optimizations.

By leveraging [Pieces OS], `Summary` can tap into a broader ecosystem of development
tools and services, significantly expanding its capabilities beyond basic file processing.

## Installation

```sh
cargo install psummary
```

## Usage

The Summary tool can be used with various options:

```sh
Summary [OPTIONS]
```

This command will generate summaries for all the Git tags inside the specified
repository.

## Options

The `Summary` tool can be used with various options:

-   `--Root` or `-R`: Set the root directory to analyze (default is current
    directory)
-   `--Parallel` or `-P`: Run processing in parallel (default is sequential)
-   `--Exclude` or `-E`: Exclude certain files or directories (default is
    "node_modules")
-   `--Pattern`: Specify a custom pattern for matching (default is ".git")
-   `--Omit` or `-O`: Specify regex patterns to omit files from processing
    (default is "Documentation")
-   `--Separator`: Define a custom separator (default is system path separator)

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

## Options

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):

```sh
Summary -R D:\Developer
```

#### --Parallel or -P:

Summary commands in `parallel` (default is `sequential`):

```sh
Summary -P -R D:\Developer
```

#### --Exclude:

Exclude certain files or directories (defailt is
`node_modules target dist vendor`)

#### --Pattern:

Specify a custom pattern for matching (defailt is `.git`)

#### --Separator:

Define a custom separator.

#### --Omit:

List of regex to to match on files omitted from processing.

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

[Pieces OS] For extended functionality and system integration.

[Summary]: HTTPS://crates.io/crates/psummary
[Pieces OS]: HTTPS://Pieces.App

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a history of changes to this CLI.
