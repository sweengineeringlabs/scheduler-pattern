# Glossary

Alphabetized list of terms used in `scheduler-pattern`.

---

**Job** - The callable unit of work a `Scheduler` runs: `Arc<dyn Fn() -> BoxFuture<'static, Result<(), SchedulerError>> + Send + Sync>`. `Fn`, not `FnOnce` — a recurring (`Trigger::Every`) job is invoked more than once.

**JobId** - Opaque handle to a scheduled job, wrapping a `Uuid`. Returned by `Scheduler::schedule`, passed to `Scheduler::cancel`.

**Scheduler** - Time-based scheduling contract: `schedule(trigger, job) -> JobId`, `cancel(job_id)`.

**SchedulerError** - Domain error type: `InvalidTrigger`, `ScheduleFailed`, `CancelFailed`, `JobNotFound(JobId)`, `JobFailed`.

**Trigger** - When a job should run: `Once(Duration)` (once, after a delay) or `Every(Duration)` (repeatedly, on an interval). No cron-expression syntax yet — see ADR-001.

[← Docs index](README.md)
