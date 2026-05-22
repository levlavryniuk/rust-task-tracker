//! JSON file backend. Atomic-ish writes via a tempfile + rename so we don't
//! truncate the existing data file on a partial failure.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::models::task::Task;
use super::TaskStore;

pub struct JsonStore {
    path: PathBuf,
}

impl JsonStore {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self { path: path.as_ref().to_path_buf() }
    }

    fn ensure_parent(&self) {
        if let Some(parent) = self.path.parent() {
            if !parent.as_os_str().is_empty() {
                let _ = fs::create_dir_all(parent);
            }
        }
    }

    fn tmp_path(&self) -> PathBuf { self.path.with_extension("json.tmp") }
}

impl TaskStore for JsonStore {
    fn load(&self) -> Result<Vec<Task>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }
        let bytes = fs::read(&self.path)
            .with_context(|| format!("reading {}", self.path.display()))?;
        if bytes.is_empty() {
            return Ok(Vec::new());
        }
        let tasks: Vec<Task> = serde_json::from_slice(&bytes)
            .with_context(|| format!("parsing {}", self.path.display()))?;
        Ok(tasks)
    }

    fn save(&self, tasks: &[Task]) -> Result<()> {
        self.ensure_parent();
        let tmp = self.tmp_path();
        let mut f = fs::File::create(&tmp)
            .with_context(|| format!("creating {}", tmp.display()))?;
        let buf = serde_json::to_vec_pretty(tasks)?;
        f.write_all(&buf)?;
        let _ = f.sync_all();
        fs::rename(&tmp, &self.path)
            .with_context(|| format!("renaming {} -> {}", tmp.display(), self.path.display()))?;
        Ok(())
    }
}
