## 0.1.3

## 0.1.2

### Added

-   Added chrono = "0.4.38" as a new dependency

### Changed

-   Updated version number in Cargo.toml from 0.1.1 to 0.1.2

### Improved

-   Enhanced tag handling in Source/Fn/Summary.rs:
    -   Now sorts tags by commit date instead of alphabetically
    -   Improved date parsing and handling using chrono

## 0.1.1

### Changed

-   Updated version number in Cargo.toml from 0.1.0 to 0.1.1

### Improved

-   Enhanced diff processing in Source/Fn/Summary/Difference.rs:
    -   Simplified path handling for old and new files
    -   Improved content processing with better formatting
    -   Added support for file rename/move operations (indicated by 'F' origin)

### Developer Notes

-   The changes in Difference.rs should improve readability and maintainability
    of the diff processing code
-   The new handling of 'F' origin allows for better tracking of file renames
    and moves in the repository

## 0.1.0

### Added

-   New dependency: itertools = "0.13.0"
-   New module: Source/Fn/Summary/Group.rs for processing and printing summaries
    of differences

### Changed

-   Updated dependencies:
    -   tokio from 1.39.1 to 1.39.2
    -   toml from 0.8.16 to 0.8.17
-   Refactored Source/Fn/Binary/Command/Parallel.rs:
    -   Now uses FuturesUnordered for managing asynchronous tasks
    -   Changed Output data structure from DashMap to Vec
    -   Simplified entry collection and processing
-   Updated Source/Fn/Binary/Command/Sequential.rs:
    -   Now uses futures::future::join_all for sequential processing
    -   Adjusted result handling to match the new structure
-   Modified Source/Fn/Summary.rs:
    -   Updated format strings for summary messages
    -   Added new Group module for summary processing

### Improved

-   Enhanced parallel processing capabilities
-   Optimized memory usage in summary generation
-   Improved code readability and maintainability

### Developer Notes

-   The new Group module provides a more efficient way to aggregate and display
    differences
-   The switch to FuturesUnordered in the parallel processing should improve
    performance for large datasets
-   Developers should review the changes in Parallel.rs and Sequential.rs to
    understand the new processing flow

## 0.0.9

### Changed

-   Updated Cargo.toml to include specific files in the package:
    -   Added include = [ "Source/**/*", "LICENSE", "README.md", "CHANGELOG.md",
        "build.rs", "Cargo.toml", ] to specify which files should be included when
        packaging the crate

### Developer Notes

-   This change ensures that only necessary files are included in the published
    crate, potentially reducing package size and improving distribution
    efficiency
-   The inclusion of LICENSE, README.md, and CHANGELOG.md ensures that important
    documentation is bundled with the package
-   Including build.rs ensures that any custom build steps are properly executed
    when the crate is built by users

### Internal

-   Version bump from 0.0.8 to 0.0.9 (implied by the new CHANGELOG entry, though
    not explicitly shown in the diff)

## 0.0.8

### Changed

-   Updated version number in Cargo.toml from 0.0.7 to 0.0.8

### Added

-   Added dashmap = "6.0.1" as a new dependency

### Improved

-   Enhanced command-line interface in Source/Fn/Binary/Command.rs:
    -   Updated default values for the Omit argument to use case-insensitive
        patterns
    -   Simplified and improved regex patterns for file exclusion
-   Optimized diff generation in Source/Fn/Summary/Difference.rs:
    -   Improved file filtering logic using a single RegexSet instead of
        multiple individual regexes
    -   Enhanced performance of diff generation process
-   Refactored Source/Fn/Summary.rs:
    -   Improved error handling and return types
    -   Enhanced summary generation process using DashMap for concurrent access
-   Added new module Source/Fn/Summary/Insert.rs for handling summary insertions

### Documentation

-   Significantly improved documentation throughout the codebase:
    -   Added more detailed function descriptions
    -   Included usage examples in function documentation
    -   Clarified error handling and return types

### Internal Changes

-   Refactored internal structures and type definitions for better code
    organization
-   Updated various internal function signatures for improved consistency and
    type safety

### Developer Notes

-   This update focuses on performance improvements, code organization, and
    documentation enhancements
-   The new dashmap dependency allows for more efficient concurrent operations
    in summary generation
-   The license change should be reviewed to ensure compliance with the new
    licensing terms

## 0.0.7

### Changed

-   Updated version number in Cargo.toml from 0.0.6 to 0.0.7

### Improved

-   Enhanced output formatting in Source/Fn/Summary.rs:
    -   Changed println! format from "\n{}\n" to "{}" for cleaner output
    -   This change affects multiple sections of the summary output, including:
        -   Summary from first commit to last commit
        -   Summary between specified tags
        -   Summary from first commit to latest tag
        -   Summary from latest tag to last commit

### Developer Notes

-   These changes improve the readability of the summary output by removing
    unnecessary backticks and adjusting newline placement.
-   The core functionality remains the same, with only cosmetic improvements to
    the output format.

## 0.0.6

### Dependencies

-   Updated toml dependency from version 0.8.16 to 0.8 Changes
-   Improved error handling and messaging in repository operations
-   Enhanced summary generation to include comparisons between:
    -   First commit and last commit (when no tags are present)
    -   First commit and latest tag
    -   Latest tag and last commit
-   Added new module First.rs to handle retrieving the first commit in a
    repository
-   Refined README.md documentation for the --Omit or -O option

### Internal Improvements

-   Refactored Fn::Summary::Fn function for better handling of different commit
    comparison scenarios
-   Introduced Fn::Summary::First::Fn function to get the first commit in
    topological order
-   Version bump from 0.0.5 to 0.0.6

## 0.0.5

### Changes

-   Updated version number in Cargo.toml from 0.0.4 to 0.0.5
-   Improved README.md formatting and clarified feature descriptions
-   Enhanced Git repository analysis functionality

### Code Improvements

-   Refactored Fn::Summary::Difference::Option to
    Struct::Summary::Difference::Struct
-   Implemented parallel processing for file filtering using Rayon
-   Added extensive list of file extensions to omit from diff generation
-   Improved error handling in diff generation process
-   Optimized diff options for better performance and readability

### Bug Fixes

-   Fixed potential UTF-8 encoding issues in diff content handling

### Internal Changes

-   Reorganized project structure with new Struct module
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

-   New Omit parameter in Option struct for excluding specific files or
    directories
-   Improved parallel processing in Parallel.rs
-   Enhanced error handling and logging in Summary.rs
-   New regex-based file filtering in Difference.rs

### Changed

-   Updated build script (build.rs) for better dependency management
-   Refactored Summary.rs for improved performance and flexibility
-   Modified Command.rs to include new command-line options
-   Updated README.md with more detailed feature descriptions and usage
    instructions

### Removed

-   Removed Release.rs file, functionality likely integrated elsewhere

### Dependencies

-   Added regex = "1.10.5" to dependencies in Cargo.toml

### Notes

-   This update focuses on code cleanup, performance improvements, and enhanced
    file filtering capabilities
-   The Omit feature allows for more granular control over which files are
    processed
-   README.md now includes information about Pieces OS integration

## 0.0.2

### Changed

-   Updated version number from 0.0.1 to 0.0.2 in Cargo.toml

### Notes

-   This update represents a minor version bump
-   Referred to as "Version 2.0" in internal documentation

## 0.0.1

-   Initial version
