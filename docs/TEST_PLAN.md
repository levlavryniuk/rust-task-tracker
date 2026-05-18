# Test Plan

## Strategy

The codebase is exercised at three levels:

1. **Unit tests in the model crate** — small, fast checks of `Task`, `Priority`,
   and `TaskId` parsing.
2. **Black-box tests** (`tests/blackbox_service.rs`) — derived from the
   functional requirements in the README without reference to the internal
   control flow. Each test maps to a requirement number (BB-1 … BB-5+).
3. **White-box tests** (`tests/whitebox_service.rs`) — derived from internal
   code structure, targeting branches in `TaskService::filter`,
   `Task::is_overdue`, and `TaskService::resolve_prefix`. Technique used:
   **branch coverage**. Each independent boolean branch in `filter()` is taken
   in isolation, plus a combined case where all three are active.
4. **CLI integration tests** (added on a later branch) using `assert_cmd` to
   drive the actual binary end-to-end.
5. **BDD-style acceptance tests** (`tests/acceptance.rs`) using
   Given-When-Then comments and helper functions.

## Coverage target

Target: ≥70% line coverage of business logic (`src/models/`, `src/services/`,
`src/storage/`). Measured with `cargo llvm-cov`.

```sh
cargo install cargo-llvm-cov          # one-time
cargo llvm-cov --summary-only         # quick line/branch summary
cargo llvm-cov --html                 # ./target/llvm-cov/html/index.html
```

The `cli` and `main.rs` glue are excluded from the target because their
behaviour is verified by the integration tests, not by line-coverage counting.

## Test inventory

| ID    | File                          | Type        | Targets                          |
|-------|-------------------------------|-------------|----------------------------------|
| BB-1  | blackbox_service.rs           | black-box   | add → list                       |
| BB-2  | blackbox_service.rs           | black-box   | delete                           |
| BB-3  | blackbox_service.rs           | black-box   | search across description        |
| BB-4  | blackbox_service.rs           | black-box   | filter by priority               |
| BB-5  | blackbox_service.rs           | black-box   | empty-title rejection            |
| BB-6  | blackbox_service.rs           | black-box   | persistence round-trip           |
| WB-1  | whitebox_service.rs           | white-box   | filter — priority branch         |
| WB-2  | whitebox_service.rs           | white-box   | filter — `from` branch           |
| WB-3  | whitebox_service.rs           | white-box   | filter — `to` branch             |
| WB-4  | whitebox_service.rs           | white-box   | filter — all branches            |
| WB-5  | whitebox_service.rs           | white-box   | is_overdue true branch           |
| WB-6  | whitebox_service.rs           | white-box   | is_overdue false via `done`      |
| WB-7  | whitebox_service.rs           | white-box   | resolve_prefix ambiguous → None  |
| AT-1  | acceptance.rs                 | BDD/ATDD    | Given/When/Then for add+list     |
| AT-2  | acceptance.rs                 | BDD/ATDD    | Given/When/Then for export       |
| AT-3  | acceptance.rs                 | BDD/ATDD    | Given/When/Then for undo         |

Black-box vs white-box classification is encoded both here and as comments
at the top of each test file.
