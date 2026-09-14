//! [`Trigger`] — when a scheduled job should run.

use std::time::Duration;

/// When a scheduled job should run.
///
/// Deliberately minimal: fixed delay or fixed interval only, both directly
/// backed by `std::time::Duration` — no cron-expression syntax. Whether a
/// `Trigger::Cron(String)` variant (or a separate trait) is worth adding is
/// an open question, deferred until a real consumer needs calendar-based
/// scheduling ("every weekday at 9am") that a fixed interval can't express
/// — see `docs/3-design/architecture.md` for the full reasoning. Adding it
/// later is a backward-compatible enum addition, not a breaking redesign.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trigger {
    /// Run the job exactly once, after this delay from the moment it was
    /// scheduled.
    Once(Duration),
    /// Run the job repeatedly, waiting this long between the end of one run
    /// and the start of the next.
    Every(Duration),
}

impl Trigger {
    /// `true` for [`Trigger::Once`].
    #[must_use]
    pub fn is_once(&self) -> bool {
        matches!(self, Trigger::Once(_))
    }

    /// `true` for [`Trigger::Every`].
    #[must_use]
    pub fn is_recurring(&self) -> bool {
        matches!(self, Trigger::Every(_))
    }
}
