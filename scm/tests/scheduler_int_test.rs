//! Integration tests for [`scheduler_pattern::Scheduler`].
//!
//! `scheduler_pattern` declares the trait only — no implementation lives
//! here (that's `scheduler-svc`'s job). These tests exercise the trait's
//! own contract via a minimal stub scheduler that actually tracks
//! scheduled jobs in a `HashMap`, proving `schedule`/`cancel` round-trip
//! correctly and that `JobNotFound` is real, not decorative.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use scheduler_pattern::{Job, JobId, Scheduler, SchedulerError, Trigger};

#[derive(Default)]
struct StubScheduler {
    scheduled: Mutex<HashMap<JobId, Trigger>>,
}

impl Scheduler for StubScheduler {
    fn schedule(&self, trigger: Trigger, _job: Job) -> Result<JobId, SchedulerError> {
        if let Trigger::Every(d) = trigger {
            if d.is_zero() {
                return Err(SchedulerError::InvalidTrigger(
                    "interval must be non-zero".to_string(),
                ));
            }
        }
        let id = JobId::new();
        self.scheduled
            .lock()
            .expect("lock poisoned")
            .insert(id, trigger);
        Ok(id)
    }

    fn cancel(&self, job_id: &JobId) -> Result<(), SchedulerError> {
        self.scheduled
            .lock()
            .expect("lock poisoned")
            .remove(job_id)
            .map(|_| ())
            .ok_or(SchedulerError::JobNotFound(*job_id))
    }
}

fn noop_job() -> Job {
    std::sync::Arc::new(|| Box::pin(async { Ok(()) }))
}

/// @covers: Scheduler::schedule — a valid Once trigger is accepted and
/// returns a real, usable JobId.
#[test]
fn test_schedule_once_returns_a_job_id() {
    let scheduler = StubScheduler::default();
    let id = scheduler
        .schedule(Trigger::Once(Duration::from_secs(5)), noop_job())
        .expect("schedule must succeed");
    assert!(!id.to_string().is_empty());
}

/// @covers: Scheduler::schedule — an invalid trigger is rejected, not
/// silently accepted.
#[test]
fn test_schedule_rejects_zero_duration_interval() {
    let scheduler = StubScheduler::default();
    let result = scheduler.schedule(Trigger::Every(Duration::ZERO), noop_job());
    assert!(matches!(result, Err(SchedulerError::InvalidTrigger(_))));
}

/// @covers: Scheduler::cancel — cancelling a real scheduled job succeeds.
#[test]
fn test_cancel_succeeds_for_scheduled_job() {
    let scheduler = StubScheduler::default();
    let id = scheduler
        .schedule(Trigger::Once(Duration::from_secs(5)), noop_job())
        .expect("schedule must succeed");
    assert!(scheduler.cancel(&id).is_ok());
}

/// @covers: Scheduler::cancel — cancelling an unknown/already-cancelled job
/// returns JobNotFound, not silently succeeding.
#[test]
fn test_cancel_returns_job_not_found_for_unknown_id() {
    let scheduler = StubScheduler::default();
    let unknown = JobId::new();
    assert!(matches!(
        scheduler.cancel(&unknown),
        Err(SchedulerError::JobNotFound(id)) if id == unknown
    ));
}

/// @covers: Scheduler::cancel — cancelling the same job twice fails the
/// second time (proves cancel actually removes state, not a no-op).
#[test]
fn test_cancel_twice_fails_the_second_time() {
    let scheduler = StubScheduler::default();
    let id = scheduler
        .schedule(Trigger::Once(Duration::from_secs(5)), noop_job())
        .expect("schedule must succeed");
    assert!(scheduler.cancel(&id).is_ok());
    assert!(matches!(
        scheduler.cancel(&id),
        Err(SchedulerError::JobNotFound(_))
    ));
}
