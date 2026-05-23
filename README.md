# rust-task-tracker

[![CI](https://github.com/OWNER/REPO/actions/workflows/ci.yml/badge.svg)](https://github.com/OWNER/REPO/actions/workflows/ci.yml)

A command-line personal task tracker written in Rust. Built as the term project
for the Software Engineering course (3rd-year undergraduate). The application
itself is intentionally simple — the goal is to demonstrate professional
software engineering practices end-to-end: version control hygiene, unit and
acceptance testing, TDD, CI, design patterns, refactoring, and metrics.

## Features

- Create, list, update, delete tasks (CRUD)
- Search by keyword across title and description
- Filter by priority and by due-date range
- Sort by due date or by priority (Strategy pattern)
- Persistence to JSON between sessions (atomic-rename writes)
- Export to JSON and CSV (Template Method)
- Statistics: counts by priority, overdue items, completion rate
- Factory Method for the common task flavours
- Undo / redo scaffolding (Command pattern, UndoStack)

## Build & run

```sh
cargo build --release
cargo run -- --help
cargo run -- add "buy bread" --priority high --due 2026-06-01
cargo run -- list --sort priority
cargo run -- search bread
cargo run -- export json --out tasks.json
cargo run -- stats
```

## Run tests

```sh
cargo test
cargo clippy --all-targets
```

## Coverage

```sh
cargo install cargo-llvm-cov  # one-time
cargo llvm-cov --summary-only
```

## Documentation

- [`docs/TEST_PLAN.md`](docs/TEST_PLAN.md) — testing strategy and inventory
- [`docs/TDD_EVIDENCE.md`](docs/TDD_EVIDENCE.md) — red/green/refactor cycle
- [`docs/ESTIMATION.md`](docs/ESTIMATION.md) — story points vs actuals
- [`docs/DESIGN_PATTERNS.md`](docs/DESIGN_PATTERNS.md) — patterns used, with rationale
- [`docs/REFACTORING_REPORT.md`](docs/REFACTORING_REPORT.md) — smells, fixes, metrics

## License

MIT.
