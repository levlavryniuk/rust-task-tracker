//! Placeholder CLI; flesh out in feature/storage.
use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "task-tracker", version, about = "A tiny task tracker.")]
pub struct Cli {}

pub fn dispatch(_cli: Cli) -> Result<()> {
    println!("task-tracker (skeleton)");
    Ok(())
}
