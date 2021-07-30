use std::path::{Path, PathBuf};

use crate::fs;
use crate::git;
use crate::ignore::Ignorer;

pub struct RepositoryFilter {
    pub dirty: bool,
    pub unpushed: bool,
    pub ignorer: Ignorer,
}

#[derive(Debug)]
pub struct Repository {
    path: PathBuf,
}

impl Repository {
    pub fn run(&self, args: &[String]) -> bool {
        git::run_in_context(&self.path, args).unwrap_or(false)
    }

    pub fn get_path(&self, base: &Path, absolute: bool) -> &Path {
        if absolute {
            &self.path
        } else {
            self.rel_path(base)
        }
    }

    fn rel_path(&self, base: &Path) -> &Path {
        let name = self.path.strip_prefix(base).unwrap();

        if name.components().next().is_none() {
            Path::new(".")
        } else {
            name
        }
    }
}

pub fn list_from_vec(filter: &RepositoryFilter, base_dir: &Path, paths: Vec<PathBuf>) -> Vec<Repository> {
    let paths = paths
        .into_iter()
        .map(|path| {
            if path.is_absolute() {
                path
            } else {
                base_dir.join(path)
            }
        })
        .collect();

    mkrepos(filter, paths)
}

pub fn list_from_fs(filter: &RepositoryFilter, base_dir: &Path) -> Vec<Repository> {
    mkrepos(filter, fs::list_repos(base_dir).unwrap())
}

fn mkrepos(filter: &RepositoryFilter, paths: Vec<PathBuf>) -> Vec<Repository> {
    use rayon::prelude::*;
    paths
        .into_par_iter()
        .filter_map(|path| {
            if !git::is_actual(&path)
                || (filter.dirty && !git::is_dirty(&path))
                || (filter.unpushed && !git::is_unpushed(&path))
                || filter.ignorer.is_match(&path)
            {
                None
            } else {
                Some(Repository { path })
            }
        })
        .collect()
}
