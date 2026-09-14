//! [`Scheduler`] — time-based scheduling contract.

use crate::{Job, JobId, SchedulerError, Trigger};

/// Schedules jobs to run once after a delay, or repeatedly on a fixed
/// interval.
///
/// Implement this trait to plug in any scheduling backend — an in-process
/// timer wheel, or a persistent/distributed one. `scheduler-svc` ships a
/// ready-made in-process implementation; alternative backends are added as
/// their own `spi` crate there.
pub trait Scheduler {
    /// Schedule `job` to run according to `trigger`, returning a [`JobId`]
    /// that can later be passed to [`Scheduler::cancel`].
    ///
    /// # Errors
    ///
    /// Returns [`SchedulerError::InvalidTrigger`] if `trigger` cannot be
    /// honored (e.g. a zero-duration interval), or
    /// [`SchedulerError::ScheduleFailed`] if the backend itself could not
    /// accept the job.
    fn schedule(&self, trigger: Trigger, job: Job) -> Result<JobId, SchedulerError>;

    /// Cancel a previously scheduled job. For a [`Trigger::Once`] job that
    /// has already run, or a [`Trigger::Every`] job mid-run, this stops
    /// *future* firings — it does not interrupt a run already in progress.
    ///
    /// # Errors
    ///
    /// Returns [`SchedulerError::JobNotFound`] if `job_id` is not currently
    /// scheduled (already cancelled, already completed as a `Once` job, or
    /// never existed), or [`SchedulerError::CancelFailed`] if the backend
    /// itself failed to process the cancellation.
    fn cancel(&self, job_id: &JobId) -> Result<(), SchedulerError>;
}
