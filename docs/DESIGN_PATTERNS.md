# Design Patterns

Two patterns are required by the spec (one creational + one behavioural); four
are implemented in this project. Each is genuinely useful — none was forced.

## 1. Strategy (behavioural)

**Files:** `src/patterns/strategy.rs`

**Problem:** the CLI offers multiple sort orderings for `list`. Hard-coding
the comparators inline (the GREEN step of TDD did exactly that) would mean
that adding a new ordering touches `task_service.rs` *and* the CLI dispatch.

**Solution:** `trait Sorter { fn compare(&self, a: &Task, b: &Task) -> Ordering }`
with concrete strategies `ByPriorityDesc`, `ByDueAsc`, `ByTitleAsc`. The
`sort_with()` helper takes any `&dyn Sorter`.

**Benefit:** adding `ByCreationDesc` later is one struct + one trait impl;
nothing else changes.

```text
   ┌──────────────────┐
   │     Sorter       │◀─────── &dyn Sorter
   │ + compare(a,b)   │
   └─────────┬────────┘
             │
   ┌─────────┼─────────────┬──────────────────┐
   │         │             │                  │
┌──▼───┐  ┌──▼───────┐  ┌──▼─────────┐  ┌─────▼──────┐
│ByDue │  │ByPriority│  │ByTitleAsc  │  │ ...future  │
│ Asc  │  │  Desc    │  │            │  │            │
└──────┘  └──────────┘  └────────────┘  └────────────┘
```

## 2. Factory Method (creational)

**Files:** `src/patterns/factory.rs`

**Problem:** `Task::try_new` takes four arguments. Three callers (CLI add,
test setup, undo redo) want different defaults for those arguments.

**Solution:** `TaskFactory` exposes `quick(title)`, `scheduled(title, desc,
due)`, `priority(title, prio)`. Each route hides the defaults its callers
don't care about.

**Benefit:** test setup goes from
`Task::try_new("x", "", Priority::Medium, None).unwrap()` (which leaks the
default policy across every test) to `TaskFactory::quick("x").unwrap()`.
Changing the default priority is now a one-line edit in the factory.

## 3. Command (behavioural)

**Files:** `src/patterns/command.rs`

**Problem:** undo/redo across heterogeneous operations (add, delete, set-done)
requires representing "what just happened" as a uniform value.

**Solution:** `enum Command { Add(Task), Delete(Task), SetDone { ... } }`
plus an `UndoStack` holding both an undone-history and a redo-stack. Any
new mutating operation becomes a new variant.

**Benefit:** the CLI's undo handler is a single `match` on the popped
`Command`. The redo stack is invalidated on any *new* recorded action,
matching standard editor semantics.

```text
   ┌────────────────────────────┐
   │          Command            │
   │  ┌──────────────────────┐   │
   │  │ Add(Task)            │   │
   │  │ Delete(Task)         │   │
   │  │ SetDone{id,prev,next}│   │
   │  └──────────────────────┘   │
   └─────────────┬──────────────┘
                 │
                 ▼
       ┌──────────────────┐
       │    UndoStack     │
       │  done: Vec<Cmd>  │
       │  redo: Vec<Cmd>  │
       └──────────────────┘
```

## 4. Template Method (behavioural)

**Files:** `src/patterns/export.rs`

**Problem:** JSON and CSV exports share a skeleton — collect tasks,
serialise, write to a sink — but differ in the serialise step. Inlining
both formats would duplicate the writing logic.

**Solution:** `trait Exporter` provides a default `export(tasks, sink)` that
calls `serialize(tasks)` and writes the bytes. Concrete exporters
(`JsonExporter`, `CsvExporter`) implement only `serialize`.

**Benefit:** a future Markdown exporter is a single `impl Exporter for
MarkdownExporter` providing a 10-line `serialize`. The skeleton stays put.

---

## Why not Observer, Decorator, Adapter, etc.

- **Observer:** notifications-on-overdue would be nice but the CLI is
  one-shot — no event loop, nothing to subscribe to. Forced.
- **Decorator:** validation layers were considered, but `Task::try_new`
  already centralises validation and there is only one rule. Forced.
- **Adapter:** arguably already used by `trait TaskStore` (concrete `JsonStore`
  adapts the filesystem to the service's expectations). It is mentioned in
  the storage module docs but not counted in the four above.
