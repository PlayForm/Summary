/// Represents the structure for binary command execution.
///
/// This struct holds various fields related to the command execution, including the separator for file paths
/// and a function to execute the command asynchronously.
pub struct Struct {
	/// The separator used for file paths.
	pub Separator: Option::Separator,

	/// A boxed asynchronous function that returns a pinned future.
	pub Fn: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static>,
}

impl Struct {
	/// Creates a new instance of the Struct.
	///
	/// This function initializes the Struct with the default file path separator and an asynchronous function
	/// that executes the command based on the provided options. The function determines whether to execute
	/// the command in parallel or sequentially based on the `Parallel` flag in the options.
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

					// match Option.Parallel {
					// 	true => {
					// 		Parallel::Fn(Option).await;
					// 	}
					// 	false => {
					// 		Sequential::Fn(Option).await;
					// 	}
					// };

					apis::well_known_api::get_well_known_health(
						&apis::configuration::Configuration {
							base_path: "http://localhost:1000".to_string(),
							user_agent: Option {},
							client: reqwest::Client,
							basic_auth: Option {},
							oauth_access_token: Option {},
							bearer_access_token: Option {},
							api_key: Option {},
						},
					)
					.await;
				})
			}),
		}
	}
}

use futures::Future;
use git2::opts::get_mwindow_file_limit;
use std::pin::Pin;

pub mod Entry;
pub mod Option;

use crate::Fn::Binary::Command::{Parallel, Sequential};
use pieces_os_client::*;
