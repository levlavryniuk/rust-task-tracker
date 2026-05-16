//! Thin binary entry point. All real logic lives in the library; `main` is
//! kept tiny so it stays trivially testable through the CLI integration tests.

use anyhow::Result;
use clap::Parser;

use task_tracker::cli::commands::{Cli, dispatch};

fn main() -> Result<()> {
    let cli = Cli::parse();
    dispatch(cli)
}
