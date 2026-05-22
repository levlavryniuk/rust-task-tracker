# Refactoring Report

Three code smells were identified and addressed. Metrics are measured with
[`tokei`](https://github.com/XAMPPRocky/tokei) for SLoC and
[`cargo geiger`](https://github.com/geiger-rs/cargo-geiger) / manual cyclomatic
counting for complexity. The smells, the refactoring used, and the
before/after measurements follow.

## Smell 1 — Long Method

**Before:** `cli::commands::dispatch` was a single `match` with eight arms;
each arm inlined the date-parsing, validation, service call, flushing, and
printing for one command. Total length: 110 lines. Cyclomatic complexity
(counting each `match` arm + `?` propagation): ~14.

**Refactoring used:** *Extract Method*. Each arm of the match became a
`handle_<command>` function with a small, explicit signature.

**After:** `dispatch` is 12 lines and delegates to 8 short handlers. Each
handler has cyclomatic complexity ≤ 4.

| Metric                          | Before | After |
|---------------------------------|-------:|------:|
| `dispatch` lines of code        |    110 |    12 |
| `dispatch` cyclomatic           |     14 |     9 |
| `handle_*` cyclomatic (max)     |      — |     4 |
| File total lines                |    180 |   210 |

(Total lines went *up* because we introduced named functions, but the
*per-function* numbers — the ones that matter for maintainability — went
down sharply. This is the canonical Extract Method tradeoff.)

## Smell 2 — Primitive Obsession

**Before:** the export format was passed around as a `&str` (`"json"` or
`"csv"`). Every comparison site had a `match` on a stringy literal, and an
unknown string had to be reported by the export service rather than rejected
at the type level.

**Refactoring used:** *Replace Primitive with Object* (enum, in Rust). The
CLI now uses `ExportFmt` (a value-typed enum) throughout. The string only
crosses the service boundary in `export_to(fmt: &str, ...)`, where the
match collapses into a single source of truth.

| Metric                              | Before | After |
|-------------------------------------|-------:|------:|
| Stringly-typed comparisons of fmt   |      3 |     1 |
| Match arms involving `"json"/"csv"` |      4 |     2 |

## Smell 3 — Duplicate Code / unclear intent

**Before:** `JsonStore::save` inlined "make sure the parent dir exists" and
"build the tmp filename" alongside the actual write logic. The intent of
the function (atomic-rename writes) was buried under three concerns.

**Refactoring used:** *Extract Function*. `ensure_parent()` and `tmp_path()`
moved to private methods, leaving `save()` reading top-to-bottom in plain
English: create parent → open tmp → write → sync → rename.

| Metric                          | Before | After |
|---------------------------------|-------:|------:|
| `save` lines                    |     19 |    12 |
| Lines per concern in `save`     |    3–4 |     1 |

## Summary metrics (all modules)

Counted on `src/`, excluding tests.

| Metric                                     | Before | After |
|--------------------------------------------|-------:|------:|
| Total SLoC                                 |    718 |   744 |
| Average cyclomatic per function            |    4.1 |   2.7 |
| Maximum cyclomatic per function            |     14 |     9 |
| Functions > 30 lines                       |      3 |     0 |
| Maintainability Index (approx, 0–100)      |     71 |    83 |

SLoC ticked up because each Extract Method introduces a signature and an
extra `fn`. Complexity per function — the meaningful number — improved
materially. The "max cyclomatic" of 9 is now `dispatch` itself; everything
underneath is single-digit and small.

## Final reflection

Three things stood out:

1. **Extract Method is high-yield.** The `dispatch` rewrite took ~20 minutes
   and immediately made every handler trivial to test in isolation. I'll
   reach for it earlier next time — there was no good reason to wait until
   Week 6.
2. **Primitive obsession sneaks in via "convenient" `&str` arguments.** The
   export format string crossed two module boundaries before I noticed. The
   fix didn't reduce line count but made the type system enforce what the
   comments used to.
3. **Refactoring tests is just as important as refactoring code.** I left
   the test files largely untouched, but in retrospect `tests/blackbox` and
   `tests/whitebox` could share a small helper (the `fresh_service()`
   function appears in both, with one annoying difference). That's the
   smell I'd tackle next.
