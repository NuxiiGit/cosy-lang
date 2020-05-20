pub mod diagnostics;

use diagnostics::IssueTracker;

/// A struct which stores session information, such as:
/// - Source code
/// - Errors
#[derive(Default)]
pub struct Session {
	/// The source code of the script you want o compile.
	pub src : String,
	/// Used to log any errors encountered during the session.
	pub issues : IssueTracker
}
impl From<String> for Session {
	fn from(src : String) -> Self {
		Self {
			src,
			issues : IssueTracker::new()
		}
	}
}