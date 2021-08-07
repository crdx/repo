use std::path::Path;

use ignore::gitignore::{Gitignore, GitignoreBuilder};

#[derive(Debug)]
pub struct Ignorer {
    matcher: Option<Gitignore>,
}

impl Ignorer {
    pub fn noop() -> Self {
        Self { matcher: None }
    }

    pub fn new(path: impl AsRef<Path>) -> Self {
        let mut builder = GitignoreBuilder::new("/");
        builder.add(path);
        let matcher = builder.build().unwrap();

        Self {
            matcher: Some(matcher),
        }
    }

    pub fn is_match(&self, path: &Path) -> bool {
        match &self.matcher {
            Some(matcher) => matcher.matched_path_or_any_parents(path, true).is_ignore(),
            None => false,
        }
    }
}
