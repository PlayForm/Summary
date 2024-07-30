pub fn Fn(Summary: &DashMap<u64, (String, String)>, Difference: String, Message: String) {
	Summary.insert(Calculate::Fn(&Difference), (Difference, Message));
}

use dashmap::DashMap;

pub mod Calculate;
