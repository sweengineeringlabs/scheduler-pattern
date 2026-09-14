//! [`JobId`] — unique identifier for a scheduled job.

use std::fmt;

use uuid::Uuid;

/// Opaque handle to a job scheduled via [`crate::Scheduler::schedule`].
///
/// Returned by `schedule`, passed back to
/// [`Scheduler::cancel`](crate::Scheduler::cancel). Carries no scheduling
/// state itself — implementations own that.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JobId(Uuid);

impl JobId {
    /// Generate a fresh, unique [`JobId`].
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for JobId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for JobId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
