/// Represents the main command structure for the Summary application.
pub struct Struct {
	/// The path separator character.
	pub Separator: Option::Separator,
	/// A boxed function that returns a pinned future.
	pub Fn: Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> + Send + 'static>,
}

impl Struct {
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
							Sequential::Fn(Option);
						}
					};
				})
			}),
		}
	}
}

use crate::Fn::Binary::Command::{Parallel, Sequential};

use futures::Future;
use std::pin::Pin;

pub mod Entry;
pub mod Option;
