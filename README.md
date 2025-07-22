# `Summary` 🗣️

[![Crates.io](https://img.shields.io/crates/v/psummary.svg)](https://crates.io/crates/psummary)

`Summary` is a powerful command-line tool that recursively scans directories for
Git repositories and generates concise summaries of the changes within them. It
intelligently creates diffs between tags or, for untagged repositories, between
the first and last commits.

It's designed for developers who want a quick overview of progress across
multiple projects, helping to generate release notes, track changes, or analyze
development history efficiently.

```sh
# Analyze all git repositories in the current directory and save the output
psummary -P > change_summary.txt
```

### Key Functionality

`Summary` operates with the following logic:

1.  **Repository Discovery:** It walks the specified directory tree (defaulting
    to the current location) and identifies all Git repositories by looking for
    `.git` folders.
2.  **Tag Analysis:** For each repository, it inspects the existing tags and
    sorts them chronologically.
3.  **Diff Generation:**
    - **If tags are present:** It generates a diff for each period between
      consecutive tags (e.g., `v1.0.0` -> `v1.1.0`). It also creates a summary
      from the latest tag to the current `HEAD`.
    - **If no tags exist:** It generates a single, comprehensive diff from the
      very first commit to the final commit in the repository.
4.  **Intelligent Filtering:** It automatically excludes numerous binary file
    types from the diffs and allows for custom exclusion of directories and file
    patterns.
5.  **Grouped Output:** The final output groups all changes by their respective
    repository and commit/tag range, providing a clean, organized report.

## ✨ Features

- **Recursive Git Repository Discovery:** Automatically find and analyze all
  repositories within a given path.
- **Automatic Change Summarization:** Generates diffs between consecutive tags
  and from the last tag to `HEAD`.
- **Parallel Processing:** Utilizes multiple CPU cores to analyze repositories
  in parallel for maximum speed (`-P` flag).
- **Flexible Directory Filtering:** Exclude specific directories like
  `node_modules` or `target` from the initial scan.
- **Regex-Based File Omission:** Use regular expressions to omit certain files
  (e.g., `*.md`, `CHANGELOG.md`) from the diff generation.
- **Smart Binary Exclusion:** By default, ignores common binary file extensions
  (`.png`, `.zip`, `.exe`, etc.) to keep summaries focused on source code.
- **Cross-Platform:** Built with Rust, it runs on Windows, macOS, and Linux.

## 🚀 Installation

You can install `Summary` directly from `crates.io` using `cargo`.

```sh
cargo install psummary
```

Make sure that your `~/.cargo/bin` directory is in your system's `PATH`.

## 🛠️ Usage

### Command-Line Options

Here is the full set of options available for the `Summary` command:

```
A tool to recursively find Git repositories and summarize changes between tags.

Usage: psummary [OPTIONS]

Options:
  -P, --Parallel
          Run analysis in parallel across multiple repositories for speed

  -R, --Root <ROOT>
          The root directory to start scanning from
          [default: .]

  -E, --Exclude <EXCLUDE>
          A space-separated list of directory names to exclude from the scan
          [default: node_modules]

      --Pattern <PATTERN>
          The pattern to look for when identifying project roots
          [default: .git]

  -O, --Omit <OMIT>
          A regex pattern to omit files from the diff summary. Can be used multiple times
          [default: (?i)documentation (?i)target (?i)changelog\.md$ (?i)summary\.md$]

  -h, --help
          Print help information

  -V, --version
          Print version information
```

### Option Details

- `--Parallel` or `-P`: Enables multi-threaded processing. This is highly
  recommended when scanning a directory with many repositories.
- `--Root` or `-R`: Specifies the starting directory for the scan. If not
  provided, it defaults to the current working directory.
- `--Exclude` or `-E`: Prevents the tool from scanning any directory whose path
  contains one of the specified strings. The default value is `node_modules`. To
  exclude multiple directories, wrap them in quotes:
  `-E "node_modules target dist"`.
- `--Omit` or `-O`: This powerful option uses regular expressions to filter out
  files _from the diff summary_. It can be specified multiple times. For
  example, to ignore all Markdown and text files, you would use
  `-O "\.md$" -O "\.txt$"`.

## 💡 Examples

#### 1. Analyze All Repositories in the Current Directory

Run a parallel scan on the current folder and print the output to the console.

```sh
psummary -P
```

#### 2. Analyze a Specific Development Folder and Save to a File

Scan a folder named `~/dev/projects` and save the complete summary to
`summary.diff`.

```sh
psummary -P -R ~/dev/projects > summary.diff
```

#### 3. Exclude Multiple Directory Names

Scan the current directory but ignore any paths containing `node_modules`,
`target`, or `vendor`.

```sh
psummary -P -E "node_modules target vendor"
```

#### 4. Omit Specific File Patterns from Diffs

Analyze all repositories but exclude any changes to lock files (`*.lock`),
Markdown files (`*.md`), and build artifacts in a `dist` directory from the
summaries.

```sh
psummary -P -O ".*\.lock$" -O "\.md$" -O "/dist/"
```

## Dependencies

`Summary` is built in Rust and stands on the shoulders of these excellent
crates:

- `clap` - For robust command-line argument parsing.
- `git2` - For all Git repository operations.
- `rayon` & `tokio` - For parallel and asynchronous execution.
- `walkdir` - For efficient directory traversal.
- `regex` - For pattern matching in the `--Omit` filter.
- `futures` - For managing asynchronous tasks.
- `num_cpus` - For determining the number of available CPU cores.

## Changelog

For a detailed history of changes, please see the [`CHANGELOG.md`](CHANGELOG.md)
file.
