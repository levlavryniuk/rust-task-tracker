//! Placeholder; real implementation lands with the patterns branch (Template
//! Method for JSON/CSV exporters).
use anyhow::Result;
use crate::models::task::Task;

pub fn export_json(tasks: &[Task]) -> Result<String> {
    Ok(serde_json::to_string_pretty(tasks)?)
}
