# Changelog

All notable changes to this project are documented here. Format loosely follows
[Keep a Changelog](https://keepachangelog.com/) and the project uses
[Semantic Versioning](https://semver.org/).

## [1.0.0] — 2026-05-23

### Added
- Core CRUD: add, list, delete, mark-done.
- Search by keyword across title and description.
- Filter by priority and by due-date range.
- Sort by priority or by due date (Strategy pattern).
- Persistence via atomic-rename JSON writes.
- Export to JSON and CSV (Template Method pattern).
- Statistics: total / done / overdue / counts by priority.
- Factory Method for the three common task flavours.
- Command + UndoStack scaffolding (Command pattern).

### Engineering
- Black-box and white-box test suites; 70% coverage target.
- GitHub Actions CI: build, test, clippy (deny warnings), llvm-cov.
- TDD evidence for the sort feature (red → green → refactor commits).
- Refactoring report with before/after metrics.

## [0.1.0] — initial scaffolding

### Added
- Cargo project skeleton, README, .gitignore.
