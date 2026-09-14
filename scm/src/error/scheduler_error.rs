//! [`SchedulerError`] — domain error type for the scheduler pattern.

use crate::JobId;

/// Errors that can occur while scheduling, running, or cancelling a job.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SchedulerError {
    /// The trigger could not be accepted (e.g. a zero-duration interval).
    #[error("invalid trigger: {0}")]
    InvalidTrigger(String),
    /// The job could not be scheduled.
    #[error("schedule failed: {0}")]
    ScheduleFailed(String),
    /// The job could not be cancelled.
    #[error("cancel failed: {0}")]
    CancelFailed(String),
    /// No job with this ID is currently scheduled.
    #[error("job not found: {0}")]
    JobNotFound(JobId),
    /// The job itself returned an error when it ran.
    #[error("job failed: {0}")]
    JobFailed(String),
}
