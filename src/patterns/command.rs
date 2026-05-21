//! Command pattern — every mutating action becomes a value that knows how to
//! `apply` itself and how to `undo` itself. The `UndoStack` keeps a history of
//! applied commands so the CLI can offer `undo` / `redo`.

use crate::models::task::{Task, TaskId};

#[derive(Debug, Clone)]
pub enum Command {
    Add(Task),
    Delete(Task),
    SetDone { id: TaskId, previous: bool, next: bool },
}

#[derive(Default)]
pub struct UndoStack {
    done: Vec<Command>,
    redo: Vec<Command>,
}

impl UndoStack {
    pub fn record(&mut self, c: Command) {
        self.done.push(c);
        self.redo.clear(); // any new action invalidates the redo path
    }

    /// Returns the most recent command for the caller to invert.
    pub fn pop_undo(&mut self) -> Option<Command> {
        let c = self.done.pop()?;
        self.redo.push(c.clone());
        Some(c)
    }

    /// Returns the most recently undone command for the caller to re-apply.
    pub fn pop_redo(&mut self) -> Option<Command> {
        let c = self.redo.pop()?;
        self.done.push(c.clone());
        Some(c)
    }

    pub fn len(&self) -> usize { self.done.len() }
    pub fn is_empty(&self) -> bool { self.done.is_empty() }
}
