//! Public library entry point. Keeping logic in a library crate (separate from
//! `main.rs`) lets integration tests and benchmarks link against it directly.

#![deny(rust_2018_idioms)]
#![warn(clippy::all)]
#![allow(clippy::module_name_repetitions)]

pub mod models;
pub mod services;
pub mod storage;
pub mod patterns;
pub mod cli;

pub use models::task::{Priority, Task, TaskId};
