//! [`Job`] — the unit of work a [`crate::Scheduler`] runs.

use futures::future::BoxFuture;

use crate::SchedulerError;

/// A callable unit of work, invoked once per firing of its [`crate::Trigger`].
///
/// `Fn`, not `FnOnce`: a [`Trigger::Every`](crate::Trigger::Every) job is
/// invoked repeatedly, so it must be callable more than once — unlike
/// `message-broker-pattern`'s `TaskQueue` (`Task` payloads are consumed
/// exactly once by a competing consumer) or `executor-pattern`'s `Executor`
/// (`run` takes one `FnOnce`-shaped future). `Send + Sync + 'static` so a
/// scheduler implementation can store and invoke it from any worker thread,
/// any number of times.
pub type Job = std::sync::Arc<
    dyn Fn() -> BoxFuture<'static, Result<(), SchedulerError>> + Send + Sync + 'static,
>;
