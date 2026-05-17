pub mod json_storage;

use crate::models::task::Task;

/// Trait abstracting the persistence backend. The service layer talks to this,
/// not to concrete files, so we can swap JSON for CSV (or an in-memory store
/// in tests) without touching business logic. This is the Adapter pattern —
/// see docs/DESIGN_PATTERNS.md.
pub trait TaskStore {
    fn load(&self) -> anyhow::Result<Vec<Task>>;
    fn save(&self, tasks: &[Task]) -> anyhow::Result<()>;
}
