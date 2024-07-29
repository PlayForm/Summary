# 🗣️ [Summary] —

`Summary` is a command-line tool that executes commands in multiple directories
simultaneously. It leverages parallel processing and concurrent `I/O` to
efficiently run tasks across directories.

[Summary]: HTTPS://crates.io/crates/psummary

## Installation

```sh
cargo install psummary
```

## Usage

```sh
Summary
```

This command will fetch from upstream for all `.git` repositories inside the
current directory. It essentially replaces the following command:

```sh
find -iname .git -type d -execdir git fetch upstream \;
```

## Options

#### --Root or -R:

Set the current working directory to a different folder (default is `.`):

```sh
Summary -R D:\Developer .git git fetch upstream
```

#### --Parallel or -P:

Summary commands in `parallel` (default is `sequential`):

```sh
Summary -P -R D:\Developer .git git fetch upstream
```

#### --Exclude:

Exclude certain files or directories (defailt is
`node_modules target dist vendor`)

#### --Pattern:

Specify a custom pattern for matching (defailt is `.git`)

#### --Separator:

Define a custom separator

## Dependencies

`Summary` relies on several Rust crates to provide its functionality:

-   `clap` - Parses command-line arguments
-   `rayon` - Enables parallel processing
-   `tokio` - Provides an asynchronous runtime
-   `walkdir` - Facilitates efficient filesystem traversal

[Summary]: HTTPS://crates.io/crates/psummary

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a history of changes to this CLI.
