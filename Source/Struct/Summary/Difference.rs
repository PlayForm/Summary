/// Represents a structure containing omit patterns.
///
/// This struct holds a vector of strings representing patterns to omit.
///
/// # Fields
///
/// * `Omit` - A vector of strings representing patterns to omit.
///
/// # Example
///
/// ```rust
/// let omit_patterns = Struct {
///     Omit: vec!["pattern1".to_string(), "pattern2".to_string()],
/// };
/// ```
pub struct Struct {
	/// A vector of strings representing patterns to omit.
	pub Omit: Vec<String>,
}
