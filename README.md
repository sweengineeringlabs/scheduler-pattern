# scheduler-pattern

> **TLDR:** The reusable time-based scheduling pattern — schedule a job to
> run once after a delay or repeatedly on a fixed interval, cancel it. Zero
> implementation. See [Architecture](docs/3-design/architecture.md) for the
> full design.

A single crate, not a workspace, not a scheduler. Implement `Scheduler` to
plug in any scheduling backend; this repo ships none — see
[`scheduler-svc`](https://github.com/sweengineeringlabs/scheduler-svc) for a
reference in-process implementation. A consumer depends on this crate plus
`scheduler-svc` and defines no primitives of its own.

Designed contract-first, with no existing pilot to extract from — see
[ADR-001](docs/3-design/adr/ADR-001-contract-first-domain-design.md) for the
domain-modeling reasoning.

## Quick Start

```rust
use std::sync::Arc;
use std::time::Duration;

use scheduler_pattern::{Scheduler, Trigger};

fn schedule_heartbeat(scheduler: &impl Scheduler) {
    let job = Arc::new(|| Box::pin(async {
        // ... do the work ...
        Ok(())
    }) as _);
    let _ = scheduler.schedule(Trigger::Every(Duration::from_secs(30)), job);
}
```

## Documentation

| Document | Description |
|----------|--------------|
| [Docs index](docs/README.md) | Full documentation index |
| [Architecture](docs/3-design/architecture.md) | Component diagram, dependency rationale |
| [ADR-001](docs/3-design/adr/ADR-001-contract-first-domain-design.md) | Why this crate is contract-first, what's deliberately deferred |
| [Developer Guide](docs/4-development/developer_guide.md) | Repo layout, branching, working on this crate |

## License

MIT OR Apache-2.0
