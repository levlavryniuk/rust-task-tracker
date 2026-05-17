//! Smoke test for round-tripping tasks through JsonStore. Black-box: we only
//! touch the public API (TaskStore trait + Task constructor).
use chrono::Utc;
use tempfile::tempdir;

use task_tracker::models::task::{Priority, Task};
use task_tracker::storage::TaskStore;
use task_tracker::storage::json_storage::JsonStore;

#[test]
fn roundtrip_json_store_preserves_tasks() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("tasks.json");
    let store = JsonStore::new(&path);

    let t = Task::try_new("Buy milk", "2L semi-skimmed", Priority::Low, None).unwrap();
    store.save(&[t.clone()]).unwrap();

    let loaded = store.load().unwrap();
    assert_eq!(loaded.len(), 1);
    assert_eq!(loaded[0].title, "Buy milk");
    assert_eq!(loaded[0].priority, Priority::Low);
    // round-trip should not invent a due date
    assert!(loaded[0].due.is_none());
    // sanity: created_at survives serialisation
    assert!((Utc::now() - loaded[0].created_at).num_seconds() < 60);
}

#[test]
fn loading_a_missing_file_returns_empty() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("does_not_exist.json");
    let store = JsonStore::new(&path);
    let loaded = store.load().unwrap();
    assert!(loaded.is_empty());
}
