pub fn Fn(Summary: &DashMap<u64, (String, String)>, Difference: String, Message: String) {
	Summary.insert(Hash::Fn(&Difference), (Difference, Message));
}

use dashmap::DashMap;

pub mod Hash;
