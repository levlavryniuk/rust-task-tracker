//! Domain model: a single task. Kept deliberately small — the engineering
//! requirements (testing, patterns, refactoring) matter more than the domain
//! richness.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

/// Strongly-typed ID newtype. Avoids the "primitive obsession" smell of using
/// raw `String`/`Uuid` everywhere — see the refactoring report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub Uuid);

impl TaskId {
    pub fn new() -> Self { Self(Uuid::new_v4()) }
    pub fn short(&self) -> String { self.0.to_string()[..8].to_string() }
}

impl Default for TaskId {
    fn default() -> Self { Self::new() }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.0.fmt(f) }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid task id: {0}")]
pub struct ParseTaskIdError(String);

impl FromStr for TaskId {
    type Err = ParseTaskIdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Accept both the full UUID and the 8-char short prefix that `list`
        // displays — convenient for users typing IDs on the command line.
        if let Ok(u) = Uuid::parse_str(s) {
            return Ok(TaskId(u));
        }
        // Short-prefix lookup is handled by TaskService; the parser only
        // validates that the input is a reasonable hex prefix here.
        if s.len() >= 4 && s.chars().all(|c| c.is_ascii_hexdigit()) {
            // We cannot resolve a real UUID from a prefix alone, but we accept
            // the shape; the service layer does prefix lookup.
            return Err(ParseTaskIdError(format!("prefix-only id: {s}")));
        }
        Err(ParseTaskIdError(s.to_string()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
        })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("unknown priority '{0}', expected one of: low, medium, high")]
pub struct ParsePriorityError(String);

impl FromStr for Priority {
    type Err = ParsePriorityError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "low" | "l" => Ok(Priority::Low),
            "medium" | "med" | "m" => Ok(Priority::Medium),
            "high" | "h" => Ok(Priority::High),
            other => Err(ParsePriorityError(other.to_string())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub due: Option<DateTime<Utc>>,
    pub done: bool,
    pub created_at: DateTime<Utc>,
}

impl Task {
    /// Constructs a fully-formed task. Validation is centralised here so every
    /// code path that creates a task hits the same checks.
    pub fn try_new(
        title: impl Into<String>,
        description: impl Into<String>,
        priority: Priority,
        due: Option<DateTime<Utc>>,
    ) -> Result<Self, TaskError> {
        let title = title.into();
        if title.trim().is_empty() {
            return Err(TaskError::EmptyTitle);
        }
        Ok(Self {
            id: TaskId::new(),
            title,
            description: description.into(),
            priority,
            due,
            done: false,
            created_at: Utc::now(),
        })
    }

    /// Convenience predicate — a task is overdue if it has a due date in the
    /// past *and* isn't done.
    pub fn is_overdue(&self, now: DateTime<Utc>) -> bool {
        match self.due {
            Some(d) => !self.done && d < now,
            None => false,
        }
    }

    /// True if the task's text content matches the keyword (case-insensitive).
    /// Lives on the model so service-layer search stays a one-liner.
    pub fn matches_keyword(&self, kw: &str) -> bool {
        let kw_l = kw.to_lowercase();
        self.title.to_lowercase().contains(&kw_l)
            || self.description.to_lowercase().contains(&kw_l)
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TaskError {
    #[error("task title cannot be empty")]
    EmptyTitle,
    #[error("task with id {0} not found")]
    NotFound(TaskId),
}
