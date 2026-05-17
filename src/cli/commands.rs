//! CLI surface. Uses clap's derive macros so the argument grammar stays close
//! to the type definitions. The `dispatch` function is intentionally thin —
//! all heavy lifting is delegated to the service layer.

use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use clap::{Parser, Subcommand};

use crate::models::task::{Priority, Task};
use crate::services::task_service::TaskService;
use crate::storage::json_storage::JsonStore;

#[derive(Parser, Debug)]
#[command(name = "task-tracker", version, about = "A tiny task tracker.")]
pub struct Cli {
    /// Path to the JSON data file.
    #[arg(long, default_value = "tasks.json", global = true)]
    pub data_file: PathBuf,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add a new task.
    Add {
        title: String,
        #[arg(short, long, default_value = "")]
        description: String,
        #[arg(short, long, default_value_t = Priority::Medium)]
        priority: Priority,
        /// Due date in YYYY-MM-DD.
        #[arg(long)]
        due: Option<String>,
    },
    /// List all tasks.
    List,
    /// Delete a task by its (short or full) id.
    Delete { id: String },
    /// Mark a task done.
    Done { id: String },
}

pub fn dispatch(cli: Cli) -> Result<()> {
    let store = JsonStore::new(&cli.data_file);
    let mut svc = TaskService::open(store)?;

    match cli.command {
        Command::Add { title, description, priority, due } => {
            let due = match due {
                Some(s) => Some(parse_date(&s)?),
                None => None,
            };
            let task = Task::try_new(title, description, priority, due)
                .map_err(|e| anyhow!("{e}"))?;
            println!("created {}", task.id.short());
            svc.add(task);
            svc.flush()?;
        }
        Command::List => {
            print_tasks(svc.all());
        }
        Command::Delete { id } => {
            let id = resolve(&svc, &id)?;
            svc.delete(id).map_err(|e| anyhow!("{e}"))?;
            svc.flush()?;
            println!("deleted {}", &id.short());
        }
        Command::Done { id } => {
            let id = resolve(&svc, &id)?;
            svc.set_done(id, true).map_err(|e| anyhow!("{e}"))?;
            svc.flush()?;
            println!("done {}", &id.short());
        }
    }
    Ok(())
}

fn resolve<S: crate::storage::TaskStore>(svc: &TaskService<S>, s: &str)
    -> Result<crate::models::task::TaskId>
{
    use std::str::FromStr;
    if let Ok(id) = crate::models::task::TaskId::from_str(s) {
        return Ok(id);
    }
    svc.resolve_prefix(s).ok_or_else(|| anyhow!("no task matches id prefix '{s}'"))
}

fn parse_date(s: &str) -> Result<DateTime<Utc>> {
    let nd = NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .with_context(|| format!("date '{s}' is not YYYY-MM-DD"))?;
    Ok(Utc.from_utc_datetime(&nd.and_hms_opt(0, 0, 0).unwrap()))
}

fn print_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("(no tasks)");
        return;
    }
    println!("{:<10} {:<8} {:<10} {}", "ID", "PRIORITY", "DONE", "TITLE");
    for t in tasks {
        println!("{:<10} {:<8} {:<10} {}",
            t.id.short(),
            t.priority,
            if t.done { "yes" } else { "no" },
            t.title);
    }
}
