//! The main entry point for the Summary application.

#![allow(non_snake_case)]

/// The main function that initializes and runs the `Summary` application.
///
/// # Errors
///
/// This function will return an error if there are issues parsing arguments
/// or executing the requested commands.
#[allow(dead_code)]
#[tokio::main]
async fn main() {
	(Struct::Binary::Command::Struct::Fn().Fn)().await
}

pub mod Fn;
pub mod Struct;
