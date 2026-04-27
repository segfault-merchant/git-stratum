use std::path::Path;

// For now include everything as pub mod to get errors in IDE
use git_url_parse::GitUrlParseError;
use thiserror::Error;

mod domain;
mod repository;
mod url;

pub use domain::{actor::Actor, commit::Commit};
pub use repository::{Local, Remote, Repository};
pub use url::GitUrl;

/// Helper function for opening a local repository given a path P
pub fn open_repository<P: AsRef<Path>>(p: P) -> Result<Repository<Local>, Error> {
    Repository::<Local>::new(p)
}

/// Helper function for cloning a remote repository given a url
pub fn clone_repository<P: AsRef<Path>>(
    url: &str,
    dest: Option<P>,
) -> Result<Repository<Local>, Error> {
    Repository::<Remote>::new(url, dest)
}

#[derive(Debug, Error)]
pub enum Error {
    /// An abstraction of git2::Error to raise the error effectively
    #[error(transparent)]
    Git(#[from] git2::Error),

    /// An abstraction of git-url-parse::GitUrlParseError
    #[error(transparent)]
    GitUrlError(#[from] GitUrlParseError),

    /// If a URL can be parsed but is not a valid GitUrl schem
    #[error("URL scheme was {0}, cannot clone URL.")]
    UrlScheme(String),

    /// An error associated with a bad path
    #[error("{0}")]
    PathError(String),
}

/// Common functionality that can be imported into any and all unit tests
/// throughout the library
#[cfg(test)]
mod common {
    use once_cell::sync::Lazy;
    use std::{fs, path::Path};
    use tempfile::TempDir;

    use super::{Local, Repository};

    pub const EXPECTED_MSG: &str = "commit msg";
    pub const EXPECTED_ACTOR_NAME: &str = "test";
    pub const EXPECTED_ACTOR_EMAIL: &str = "test@example.com";

    /// Make a repository a very basic histroy
    fn make_repo(tmpdir: &TempDir) {
        let repo = git2::Repository::init(tmpdir.path()).expect("Failed to init repo");

        let fp = tmpdir.path().join("file.txt");
        fs::write(&fp, "Hello World\n").expect("Failed to write to file");

        // Stage file for commit
        let mut index = repo.index().expect("Failed to get index");
        index
            .add_path(Path::new("file.txt"))
            .expect("Failed to add file to index");
        index.write().expect("Failed to write index");

        let tree_id = index.write_tree().expect("Failed to write tree");
        let tree = repo.find_tree(tree_id).expect("Failed to find tree");

        // Define author and committer local to this repo
        let sig = git2::Signature::now(EXPECTED_ACTOR_NAME, EXPECTED_ACTOR_EMAIL)
            .expect("Failed to create actor signature");

        repo.commit(Some("HEAD"), &sig, &sig, EXPECTED_MSG, &tree, &[])
            .expect("Failed to create commit");
    }

    /// Lazily construct the test data into a temp dir that will last the length of
    /// a single modules test span
    static TEST_DATA_DIR: Lazy<TempDir> = Lazy::new(|| {
        let dir = TempDir::new().expect("Create temp dir");
        make_repo(&dir);
        dir
    });

    /// The path to the test data directory
    fn test_data_dir() -> &'static Path {
        TEST_DATA_DIR.path()
    }

    /// Init a repository object using the lazily constructed git2 repo
    pub fn init_repo() -> Repository<Local> {
        Repository::<Local>::new(test_data_dir()).expect("Failed to init local repository")
    }
}
