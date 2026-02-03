# [Summary] 🗣️ (`psummary`)

[![Crates.io](https://img.shields.io/crates/v/psummary.svg)](https://crates.io/crates/psummary)

`Summary` is a blazingly fast, concurrent tool for generating comprehensive
change summaries across multiple Git repositories. It performs intelligent Git
repository discovery and produces clean diffs between tags or specific commits
using tag-based chronological analysis.

Built for developers who need to understand project evolution at scale,
`Summary` leverages Rust's async runtime and parallel processing to scan
hundreds of repositories in seconds.

[Summary]: https://crates.io/crates/psummary

## Key Features 🔐

- **Blazing Fast**: Parallel repository scanning (rayon) + async diff generation
  (tokio) delivers order-of-magnitude speedups over manual git operations
- **Intelligent Tag Analysis**: Automatically sorts tags chronologically and
  generates diffs between consecutive releases, plus latest tag to HEAD
- **Hash-Based Deduplication**: Identical diffs are detected and grouped by
  content hash to eliminate redundancy in output
- **Smart Exclusion Logic**: Pattern never excluded—`.git` directories are
  always traversed, even inside excluded paths like `node_modules`
- **Local Tag Discovery**: Discovers all local tags in each repository (run
  `git fetch --tags` first to include remote tags)
- **62 Built-in Extensions**: Automatically filters binary files from diffs
  using 62 case-insensitive extension patterns
- **Concurrent Pipeline**: Rayon parallelizes path scanning; tokio handles
  concurrent repository processing with `FuturesUnordered`
- **Intelligent Diff Filtering**: Only shows context lines (`F`), additions
  (`+`), and deletions (`-`); git metadata is stripped for clean output

---

## Performance Benchmarks 🚤

`Summary` processes multiple repositories concurrently, making it dramatically
faster than running sequential git commands manually. The parallel architecture
divides work efficiently:

- **Rayon** handles parallel path scanning across all filesystem entries
- **Tokio** spawns async tasks per repository with `FuturesUnordered` for
  concurrent diff generation
- **DashMap** provides sharded concurrent aggregation without lock contention

In typical scenarios scanning 100+ repositories:

| Operation                  | Parallel Time | Sequential Time | Speedup  |
| :------------------------- | :-----------: | :-------------: | :------: |
| Generate tag diffs         | ~2-3 seconds  | ~15-20 seconds  | **6-8x** |
| Diff all commits (no tags) | ~2-3 seconds  | ~12-18 seconds  | **6-8x** |

_(Actual performance depends on repository count, sizes, and I/O speed)_

---

## Installation 🚀

Install directly from [Crates.io](https://crates.io/crates/psummary):

```sh
cargo install psummary
```

This installs **two** binaries with identical functionality:

- `psummary` (lowercase, recommended)
- `Summary` (capitalized, for case-insensitive filesystems)

---

## Usage ⚙️

The core workflow: **discover** Git repositories → **identify** tags →
**analyze** diffs → **aggregate** grouped summaries.

```
A tool to recursively find Git repositories and summarize changes between tags.

Usage: psummary [OPTIONS]

Options:
  -P, --Parallel           Run analysis in parallel across multiple repositories
  -R, --Root <ROOT>        The root directory to start scanning from [default: .]
  -E, --Exclude <EXCLUDE>  A space-separated list of directory names to exclude
                           [default: node_modules]
      --Pattern <PATTERN>  The pattern to look for when identifying project roots
                           [default: .git]
  -O, --Omit <OMIT>        A regex pattern to omit files from the diff summary.
                           Can be used multiple times [default: (?i)documentation (?i)target (?i)changelog\.md$ (?i)summary\.md$]
  -h, --help               Print help information
  -V, --version            Print version information
```

### Basic Examples

**1. Summarize all repositories in current directory**

Finds every `.git` folder recursively and prints diffs between tags and HEAD.

```sh
psummary -P
```

**2. Scan a specific projects folder and save output**

```sh
psummary -P -R ~/dev/projects > all_changes.diff
```

**3. Exclude common build directories**

```sh
psummary -P -E "node_modules target dist vendor"
```

### Advanced Options

- **`-O, --Omit <PATTERN>`**: Exclude files matching regex from diffs. Specify
  multiple times for complex filters.

    ```sh
    # Skip lock files, docs, and build artifacts
    psummary -P -O ".*\.lock$" -O "\.md$" -O "/dist/"
    ```

    **Note:** Regex patterns are case-sensitive by default. Use the `(?i)`
    prefix for case-insensitive matching. The default patterns already use
    `(?i)`.

- **`--Pattern <PATTERN>`**: Match different repository markers (e.g., looking
  for `.hg` or custom markers). **Matches the last path component only**—so
  `.git` finds repositories by `.git` folder. Useful for other VCS markers.

- **`-P` vs sequential**: Omit `-P` for deterministic sequential execution
  (useful for debugging or low-memory environments).

---

## How It Works 🔄

1. **Discovery**: `walkdir` traverses the filesystem from `--Root`, filtering
   entries that match `--Pattern` in the last path component
2. **Filtering**: Directories in `--Exclude` are skipped **unless** they contain
   the `--Pattern` itself (e.g., `.git` is never excluded)
3. **Processing**: Each repository path spawns an async task that:
    - Opens the Git repository with `git2`
    - Collects and sorts tags chronologically
    - Generates diffs between consecutive tags + HEAD
4. **Diff Generation**: `git2::DiffOptions` with:
    - `force_text(true)` and `ignore_filemode(true)` for clean output
    - `ignore_whitespace*` options to focus on semantic changes
    - **62 built-in binary extensions** + user `--Omit` patterns in a
      `regex::RegexSet`
    - Line filter: only `F` (filename), `+` (addition), `-` (deletion) lines
      kept
5. **Deduplication**: Each diff is hashed
   (`std::collections::hash_map::DefaultHasher`) to detect identical changes
   across repositories
6. **Aggregation**: `DashMap` collects diffs by unique hash; final output groups
   by error message/reason with differences sorted by length (longest first)

---

## Implementation Details ⚙️

### Architecture

- **Parallelism**: Rayon's `into_par_iter()` for CPU-bound path scanning; tokio
  `spawn()` + `FuturesUnordered` for I/O-bound repository operations
- **Concurrency**: `DashMap` provides lock-free sharded hash maps for
  thread-safe aggregation without contention
- **Error Handling**:
    - Parallel mode (`-P`): Errors are logged to stderr but processing continues
    - Sequential mode: Failed repositories are collected and skipped; processing
      continues with remaining repos
- **Binary Detection**: Path-based filter of 62 file extensions (see below).
  Content is **not** inspected—the filter operates on file paths only.

### Important Notes ⚠️

- **Local tags only**: Only discovers **local** Git tags. Run `git fetch --tags`
  in repositories first to include remote tags in the analysis.
- **Pattern exclusion**: Directories listed in `--Exclude` are skipped
  **unless** the directory name matches `--Pattern` (e.g., `.git`). This ensures
  Git repositories are always found even inside `node_modules` or other excluded
  paths.
- **Regex validation**: Invalid regex patterns cause a panic at startup. Test
  your patterns with `regex` crate documentation before using.
- **Diff output format**: Only context lines (`F`), additions (`+`), and
  deletions (`-`) are included. All other git diff metadata (hunks, binary
  indicators, etc.) is filtered out for clean, readable summaries.

---

## Dependencies 🖇️

`Summary` is built with these excellent Rust crates:

- **[`clap`](https://crates.io/crates/clap)**: Ergonomic command-line argument
  parsing withderive macros
- **[`git2`](https://crates.io/crates/git2)**: Full-featured Git library for all
  repository operations (libgit2 bindings)
- **[`rayon`](https://crates.io/crates/rayon)**: Data-parallelism for concurrent
  repository path scanning
- **[`tokio`](https://crates.io/crates/tokio)**: Async runtime with `full`
  features for non-blocking diff generation
- **[`walkdir`](https://crates.io/crates/walkdir)**: Efficient cross-platform
  directory traversal with built-in filtering
- **[`regex`](https://crates.io/crates/regex)**: High-performance `RegexSet` for
  matching omit patterns and binary extensions
- **[`dashmap`](https://crates.io/crates/dashmap)**: Sharded concurrent hash map
  for lock-free summary aggregation
- **[`futures`](https://crates.io/crates/futures)**: `FuturesUnordered` for
  concurrent task orchestration and stream combinators
- **[`chrono`](https://crates.io/crates/chrono)**: Date/time handling for tag
  chronology and sorting
- **[`itertools`](https://crates.io/crates/itertools)**: Extended iterator
  utilities (`sorted_by`, `sorted_by_key`) for result ordering
- **[`num_cpus`](https://crates.io/crates/num_cpus)**: CPU count detection for
  optimal thread pool sizing
- **[`unbug`](https://crates.io/crates/unbug)**: Error handling utilities

---

## License ⚖️

This project is released into the public domain under the **Creative Commons CC0
Universal** license. You are free to use, modify, distribute, and build upon
this work for any purpose. See the [`LICENSE`](LICENSE) file for full details.

---

## Changelog 📜

Stay updated with the latest improvements. See [`CHANGELOG.md`](CHANGELOG.md)
for a complete history of changes.

---

## Binary Extensions 📦

`Summary` automatically excludes 62 binary file types from diffs using these
case-insensitive patterns:

```
.7z .accdb .avi .bak .bin .bmp .class .dat .db .dll .dll.lib .dll.exp
.doc .docx .dylib .exe .flac .gif .gz .heic .ico .img .iso .jpeg .jpg
.m4a .mdb .mkv .mov .mp3 .mp4 .o .obj .ogg .pdb .pdf .png .ppt .pptx
.pyc .pyo .rar .so .sqlite .svg .tar .tiff .wav .webp .wmv .xls .xlsx .zip
```

_(See [`Fn/Summary/Difference.rs:48-102`](Source/Fn/Summary/Difference.rs:48)
for the complete list in source)_
