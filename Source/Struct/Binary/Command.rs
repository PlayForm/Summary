//! This module defines the main command structure and its implementation for the binary command execution.

/// Represents the main command structure for binary command execution.
pub struct Struct {
	/// The separator used for file paths.
	pub Separator: Option::Separator,

	/// A boxed function that returns a pinned future representing the command execution.
	pub Fn: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static>,
}

impl Struct {
	/// Creates a new instance of the Struct.
	///
	/// This function initializes the Struct with the system's main separator
	/// and a boxed async function that handles command execution.
	///
	/// # Returns
	///
	/// Returns a new instance of Struct.
	pub fn Fn() -> Self {
		Self {
			Separator: std::path::MAIN_SEPARATOR,
			Fn: Box::new(|| {
				Box::pin(async move {
					let Option = Entry::Struct::Fn(&Option::Struct::Fn(Struct::Fn()));

					match Option.Parallel {
						true => {
							Parallel::Fn(Option).await;
						}
						false => {
							Sequential::Fn(Option).await;
						}
					};
				})
			}),
		}
	}
}

use futures::Future;
use std::pin::Pin;

pub mod Entry;
pub mod Option;

use crate::Fn::Binary::Command::{Parallel, Sequential};
