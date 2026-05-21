//! Factory Method pattern — `TaskFactory` centralises construction of the
//! various task flavours we support.
//!
//! In a richer domain this would dispatch to different concrete types; here
//! we use it to encapsulate the difference between "quick" tasks (medium
//! priority, no description) and "scheduled" tasks (with a due date). The
//! point is that no caller needs to know the defaults — they ask the factory.

use chrono::{DateTime, Utc};
use crate::models::task::{Priority, Task, TaskError};

pub struct TaskFactory;

impl TaskFactory {
    /// A quick task: title only, defaults everywhere else.
    pub fn quick(title: impl Into<String>) -> Result<Task, TaskError> {
        Task::try_new(title, "", Priority::Medium, None)
    }

    /// A scheduled task: a due date is required.
    pub fn scheduled(
        title: impl Into<String>,
        description: impl Into<String>,
        due: DateTime<Utc>,
    ) -> Result<Task, TaskError> {
        Task::try_new(title, description, Priority::Medium, Some(due))
    }

    /// A priority task: an explicit priority is required.
    pub fn priority(
        title: impl Into<String>,
        priority: Priority,
    ) -> Result<Task, TaskError> {
        Task::try_new(title, "", priority, None)
    }
}
