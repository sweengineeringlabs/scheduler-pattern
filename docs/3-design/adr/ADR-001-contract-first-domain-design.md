# ADR-001: Contract-first domain design, no existing pilot

**Status**: Accepted
**Date**: 2026-09-14

## Context

Unlike `executor-pattern` (extracted from `edge/scheduler`'s real,
already-shipping `swe-edge-runtime-scheduler`), no existing implementation
of time-based/cron scheduling exists anywhere in this org. This crate is
designed contract-first: model the domain soundly from first principles,
rather than waiting for an accidental existing implementation to extract
from. See
[Pattern/Svc Workflow](https://github.com/sweengineeringlabs/template-engine/blob/main/pattern_svc_workflow.md)'s
own correction on this point — a real implementation to extract from is not
a prerequisite for a sound contract.

"Contract-first" is not license for unconstrained scope, though: every
primitive below is scoped to what's needed to express "run this once after
a delay, or repeatedly on an interval, and be able to cancel it" — the
smallest real, well-understood scheduling domain — not an attempt to
anticipate every possible scheduling need up front.

## Decision

- **`Trigger`**: `Once(Duration)` / `Every(Duration)` only. No
  `Cron(String)` variant, no calendar-based scheduling ("every weekday at
  9am"). Both real, well-understood shapes backed directly by
  `std::time::Duration` — no need for a new value type. Cron-expression
  syntax is a materially different feature (parsing, timezone handling,
  DST edge cases) that no real consumer has asked for yet; adding a variant
  later is backward-compatible, so deferring it isn't a wall this decision
  builds — it's declining to guess at syntax nobody has validated yet.
- **`Job`**: `Arc<dyn Fn() -> BoxFuture<'static, Result<(), SchedulerError>> + Send + Sync>`.
  `Fn`, not `FnOnce` — a `Trigger::Every` job must be callable more than
  once, unlike `message-broker-pattern`'s `Task` (consumed exactly once) or
  `executor-pattern`'s `Executor::run` (one `FnOnce`-shaped future per
  call).
- **`JobId`**: opaque `Uuid` wrapper, mirroring `message-broker-pattern`'s
  own `TaskId` exactly (same precedent, same reasoning: a scheduler needs
  something to hand back from `schedule()` and take in `cancel()` without
  exposing implementation-internal state).
- **`SchedulerError`**: `InvalidTrigger`, `ScheduleFailed`, `CancelFailed`,
  `JobNotFound(JobId)`, `JobFailed`. Five variants covering the two entry
  points (`schedule`/`cancel`) plus the job's own failure surfacing back
  through the same error type — no separate error type for job execution
  failures, since a caller handling `SchedulerError` shouldn't need to
  match two different types depending on which part of the lifecycle
  failed.
- **No `Validator`/config-validation trait** in this crate, unlike
  `message-broker-pattern`'s `Validator` or `executor-pattern`'s
  `Validator`. No backend config type exists yet to validate — adding this
  speculatively, before `scheduler-svc` has a real backend with real config
  fields, would be exactly the premature-generalization trap this ADR's
  own "contract-first is not unconstrained scope" principle warns against.
  Add it when `scheduler-svc`'s first real backend needs it, following
  `executor-pattern`'s own `Validator` as the shape to match.

## Consequences

- `scheduler-pattern` depends on `futures` (for `Job`'s `BoxFuture`),
  `thiserror` (error boilerplate), and `uuid` (`JobId`) — the same
  dependency-footprint reasoning `message-broker-pattern`'s own
  architecture doc gives for each of these.
- `scheduler-svc` (companion repo) starts with a `core` reference
  implementation only — no `spi` backend yet, since no external scheduling
  technology (a persistent job store, a distributed cron service) has a
  real consumer demanding it. See that repo's own architecture.md.
- Every primitive here should be revisited once `scheduler-svc` has a real
  backend and `scheduler-pattern` has a real consumer outside this org's
  own repos — contract-first design is a starting point verified by later
  real use, not a one-shot guess treated as final.
