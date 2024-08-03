## 0.1.3

### Changed

-   Updated version number in Cargo.toml from 0.1.2 to 0.1.3

### Improved

-   Enhanced error handling in Source/Fn/Binary/Command/Parallel.rs:
    -   Changed error messages to be more consistent and informative
-   Refined tag handling in Source/Fn/Summary.rs:
    -   Reversed the order of processing tags (now processes from latest to
        earliest)
    -   Simplified the logic for generating summaries between tags

## 0.1.2

### Added

-   Added chrono = "0.4.38" as a new dependency

### Changed

-   Updated version number in Cargo.toml from 0.1.1 to 0.1.2
-   Updated dependencies:
    -   clap from 4.5.11 to 4.5.13
    -   regex from 1.10.5 to 1.10.6
    -   toml from 0.8.17 to 0.8.19

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

(The rest of the changelog remains unchanged)
