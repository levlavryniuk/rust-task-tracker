# TDD Evidence

This document records the red → green → refactor cycle for the
`TaskService::sorted` feature on branch `feature/search-tdd`.

## Cycle

| Step     | Commit subject                                           | Date                |
|----------|----------------------------------------------------------|---------------------|
| RED      | `test(tdd): RED — failing test for sort_by(priority)`    | 2026-05-19 09:02:00 |
| GREEN    | `feat(tdd): GREEN — minimal sort_by implementation`      | 2026-05-19 09:38:00 |
| REFACTOR | `refactor(tdd): REFACTOR — extract SortKey::comparator`  | 2026-05-19 10:21:00 |

Reproduce locally:

```sh
git log --oneline --grep='RED\|GREEN\|REFACTOR'
```

## What changed at each step

### RED (failing test)
Added `tests/tdd_sort.rs` referring to a `TaskService::sorted` method and a
`SortKey` enum that did not exist yet. `cargo test` failed at the compile
phase. The compile failure *is* the red signal.

### GREEN (smallest possible passing change)
Added `SortKey::{PriorityDesc, DueAsc}` and a single `sorted(&self, key)`
method that matches inline on the enum and sorts in place. No abstraction
beyond what the test demanded.

### REFACTOR (clean up without changing behaviour)
Pulled the comparator out into `SortKey::comparator() -> fn(&Task, &Task)
-> Ordering`. The body of `sorted` shrinks to two lines and the comparator
becomes independently usable — this is the seed of the Strategy pattern
implemented in `feature/patterns`.

## Why TDD here
Sorting is exactly the kind of feature where it is easy to write code first
and tests-as-an-afterthought. Forcing the test up front made it impossible
to skip the spec ("high before low") and ensured the comparator stayed
encapsulated in a named type instead of being scattered across `cli`.
