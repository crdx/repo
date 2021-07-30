use std::path::Path;

use ignore::gitignore::{Gitignore, GitignoreBuilder};

#[derive(Debug)]
pub struct Ignorer {
    matcher: Gitignore,
}

impl Ignorer {
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        let mut builder = GitignoreBuilder::new("/");
        builder.add(path);
        let matcher = builder.build().unwrap();
        Self { matcher }
    }

    pub fn is_match(&self, path: &Path) -> bool {
        self.matcher
            .matched_path_or_any_parents(path, true)
            .is_ignore()
    }
}
