use std::{
    fs::{self, File},
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use anyhow::anyhow;

#[derive(Debug, Clone, PartialEq)]
pub struct GoupPath {
    home: PathBuf,
}

impl GoupPath {
    pub fn new() -> anyhow::Result<Self> {
        let home = dirs::home_dir()
            .ok_or_else(|| anyhow!("home dir get failed"))?
            .join(".goup");
        Ok(Self { home })
    }

    pub fn join_path<P: AsRef<Path>>(&self, path: P) -> Self {
        Self {
            home: self.home.join(path),
        }
    }

    pub fn current(&self) -> Self {
        self.join_path("current")
    }

    pub fn cache(&self) -> Self {
        self.join_path("cache")
    }

    pub fn version(&self, ver: &str) -> Self {
        self.join_path(ver)
    }

    pub fn version_go(&self, ver: &str) -> Self {
        self.join_path(ver).join_path("go")
    }

    fn version_dot_unpacked_success<P: AsRef<Path>>(&self, ver: P) -> Self {
        let mut d = self.join_path(ver);
        d.push(".unpacked-success");
        d
    }

    pub fn is_dot_unpacked_success_file_exists<P>(&self, ver: P) -> bool
    where
        P: AsRef<Path>,
    {
        self.version_dot_unpacked_success(&ver).exists()
    }

    pub fn create_dot_unpacked_success_file<P>(&self, ver: P) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let dot_unpacked_success_file = self.version_dot_unpacked_success(&ver);
        let parent = dot_unpacked_success_file.parent();
        if let Some(parent) = parent {
            fs::create_dir_all(parent)?;
        }
        File::create(&dot_unpacked_success_file)?;
        Ok(())
    }
}

impl AsRef<Path> for GoupPath {
    fn as_ref(&self) -> &Path {
        &self.home
    }
}

impl Deref for GoupPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.home
    }
}

impl DerefMut for GoupPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.home
    }
}

impl Default for GoupPath {
    fn default() -> Self {
        Self {
            home: PathBuf::new(),
        }
    }
}
