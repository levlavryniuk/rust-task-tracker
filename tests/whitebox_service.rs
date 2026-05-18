//! White-box tests — derived from internal code structure. Each test targets a
//! specific branch in TaskService::filter / is_overdue / resolve_prefix.
//!
//! Technique: branch coverage. The filter() method has three independent
//! conditional branches (priority, from, to); we exercise each in isolation
//! plus the all-three-set case.
//!
//! WB-1 : filter — only the priority branch is taken
//! WB-2 : filter — only the `from` branch is taken (and excludes earlier)
//! WB-3 : filter — only the `to` branch is taken (and excludes later)
//! WB-4 : filter — all three branches taken simultaneously
//! WB-5 : is_overdue — true branch (past due, not done)
//! WB-6 : is_overdue — false branch via `done` short-circuit
//! WB-7 : resolve_prefix — multiple-match branch returns None

use chrono::{Duration, Utc};

use task_tracker::models::task::{Priority, Task};
use task_tracker::services::task_service::TaskService;
use task_tracker::storage::TaskStore;
use task_tracker::storage::json_storage::JsonStore;
use tempfile::tempdir;

fn svc() -> TaskService<JsonStore> {
    let dir = Box::leak(Box::new(tempdir().unwrap()));
    let store = JsonStore::new(dir.path().join("tasks.json"));
    TaskService::open(store).unwrap()
}

#[test]
fn wb1_filter_priority_branch_only() {
    let mut s = svc();
    s.add(Task::try_new("a", "", Priority::Low, None).unwrap());
    s.add(Task::try_new("b", "", Priority::Medium, None).unwrap());
    let got = s.filter(Some(Priority::Low), None, None);
    assert_eq!(got.len(), 1);
}

#[test]
fn wb2_filter_from_branch_only() {
    let mut s = svc();
    let now = Utc::now();
    let yesterday = Some(now - Duration::days(1));
    let tomorrow = Some(now + Duration::days(1));
    s.add(Task::try_new("old", "", Priority::Low, yesterday).unwrap());
    s.add(Task::try_new("new", "", Priority::Low, tomorrow).unwrap());
    let got = s.filter(None, Some(now), None);
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].title, "new");
}

#[test]
fn wb3_filter_to_branch_only() {
    let mut s = svc();
    let now = Utc::now();
    s.add(Task::try_new("a", "", Priority::Low, Some(now - Duration::days(1))).unwrap());
    s.add(Task::try_new("b", "", Priority::Low, Some(now + Duration::days(1))).unwrap());
    let got = s.filter(None, None, Some(now));
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].title, "a");
}

#[test]
fn wb4_filter_all_branches_taken() {
    let mut s = svc();
    let now = Utc::now();
    s.add(Task::try_new("hit", "", Priority::High, Some(now)).unwrap());
    s.add(Task::try_new("wrong-prio", "", Priority::Low, Some(now)).unwrap());
    let got = s.filter(Some(Priority::High),
                       Some(now - Duration::hours(1)),
                       Some(now + Duration::hours(1)));
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].title, "hit");
}

#[test]
fn wb5_is_overdue_true_branch() {
    let now = Utc::now();
    let t = Task::try_new("late", "", Priority::Low, Some(now - Duration::days(1))).unwrap();
    assert!(t.is_overdue(now));
}

#[test]
fn wb6_is_overdue_false_when_done() {
    let now = Utc::now();
    let mut t = Task::try_new("late but done", "", Priority::Low,
                              Some(now - Duration::days(1))).unwrap();
    t.done = true;
    assert!(!t.is_overdue(now));
}

#[test]
fn wb7_resolve_prefix_returns_none_when_ambiguous() {
    // Force a collision by adding two tasks and asking for a 1-char prefix —
    // unlikely to be unique across two random UUIDs.
    let mut s = svc();
    s.add(Task::try_new("a", "", Priority::Low, None).unwrap());
    s.add(Task::try_new("b", "", Priority::Low, None).unwrap());
    // Pick the first hex digit of task 0 — odds it also prefixes task 1 are
    // 1/16, but a 0-char prefix definitely matches both. We use "" indirectly
    // by checking that an obviously-ambiguous prefix returns None.
    assert!(s.resolve_prefix("").is_none());
}
