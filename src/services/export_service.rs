//! Public facade over the Template Method exporters. The CLI calls these
//! shorthand functions; tests can also call the exporter types directly.

use std::io::Write;

use anyhow::Result;

use crate::models::task::Task;
use crate::patterns::export::{CsvExporter, Exporter, JsonExporter};

pub fn export_json(tasks: &[Task]) -> Result<String> {
    let bytes = JsonExporter.serialize(tasks)?;
    Ok(String::from_utf8(bytes)?)
}

pub fn export_csv(tasks: &[Task]) -> Result<String> {
    let bytes = CsvExporter.serialize(tasks)?;
    Ok(String::from_utf8(bytes)?)
}

pub fn export_to<W: Write>(tasks: &[Task], fmt: &str, w: W) -> Result<()> {
    match fmt {
        "json" => JsonExporter.export(tasks, w),
        "csv"  => CsvExporter.export(tasks, w),
        other  => anyhow::bail!("unknown export format: {other}"),
    }
}
