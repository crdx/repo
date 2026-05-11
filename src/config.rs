use std::env;
use std::path::PathBuf;

use crate::ignore::Ignorer;

pub fn get_ignorer(all: bool) -> Ignorer {
    if all {
        Ignorer::noop()
    } else {
        let home = PathBuf::from(env::var_os("HOME").unwrap());

        let base = home.join(".config/org.crdx/repo");
        let base = if base.exists() {
            base
        } else {
            home.join(".config/repo")
        };

        Ignorer::new(base.join("ignore"))
    }
}
