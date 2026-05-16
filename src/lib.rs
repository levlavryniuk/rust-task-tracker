//! Public library entry point. Keeping logic in a library crate (separate from
//! `main.rs`) lets integration tests and benchmarks link against it directly.

pub mod models;
pub mod services;
pub mod storage;
pub mod patterns;
pub mod cli;

pub use models::task::{Priority, Task, TaskId};
