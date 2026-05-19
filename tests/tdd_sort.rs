//! TDD evidence — RED step. This test was written before
//! TaskService::sort_by existed; the project did not even compile when this
//! commit was created. See docs/TDD_EVIDENCE.md.

use task_tracker::models::task::{Priority, Task};
use task_tracker::services::task_service::{TaskService, SortKey};
use task_tracker::storage::TaskStore;
use task_tracker::storage::json_storage::JsonStore;
use tempfile::tempdir;

fn svc() -> TaskService<JsonStore> {
    let dir = Box::leak(Box::new(tempdir().unwrap()));
    let store = JsonStore::new(dir.path().join("tasks.json"));
    TaskService::open(store).unwrap()
}

#[test]
fn sort_by_priority_orders_high_first() {
    let mut s = svc();
    s.add(Task::try_new("low",    "", Priority::Low,    None).unwrap());
    s.add(Task::try_new("high",   "", Priority::High,   None).unwrap());
    s.add(Task::try_new("medium", "", Priority::Medium, None).unwrap());

    let sorted = s.sorted(SortKey::PriorityDesc);
    let titles: Vec<&str> = sorted.iter().map(|t| t.title.as_str()).collect();
    assert_eq!(titles, vec!["high", "medium", "low"]);
}
