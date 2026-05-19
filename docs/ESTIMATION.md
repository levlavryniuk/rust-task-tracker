# Estimation & Reflection

User stories were estimated in Fibonacci story points (1, 2, 3, 5, 8) before
implementation. Actual effort was recorded retrospectively in hours.

| # | User story                                                      | Estimate | Actual | Notes                                                                  |
|---|-----------------------------------------------------------------|----------|--------|------------------------------------------------------------------------|
| 1 | As a user, I can add a task with title, description, priority   | 2        | ~1h    | Slightly over-estimated — `clap` derive made the CLI trivial.          |
| 2 | As a user, I can list tasks in a readable table                 | 1        | ~0.5h  | Bang on.                                                               |
| 3 | As a user, I can delete a task by id (full or short prefix)     | 3        | ~1.5h  | Short-prefix resolution took longer than expected; ambiguity handling. |
| 4 | As a user, I can search tasks by keyword                        | 2        | ~0.5h  | Over-estimated; `matches_keyword` was a one-liner on `Task`.           |
| 5 | As a user, I can filter by priority and date range              | 3        | ~2h    | Three independent branches → trickier than it looked, ended up bigger. |
| 6 | As a user, I can sort by priority or due date                   | 2        | ~1h    | Drove this with TDD — small extra time for the refactor step.          |
| 7 | As a user, I can persist tasks across sessions                  | 3        | ~2h    | Atomic-rename writes added safety; serialisation was free with serde.  |
| 8 | As a user, I can export tasks to JSON and CSV                   | 5        | ~3h    | Template Method abstraction took the bulk of the time.                 |
| 9 | As a user, I can undo and redo my last operation                | 5        | ~3h    | Command pattern; in-memory only.                                       |
|10 | As a user, I can see stats (counts, overdue, by priority)       | 2        | ~1h    | Straightforward.                                                       |

**Total estimated:** 28 story points • **Actual:** ~15.5h

## Reflection

- The simplest stories (search, list) were consistently *over*-estimated. I'd
  internalised the friction of writing tests *and* code, but `clap` and
  `serde` collapse a lot of that.
- The more architectural items (export, undo) were *under*-estimated by about
  50% — extracting a `Template Method` skeleton or a `Command` enum requires
  one or two false starts before the shape settles.
- The TDD-driven story (sort) was on-budget but the breakdown was unusual:
  more time on the refactor step than on the implementation itself. That feels
  right; the refactor is where the design actually emerges.

Lesson: increase the estimate for anything that requires a new abstraction
(pattern, trait, generic) by ~50%. Keep flat estimates for CRUD-shaped work.
