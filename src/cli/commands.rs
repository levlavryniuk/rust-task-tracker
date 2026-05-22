//! CLI surface. The big `dispatch` match has been split into per-command
//! handlers (Extract Method) so each branch is small and individually
//! testable. The string `"fmt"` argument has been replaced by the typed
//! `ExportFmt` enum throughout (Replace Primitive Obsession).

use std::fs::File;
use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use clap::{Parser, Subcommand, ValueEnum};

use crate::models::task::{Priority, Task, TaskId};
use crate::services::task_service::{SortKey, TaskService};
use crate::services::export_service;
use crate::storage::json_storage::JsonStore;
use crate::storage::TaskStore;

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

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
pub enum ExportFmt { Json, Csv }

impl ExportFmt {
    fn as_str(self) -> &'static str {
        match self { ExportFmt::Json => "json", ExportFmt::Csv => "csv" }
    }
}

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
        Command::Add { title, description, priority, due }
            => handle_add(&mut svc, title, description, priority, due),
        Command::List { sort }                  => handle_list(&svc, sort),
        Command::Delete { id }                  => handle_delete(&mut svc, &id),
        Command::Done { id }                    => handle_done(&mut svc, &id),
        Command::Search { keyword }             => handle_search(&svc, &keyword),
        Command::Filter { priority, from, to }  => handle_filter(&svc, priority, from, to),
        Command::Stats                          => { print_stats(svc.all()); Ok(()) },
        Command::Export { format, out }         => handle_export(&svc, format, out),
    }
}

fn handle_add<S: TaskStore>(svc: &mut TaskService<S>,
                            title: String, description: String,
                            priority: Priority, due: Option<String>) -> Result<()> {
    let due = due.map(|s| parse_date(&s)).transpose()?;
    let task = Task::try_new(title, description, priority, due)
        .map_err(|e| anyhow!("{e}"))?;
    println!("created {}", task.id.short());
    svc.add(task);
    svc.flush()
}

fn handle_list<S: TaskStore>(svc: &TaskService<S>, sort: Option<SortArg>) -> Result<()> {
    let tasks: Vec<&Task> = match sort {
        Some(s) => svc.sorted(s.into()),
        None => svc.all().iter().collect(),
    };
    print_tasks(&tasks);
    Ok(())
}

fn handle_delete<S: TaskStore>(svc: &mut TaskService<S>, id: &str) -> Result<()> {
    let id = resolve(svc, id)?;
    svc.delete(id).map_err(|e| anyhow!("{e}"))?;
    svc.flush()?;
    println!("deleted {}", id.short());
    Ok(())
}

fn handle_done<S: TaskStore>(svc: &mut TaskService<S>, id: &str) -> Result<()> {
    let id = resolve(svc, id)?;
    svc.set_done(id, true).map_err(|e| anyhow!("{e}"))?;
    svc.flush()?;
    println!("done {}", id.short());
    Ok(())
}

fn handle_search<S: TaskStore>(svc: &TaskService<S>, keyword: &str) -> Result<()> {
    let hits = svc.search(keyword);
    print_tasks(&hits);
    Ok(())
}

fn handle_filter<S: TaskStore>(svc: &TaskService<S>,
                               priority: Option<Priority>,
                               from: Option<String>, to: Option<String>) -> Result<()> {
    let from = from.map(|s| parse_date(&s)).transpose()?;
    let to   = to  .map(|s| parse_date(&s)).transpose()?;
    let hits = svc.filter(priority, from, to);
    print_tasks(&hits);
    Ok(())
}

fn handle_export<S: TaskStore>(svc: &TaskService<S>,
                               format: ExportFmt, out: Option<PathBuf>) -> Result<()> {
    match out {
        Some(path) => {
            let f = File::create(&path)
                .with_context(|| format!("creating {}", path.display()))?;
            export_service::export_to(svc.all(), format.as_str(), f)?;
            println!("wrote {}", path.display());
        }
        None => {
            let text = match format {
                ExportFmt::Json => export_service::export_json(svc.all())?,
                ExportFmt::Csv  => export_service::export_csv(svc.all())?,
            };
            println!("{text}");
        }
    }
    Ok(())
}

fn resolve<S: TaskStore>(svc: &TaskService<S>, s: &str) -> Result<TaskId> {
    use std::str::FromStr;
    if let Ok(id) = TaskId::from_str(s) {
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
