# scheduler-pattern Developer Guide

**Audience**: Developers, contributors.

## Repo Structure

```
scheduler-pattern/
├── README.md
├── docs/
│   ├── README.md
│   ├── glossary.md
│   ├── 0-ideation/papers/README.md
│   ├── 3-design/README.md, architecture.md
│   ├── 3-design/compliance/compliance_checklist.md
│   ├── 3-design/adr/README.md, ADR-001-contract-first-domain-design.md
│   └── 4-development/README.md, developer_guide.md   # this file
└── scm/
    ├── Cargo.toml          # [package] scheduler-pattern -- a single crate,
    │                       #  not a workspace
    ├── src/
    │   ├── lib.rs
    │   ├── traits/          # Scheduler
    │   ├── vo/               # Trigger, JobId
    │   ├── types/            # Job
    │   └── error/            # SchedulerError
    └── tests/                 # one *_int_test.rs per public trait/type
```

## Branching and Releases

- `dev` is the default branch; all work lands there first.
- `main` gets fast-forwarded to `dev` after a shipped change, not on every commit.
- Not yet published to crates.io. First publish will be v0.1.0.

## Working on This Crate

```
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

Dependency footprint is `futures` (`Job`'s `BoxFuture`), `thiserror` (error
boilerplate), `uuid` (`JobId`) — verify with `cargo tree --depth 1` after any
change that touches `Cargo.toml`. `#![deny(unsafe_code)]` and
`#![warn(missing_docs)]` are enforced.

## This Is a Contract-First Crate — No Existing Pilot

Unlike `message-broker-pattern`/`executor-pattern` (both extracted from
real, existing implementations), no prior art for time-based scheduling
exists anywhere in this org. Every primitive here was designed from domain
first principles — see
[ADR-001](../3-design/adr/ADR-001-contract-first-domain-design.md) for the
reasoning behind each one, and for what was deliberately scoped out
(cron-expression triggers, a `Validator` trait) rather than built
speculatively.

## See Also

- [Architecture](../3-design/architecture.md)
- [ADR-001](../3-design/adr/ADR-001-contract-first-domain-design.md)
- [Pattern/Svc Workflow](https://github.com/sweengineeringlabs/template-engine/blob/main/pattern_svc_workflow.md)
