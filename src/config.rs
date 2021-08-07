use std::env;
use std::path::PathBuf;

use crate::ignore::Ignorer;

pub fn get_ignorer(all: bool) -> Ignorer {
    if all {
        Ignorer::noop()
    } else {
        let mut path = PathBuf::from(env::var_os("HOME").unwrap());
        path.push(".config/repo/ignore");
        Ignorer::new(path)
    }
}
