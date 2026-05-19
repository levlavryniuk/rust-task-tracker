//! Given-When-Then acceptance tests (ATDD/BDD style). Each test is structured
//! as three clearly-named helper sections so the intent reads like the spec.

use chrono::Utc;
use tempfile::tempdir;

use task_tracker::models::task::{Priority, Task};
use task_tracker::services::task_service::TaskService;
use task_tracker::services::export_service::export_json;
use task_tracker::storage::TaskStore;
use task_tracker::storage::json_storage::JsonStore;

// ---------- AT-1 ----------
// Given a fresh task tracker
// When the user adds three tasks and lists them
// Then the list contains all three in insertion order
#[test]
fn at1_listing_returns_added_tasks_in_order() {
    // Given
    let dir = tempdir().unwrap();
    let store = JsonStore::new(dir.path().join("tasks.json"));
    let mut svc = TaskService::open(store).unwrap();

    // When
    for title in ["a", "b", "c"] {
        svc.add(Task::try_new(title, "", Priority::Low, None).unwrap());
    }

    // Then
    let titles: Vec<&str> = svc.all().iter().map(|t| t.title.as_str()).collect();
    assert_eq!(titles, vec!["a", "b", "c"]);
}

// ---------- AT-2 ----------
// Given a task tracker containing one task
// When the user exports to JSON
// Then the output contains the task's title and is valid JSON
#[test]
fn at2_export_to_json_includes_titles_and_is_valid() {
    // Given
    let t = Task::try_new("write essay", "for class", Priority::High, None).unwrap();
    let tasks = vec![t];

    // When
    let json = export_json(&tasks).unwrap();

    // Then
    assert!(json.contains("write essay"));
    let _parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
}

// ---------- AT-3 ----------
// Given a task that has been marked done
// When we ask whether it is overdue
// Then it is not overdue, regardless of its due date
#[test]
fn at3_completed_tasks_are_never_overdue() {
    // Given
    let mut t = Task::try_new("late but done", "", Priority::Medium,
        Some(Utc::now() - chrono::Duration::days(7))).unwrap();
    t.done = true;

    // When
    let overdue = t.is_overdue(Utc::now());

    // Then
    assert!(!overdue, "a completed task must never be reported as overdue");
}
