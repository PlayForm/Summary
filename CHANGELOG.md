## 0.0.7

## 0.0.6

### Dependencies

-   Updated `toml` dependency from version 0.8.16 to 0.8.17

### Changes

-   Improved error handling and messaging in repository operations
-   Enhanced summary generation to include comparisons between:
    -   First commit and last commit (when no tags are present)
    -   First commit and latest tag
    -   Latest tag and last commit
-   Added new module `First.rs` to handle retrieving the first commit in a
    repository
-   Refined README documentation for the `--Omit` or `-O` option

### Internal Improvements

-   Refactored `Fn::Summary::Fn` function for better handling of different
    commit comparison scenarios
-   Introduced `Fn::Summary::First::Fn` function to get the first commit in
    topological order
-   Version bump from 0.0.5 to 0.0.6

## 0.0.5

### Changes

-   Updated version number in Cargo.toml from 0.0.4 to 0.0.5
-   Improved README.md formatting and clarified feature descriptions
-   Enhanced Git repository analysis functionality

### Code Improvements

-   Refactored `Fn::Summary::Difference::Option` to
    `Struct::Summary::Difference::Struct`
-   Implemented parallel processing for file filtering using Rayon
-   Added extensive list of file extensions to omit from diff generation
-   Improved error handling in diff generation process
-   Optimized diff options for better performance and readability

### Bug Fixes

-   Fixed potential UTF-8 encoding issues in diff content handling

### Internal Changes

-   Reorganized project structure with new `Struct` module
-   Updated import statements to reflect new module structure
-   Improved code consistency and formatting throughout the project

## 0.0.4

### Changes

-   Updated version number in Cargo.toml from 0.0.3 to 0.0.4
-   Improved README.md with more detailed information about the tool's features
    and usage
-   Enhanced Git repository analysis capabilities
-   Refined description of Pieces OS integration
-   Added more detailed usage instructions and examples in README.md

### Features

-   Added support for generating diff logs between Git tags
-   Improved command-line options for better flexibility

### Documentation

-   Updated README.md with new features and usage instructions
-   Added more comprehensive examples in README.md
-   Clarified Pieces OS integration benefits

### Internal

-   Cleaned up and optimized internal code (based on the "Cleanup" comment in
    the previous CHANGELOG entry)

## 0.0.3

### Added

-   New `Omit` parameter in `Option` struct for excluding specific files or
    directories
-   Improved parallel processing in `Parallel.rs`
-   Enhanced error handling and logging in `Summary.rs`
-   New regex-based file filtering in `Difference.rs`

### Changed

-   Updated build script (`build.rs`) for better dependency management
-   Refactored `Summary.rs` for improved performance and flexibility
-   Modified `Command.rs` to include new command-line options
-   Updated README.md with more detailed feature descriptions and usage
    instructions

### Removed

-   Removed `Release.rs` file, functionality likely integrated elsewhere

### Dependencies

-   Added `regex = "1.10.5"` to dependencies in Cargo.toml

### Notes

-   This update focuses on code cleanup, performance improvements, and enhanced
    file filtering capabilities
-   The `Omit` feature allows for more granular control over which files are
    processed
-   README now includes information about Pieces OS integration

## 0.0.2

### Changed

-   Updated version number from 0.0.1 to 0.0.2 in Cargo.toml

### Notes

-   This update represents a minor version bump
-   Referred to as "Version 2.0" in internal documentation

## 0.0.1

-   Initial version
