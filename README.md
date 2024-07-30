# 🗣️ [Summary] —

`Summary` is a powerful command-line tool designed for efficient file processing
and summarization. It offers both sequential and parallel processing
capabilities, along with flexible file filtering options.

[Summary]: HTTPS://crates.io/crates/psummary

## Feature

-   Directory traversal and file filtering
-   Parallel and sequential processing modes
-   Customizable file pattern matching
-   Exclusion of specified files or directories
-   Integration with [Pieces OS] for enhanced functionality

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

The `Summary` tool can be used with various options:

-   `--Root` or `-R`: Set the current working directory
-   `--Parallel` or `-P`: Run commands in parallel
-   `--Exclude`: Exclude certain files or directories
-   `--Pattern`: Specify a custom pattern for matching
-   `--Separator`: Define a custom separator

For [Pieces OS] integration, refer to the [Pieces OS] documentation for specific
flags and configuration options. [Pieces OS]

```sh
Summary
```

This command will generate summaries for all the `.git` tags inside the current
repository.

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
-   `rayon` - For parallel processing.
-   `tokio` - For asynchronous runtime.
-   `walkdir` - For efficient filesystem traversal.

[Pieces OS] For extended functionality and system integration.

[Summary]: HTTPS://crates.io/crates/psummary
[Pieces OS]: (HTTPS://Pieces.Appgit)

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a history of changes to this CLI.
