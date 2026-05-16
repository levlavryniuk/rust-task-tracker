# rust-task-tracker

A command-line personal task tracker written in Rust. Built as the term project
for the Software Engineering course (3rd-year undergraduate). The application
itself is intentionally simple — the goal is to demonstrate professional
software engineering practices end-to-end: version control hygiene, unit and
acceptance testing, TDD, CI, design patterns, refactoring, and metrics.

## Features

- Create, list, update, delete tasks (CRUD)
- Search by keyword across title and description
- Filter by priority and by due-date range
- Sort by due date or by priority
- Persistence to JSON between sessions
- Export to JSON and CSV
- Statistics: counts by priority, overdue items, completion rate
- Undo / redo of mutating operations (Command pattern)

## Build & run

```sh
cargo build --release
cargo run -- --help
```

Data is persisted to `tasks.json` in the working directory by default; override
with `--data-file <path>`.

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

## Project layout

See [Section 5 of the spec](docs/) — `src/` is split into `models/`,
`services/`, `storage/`, `patterns/`, and `cli/`.

## Documentation

- [`docs/TEST_PLAN.md`](docs/TEST_PLAN.md)
- [`docs/TDD_EVIDENCE.md`](docs/TDD_EVIDENCE.md)
- [`docs/ESTIMATION.md`](docs/ESTIMATION.md)
- [`docs/DESIGN_PATTERNS.md`](docs/DESIGN_PATTERNS.md)
- [`docs/REFACTORING_REPORT.md`](docs/REFACTORING_REPORT.md)

## License

MIT.
