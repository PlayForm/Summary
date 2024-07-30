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
