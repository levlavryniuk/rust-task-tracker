//! Template Method pattern — `Exporter` defines the export *skeleton*
//! (collect → serialise → write) and concrete formats fill in just the
//! serialise step. Adding a new format means implementing one method.

use std::io::Write;

use anyhow::Result;
use crate::models::task::Task;

/// The skeleton. `export` orchestrates the steps in a fixed order; concrete
/// exporters only choose how to serialise.
pub trait Exporter {
    fn serialize(&self, tasks: &[Task]) -> Result<Vec<u8>>;

    fn export(&self, tasks: &[Task], mut sink: impl Write) -> Result<()> {
        let bytes = self.serialize(tasks)?;
        sink.write_all(&bytes)?;
        Ok(())
    }
}

pub struct JsonExporter;
impl Exporter for JsonExporter {
    fn serialize(&self, tasks: &[Task]) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec_pretty(tasks)?)
    }
}

pub struct CsvExporter;
impl Exporter for CsvExporter {
    fn serialize(&self, tasks: &[Task]) -> Result<Vec<u8>> {
        let mut wtr = csv::Writer::from_writer(Vec::new());
        // Header row.
        wtr.write_record(["id", "title", "description", "priority", "due", "done", "created_at"])?;
        for t in tasks {
            wtr.write_record([
                t.id.to_string(),
                t.title.clone(),
                t.description.clone(),
                t.priority.to_string(),
                t.due.map(|d| d.to_rfc3339()).unwrap_or_default(),
                t.done.to_string(),
                t.created_at.to_rfc3339(),
            ])?;
        }
        let buf = wtr.into_inner()?;
        Ok(buf)
    }
}
