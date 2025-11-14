use anyhow::{Context, Result};
#[cfg(target_family = "unix")]
use libc::statvfs;
#[cfg(target_family = "unix")]
use std::ffi::CString;
use std::fs;
#[cfg(target_family = "unix")]
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub struct TempFileManager {
    root: TempRoot,
    files: Vec<PathBuf>,
    capacity_bytes: u64,
    used_bytes: u64,
}

impl TempFileManager {
    pub fn with_capacity(capacity_bytes: u64, override_dir: Option<&Path>) -> Result<Self> {
        let root = match override_dir {
            Some(path) => TempRoot::External(path.to_path_buf()),
            None => TempRoot::Owned(
                TempDir::new().context("failed to create temporary directory for disk module")?,
            ),
        };
        ensure_free_space(root.path(), capacity_bytes)?;

        Ok(Self {
            root,
            files: Vec::new(),
            capacity_bytes,
            used_bytes: 0,
        })
    }

    pub fn reserve(&mut self, bytes: u64) -> Result<PathBuf> {
        if self.used_bytes + bytes > self.capacity_bytes {
            anyhow::bail!(
                "insufficient temp storage (requested {} bytes, {} remaining)",
                bytes,
                self.capacity_bytes.saturating_sub(self.used_bytes)
            );
        }
        let path = self
            .root
            .path()
            .join(format!("mdperf-disk-{}.dat", self.files.len()));
        self.files.push(path.clone());
        self.used_bytes += bytes;
        Ok(path)
    }

    pub fn cleanup(&mut self) {
        for path in self.files.drain(..) {
            let _ = fs::remove_file(&path);
        }
    }

    #[allow(dead_code)]
    pub fn root(&self) -> &Path {
        self.root.path()
    }
}

impl Drop for TempFileManager {
    fn drop(&mut self) {
        self.cleanup();
    }
}

enum TempRoot {
    Owned(TempDir),
    External(PathBuf),
}

impl TempRoot {
    fn path(&self) -> &Path {
        match self {
            TempRoot::Owned(dir) => dir.path(),
            TempRoot::External(path) => path.as_path(),
        }
    }
}

fn ensure_free_space(path: &Path, required: u64) -> Result<()> {
    if let Some(free) = free_space_bytes(path)
        && free < required
    {
        anyhow::bail!(
            "insufficient free space at {} (need {} MB, have {} MB)",
            path.display(),
            required / 1_048_576,
            free / 1_048_576
        );
    }
    Ok(())
}

#[cfg(target_family = "unix")]
fn free_space_bytes(path: &Path) -> Option<u64> {
    let c_path = CString::new(path.as_os_str().as_bytes()).ok()?;
    let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
    let rc = unsafe { statvfs(c_path.as_ptr(), &mut stat) };
    if rc == 0 {
        Some(stat.f_bavail * stat.f_frsize)
    } else {
        None
    }
}

#[cfg(not(target_family = "unix"))]
fn free_space_bytes(_path: &Path) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::TempDir;

    #[test]
    fn respects_capacity() {
        let temp = TempDir::new().unwrap();
        let mut mgr = TempFileManager::with_capacity(1024, Some(temp.path())).unwrap();
        mgr.reserve(512).unwrap();
        assert!(mgr.reserve(600).is_err());
    }
}
