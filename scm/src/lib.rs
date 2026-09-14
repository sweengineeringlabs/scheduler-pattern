//! `scheduler_pattern` — the reusable time-based scheduling pattern.
//!
//! Provides this domain's primitives: the [`Scheduler`] trait (schedule a
//! job to run once after a delay, or repeatedly on a fixed interval; cancel
//! it), [`Trigger`] (when to run), [`JobId`] (an opaque handle to a
//! scheduled job), and [`SchedulerError`]. Zero implementation of any kind
//! — a single-responsibility interface package. Reference and any
//! technology-specific implementations live in `scheduler-svc` instead. A
//! consumer depends on this crate plus `scheduler-svc` and defines no
//! primitives of its own.
//!
//! Designed contract-first: unlike `executor-pattern` (extracted from a
//! real, existing `swe-edge-runtime-scheduler` implementation), no existing
//! pilot for time-based scheduling exists anywhere in this org yet. See
//! `docs/3-design/architecture.md` for the domain-modeling reasoning behind
//! each primitive, and for what was deliberately deferred (cron-expression
//! triggers) rather than spun into speculative machinery with no real
//! consumer to validate it against.
//!
//! This is a distinct domain from `executor-pattern` (which runtime drives
//! a future — no concept of time) and from `message-broker-pattern`'s
//! `TaskQueue` (enqueue work for a competing consumer to pick up
//! immediately — no concept of a future point in time either).

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod error;
mod traits;
mod types;
mod vo;

pub use error::SchedulerError;
pub use traits::Scheduler;
pub use types::Job;
pub use vo::{JobId, Trigger};
