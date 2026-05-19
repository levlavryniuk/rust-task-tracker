//! Business-logic layer. Holds the in-memory task list and brokers all
//! read/write traffic to the storage backend.

use std::cmp::Ordering;

use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::models::task::{Priority, Task, TaskError, TaskId};
use crate::storage::TaskStore;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortKey { PriorityDesc, DueAsc }

impl SortKey {
    /// Returns a comparator for this sort key. Extracted from `sorted()` in
    /// the REFACTOR step of TDD so that the strategy is reusable and unit-
    /// testable in isolation.
    pub fn comparator(self) -> fn(&Task, &Task) -> Ordering {
        match self {
            SortKey::PriorityDesc => |a, b| b.priority.cmp(&a.priority),
            SortKey::DueAsc => |a, b| a.due.cmp(&b.due),
        }
    }
}

pub struct TaskService<S: TaskStore> {
    store: S,
    tasks: Vec<Task>,
}

impl<S: TaskStore> TaskService<S> {
    pub fn open(store: S) -> Result<Self> {
        let tasks = store.load()?;
        Ok(Self { store, tasks })
    }

    pub fn flush(&self) -> Result<()> { self.store.save(&self.tasks) }

    pub fn all(&self) -> &[Task] { &self.tasks }

    pub fn add(&mut self, t: Task) { self.tasks.push(t); }

    pub fn delete(&mut self, id: TaskId) -> Result<Task, TaskError> {
        let pos = self.tasks.iter().position(|t| t.id == id)
            .ok_or(TaskError::NotFound(id))?;
        Ok(self.tasks.remove(pos))
    }

    pub fn update_title(&mut self, id: TaskId, new_title: String) -> Result<(), TaskError> {
        let t = self.tasks.iter_mut().find(|t| t.id == id)
            .ok_or(TaskError::NotFound(id))?;
        t.title = new_title;
        Ok(())
    }

    pub fn set_done(&mut self, id: TaskId, done: bool) -> Result<(), TaskError> {
        let t = self.tasks.iter_mut().find(|t| t.id == id)
            .ok_or(TaskError::NotFound(id))?;
        t.done = done;
        Ok(())
    }

    pub fn resolve_prefix(&self, prefix: &str) -> Option<TaskId> {
        let matches: Vec<_> = self.tasks.iter()
            .filter(|t| t.id.0.to_string().starts_with(prefix))
            .collect();
        if matches.len() == 1 { Some(matches[0].id) } else { None }
    }

    pub fn search(&self, keyword: &str) -> Vec<&Task> {
        self.tasks.iter().filter(|t| t.matches_keyword(keyword)).collect()
    }

    pub fn filter(&self, priority: Option<Priority>,
                  from: Option<DateTime<Utc>>,
                  to: Option<DateTime<Utc>>) -> Vec<&Task> {
        self.tasks.iter().filter(|t| {
            if matches!(priority, Some(p) if t.priority != p) {
                return false;
            }
            if let (Some(f), Some(d)) = (from, t.due) {
                if d < f { return false; }
            }
            if let (Some(u), Some(d)) = (to, t.due) {
                if d > u { return false; }
            }
            true
        }).collect()
    }

    pub fn sorted(&self, key: SortKey) -> Vec<&Task> {
        let mut v: Vec<&Task> = self.tasks.iter().collect();
        let cmp = key.comparator();
        v.sort_by(|a, b| cmp(a, b));
        v
    }
}
