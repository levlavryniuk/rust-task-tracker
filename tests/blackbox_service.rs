//! Black-box tests — derived from the functional spec in the project README,
//! without reference to internal control flow. Each test corresponds to a
//! requirement number from the spec.
//!
//! BB-1 : add then list shows the new task
//! BB-2 : delete by id removes a task
//! BB-3 : search by keyword finds matches across title and description
//! BB-4 : filter by priority returns only matching priorities
//! BB-5 : empty title is rejected

use chrono::Utc;
use tempfile::tempdir;

use task_tracker::models::task::{Priority, Task, TaskError};
use task_tracker::services::task_service::TaskService;
use task_tracker::storage::TaskStore;
use task_tracker::storage::json_storage::JsonStore;

fn fresh_service() -> TaskService<JsonStore> {
    let dir = tempdir().unwrap();
    let path = dir.path().join("tasks.json");
    // dir is intentionally leaked into the test scope — tempfile cleans on drop
    // but we want it alive for the duration of the test.
    let store = JsonStore::new(path);
    TaskService::open(store).unwrap()
}

#[test]
fn bb1_add_then_list_shows_task() {
    let mut svc = fresh_service();
    let t = Task::try_new("write report", "", Priority::High, None).unwrap();
    svc.add(t);
    assert_eq!(svc.all().len(), 1);
    assert_eq!(svc.all()[0].title, "write report");
}

#[test]
fn bb2_delete_removes_the_task() {
    let mut svc = fresh_service();
    let t = Task::try_new("temp", "", Priority::Low, None).unwrap();
    let id = t.id;
    svc.add(t);
    svc.delete(id).unwrap();
    assert!(svc.all().is_empty());
}

#[test]
fn bb3_search_finds_match_in_description() {
    let mut svc = fresh_service();
    svc.add(Task::try_new("groceries", "milk and bread", Priority::Low, None).unwrap());
    svc.add(Task::try_new("call dentist", "annual checkup", Priority::Medium, None).unwrap());
    let hits = svc.search("milk");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].title, "groceries");
}

#[test]
fn bb4_filter_by_priority_excludes_others() {
    let mut svc = fresh_service();
    svc.add(Task::try_new("a", "", Priority::Low, None).unwrap());
    svc.add(Task::try_new("b", "", Priority::High, None).unwrap());
    svc.add(Task::try_new("c", "", Priority::High, None).unwrap());
    let highs = svc.filter(Some(Priority::High), None, None);
    assert_eq!(highs.len(), 2);
}

#[test]
fn bb5_empty_title_is_rejected() {
    let err = Task::try_new("   ", "x", Priority::Low, None).unwrap_err();
    assert_eq!(err, TaskError::EmptyTitle);
}

#[test]
fn bb_persistence_survives_reopen() {
    // Combined persistence / round-trip black-box check.
    let dir = tempdir().unwrap();
    let path = dir.path().join("tasks.json");
    {
        let store = JsonStore::new(&path);
        let mut svc = TaskService::open(store).unwrap();
        svc.add(Task::try_new("persistent", "", Priority::Medium, None).unwrap());
        svc.flush().unwrap();
    }
    let store = JsonStore::new(&path);
    let svc = TaskService::open(store).unwrap();
    assert_eq!(svc.all().len(), 1);
    assert_eq!(svc.all()[0].title, "persistent");
    let _ = Utc::now();
}
