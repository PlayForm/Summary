/// Generates a release summary.
///
/// # Arguments
///
/// * `Option` - A struct containing the necessary information for generating the release summary.
///
/// # Returns
///
/// * `Return` - The generated release summary.
///
/// # Example
///
/// ```
/// let option = Option {
///     // Fields needed for release summary generation
/// };
/// let release_summary = Fn(&option);
/// ```
pub fn Fn(Difference: &str) -> String {
	let mut Release = String::new();

	Release.push_str("Release Notes:\n");

	for Difference in Difference.lines() {
		if Difference.starts_with("+") && !Difference.starts_with("+++") {
			Release.push_str(&format!("Added: {}\n", &Difference[1..]));
		} else if Difference.starts_with("-") && !Difference.starts_with("---") {
			Release.push_str(&format!("Removed: {}\n", &Difference[1..]));
		}
	}

	Release
}
