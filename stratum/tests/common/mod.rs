use std::path::Path;

use stratum::{Local, Repository};
use stratum_utils::test_data_dir;

/// Open a test repository given it's relative path
#[allow(dead_code)] // Code is used but flags all the same?
pub fn repo_fixture<F, R, P>(path: P, f: F) -> R
where
    F: FnOnce(&Repository<Local>) -> R,
    P: AsRef<Path>,
{
    let path = test_data_dir().join(path);
    let repo = Repository::<Local>::new(path).expect("Expected valid repository");
    f(&repo)
}
