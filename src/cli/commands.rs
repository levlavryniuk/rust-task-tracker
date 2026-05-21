//! CLI surface. Uses clap's derive macros so the argument grammar stays close
//! to the type definitions.

use std::fs::File;
use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use clap::{Parser, Subcommand, ValueEnum};

use crate::models::task::{Priority, Task};
use crate::services::task_service::{SortKey, TaskService};
use crate::services::export_service;
use crate::storage::json_storage::JsonStore;

#[derive(Parser, Debug)]
#[command(name = "task-tracker", version, about = "A tiny task tracker.")]
pub struct Cli {
    #[arg(long, default_value = "tasks.json", global = true)]
    pub data_file: PathBuf,
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Add {
        title: String,
        #[arg(short, long, default_value = "")]
        description: String,
        #[arg(short, long, default_value_t = Priority::Medium)]
        priority: Priority,
        #[arg(long)]
        due: Option<String>,
    },
    List {
        #[arg(long, value_enum)]
        sort: Option<SortArg>,
    },
    Delete { id: String },
    Done { id: String },
    Search { keyword: String },
    Filter {
        #[arg(short, long)]
        priority: Option<Priority>,
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
    },
    Stats,
    Export {
        #[arg(value_enum)]
        format: ExportFmt,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum SortArg { Priority, Due }

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum ExportFmt { Json, Csv }

impl From<SortArg> for SortKey {
    fn from(s: SortArg) -> Self {
        match s {
            SortArg::Priority => SortKey::PriorityDesc,
            SortArg::Due => SortKey::DueAsc,
        }
    }
}

pub fn dispatch(cli: Cli) -> Result<()> {
    let store = JsonStore::new(&cli.data_file);
    let mut svc = TaskService::open(store)?;

    match cli.command {
        Command::Add { title, description, priority, due } => {
            let due = match due { Some(s) => Some(parse_date(&s)?), None => None };
            let task = Task::try_new(title, description, priority, due)
                .map_err(|e| anyhow!("{e}"))?;
            println!("created {}", task.id.short());
            svc.add(task);
            svc.flush()?;
        }
        Command::List { sort } => {
            let tasks: Vec<&Task> = match sort {
                Some(s) => svc.sorted(s.into()),
                None => svc.all().iter().collect(),
            };
            print_tasks(&tasks);
        }
        Command::Delete { id } => {
            let id = resolve(&svc, &id)?;
            svc.delete(id).map_err(|e| anyhow!("{e}"))?;
            svc.flush()?;
            println!("deleted {}", id.short());
        }
        Command::Done { id } => {
            let id = resolve(&svc, &id)?;
            svc.set_done(id, true).map_err(|e| anyhow!("{e}"))?;
            svc.flush()?;
            println!("done {}", id.short());
        }
        Command::Search { keyword } => {
            let hits = svc.search(&keyword);
            print_tasks(&hits);
        }
        Command::Filter { priority, from, to } => {
            let from = from.map(|s| parse_date(&s)).transpose()?;
            let to   = to  .map(|s| parse_date(&s)).transpose()?;
            let hits = svc.filter(priority, from, to);
            print_tasks(&hits);
        }
        Command::Stats => print_stats(svc.all()),
        Command::Export { format, out } => {
            let fmt = match format { ExportFmt::Json => "json", ExportFmt::Csv => "csv" };
            match out {
                Some(path) => {
                    let f = File::create(&path)
                        .with_context(|| format!("creating {}", path.display()))?;
                    export_service::export_to(svc.all(), fmt, f)?;
                    println!("wrote {}", path.display());
                }
                None => {
                    let text = match fmt {
                        "json" => export_service::export_json(svc.all())?,
                        "csv"  => export_service::export_csv(svc.all())?,
                        _ => unreachable!(),
                    };
                    println!("{text}");
                }
            }
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

fn print_tasks(tasks: &[&Task]) {
    if tasks.is_empty() { println!("(no tasks)"); return; }
    println!("{:<10} {:<8} {:<6} {}", "ID", "PRIORITY", "DONE", "TITLE");
    for t in tasks {
        println!("{:<10} {:<8} {:<6} {}",
            t.id.short(), t.priority,
            if t.done { "yes" } else { "no" }, t.title);
    }
}

fn print_stats(tasks: &[Task]) {
    let now = Utc::now();
    let total = tasks.len();
    let done = tasks.iter().filter(|t| t.done).count();
    let overdue = tasks.iter().filter(|t| t.is_overdue(now)).count();
    let by_prio = |p: Priority| tasks.iter().filter(|t| t.priority == p).count();
    println!("total:     {total}");
    println!("done:      {done}");
    println!("overdue:   {overdue}");
    println!("by prio:   low={}, medium={}, high={}",
        by_prio(Priority::Low), by_prio(Priority::Medium), by_prio(Priority::High));
}
