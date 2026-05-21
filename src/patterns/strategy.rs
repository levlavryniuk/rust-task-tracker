//! Strategy pattern — pluggable sort orderings for tasks.
//!
//! The `Sorter` trait is the strategy interface; concrete strategies implement
//! `compare()`. The service layer takes any `&dyn Sorter` and applies it, so
//! adding a new ordering (e.g. by creation time, by alphabetical title) does
//! not require changing the service. See docs/DESIGN_PATTERNS.md.

use std::cmp::Ordering;

use crate::models::task::Task;

pub trait Sorter {
    fn compare(&self, a: &Task, b: &Task) -> Ordering;
}

pub struct ByPriorityDesc;
impl Sorter for ByPriorityDesc {
    fn compare(&self, a: &Task, b: &Task) -> Ordering {
        b.priority.cmp(&a.priority)
    }
}

pub struct ByDueAsc;
impl Sorter for ByDueAsc {
    fn compare(&self, a: &Task, b: &Task) -> Ordering {
        a.due.cmp(&b.due)
    }
}

pub struct ByTitleAsc;
impl Sorter for ByTitleAsc {
    fn compare(&self, a: &Task, b: &Task) -> Ordering {
        a.title.to_lowercase().cmp(&b.title.to_lowercase())
    }
}

/// Apply any strategy to a slice of tasks, returning a new sorted Vec of
/// references (the original slice is untouched).
pub fn sort_with<'a>(tasks: &'a [Task], strategy: &dyn Sorter) -> Vec<&'a Task> {
    let mut v: Vec<&Task> = tasks.iter().collect();
    v.sort_by(|a, b| strategy.compare(a, b));
    v
}
