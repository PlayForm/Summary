pub fn Fn<T: Hash>(Input: &T) -> u64 {
	let mut Output = DefaultHasher::new();
	Input.hash(&mut Output);
	Output.finish()
}

use std::{
	collections::hash_map::DefaultHasher,
	hash::{Hash, Hasher},
};
