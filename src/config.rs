use std::env;
use std::path::PathBuf;

use crate::ignore::Ignorer;

pub fn get_ignorer() -> Ignorer {
    let mut path = PathBuf::from(env::var_os("HOME").unwrap());
    path.push(".config/repo/ignore");
    Ignorer::new(path)
}
