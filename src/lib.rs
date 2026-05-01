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

    /// Write to a file that exists within a temp directory root
    fn write_fp(root: &TempDir, path: &str, content: &str) {
        let fp = root.path().join(path);
        fs::write(&fp, content).expect("Failed to write to file");
    }

    /// Write a file to a git index
    fn write_to_index(index: &mut git2::Index, file: &str) {
        index
            .add_path(Path::new(file))
            .expect("Failed to add file to index");
        index.write().expect("Failed to write index");
    }

    /// Write an index to a repository tree
    fn write_tree<'a>(
        repo: &'a git2::Repository,
        index: &mut git2::Index,
        file: &str,
    ) -> git2::Tree<'a> {
        write_to_index(index, file);

        let tree_id = index.write_tree().expect("Failed to write tree");
        repo.find_tree(tree_id).expect("Failed to find tree")
    }

    /// Commit a file that has been modified
    fn commit_file(
        repo: &git2::Repository,
        sig: &git2::Signature,
        file: &str,
        parent: Option<&git2::Commit<'_>>,
    ) -> git2::Oid {
        // Stage file for commit
        let mut index = repo.index().expect("Failed to get index");
        let tree = write_tree(repo, &mut index, file);

        let parents = match parent {
            Some(v) => vec![v],
            None => vec![],
        };

        repo.commit(Some("HEAD"), sig, sig, EXPECTED_MSG, &tree, &parents)
            .expect("Failed to create commit")
    }

    /// Make a repository with a very basic histroy.
    fn make_repo(tmpdir: &TempDir) {
        let repo = git2::Repository::init(tmpdir.path()).expect("Failed to init repo");
        let sig = git2::Signature::now(EXPECTED_ACTOR_NAME, EXPECTED_ACTOR_EMAIL)
            .expect("Failed to create actor signature");

        let file = "file.txt";
        write_fp(tmpdir, file, "Hello World\n");
        let first_commit_id = commit_file(&repo, &sig, file, None);

        write_fp(tmpdir, file, "Hello World\nFile Update\n");
        let parent = repo
            .find_commit(first_commit_id)
            .expect("Failed to find first commit");
        commit_file(&repo, &sig, file, Some(&parent));
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
    ///
    /// The repository exists in a temporary diectory and is made with some
    /// expected values that are public constants.
    ///
    /// The commit history is two commits long, two commits so that 2/3 of the
    /// diff creation methods can be tested, this removes the need to mock
    /// git2::Diff, which may not even be possible.
    ///
    /// ## Commit 1
    ///
    /// - One file was added, `file.txt`
    ///     - The string "Hello World\n" was written to this file
    /// - The commit is authored and committed by: test <test@example.com>
    /// - The commit message is: "commit msg"
    ///
    /// ## Commit 2
    ///
    /// - The same file, file.txt is updated
    ///     - The file now contains the string "Hello World\nFile Update\n"
    /// - The commit is authored and committed by: test <test@example.com>
    /// - The commit message is: "commit msg"
    pub fn init_repo() -> Repository<Local> {
        Repository::<Local>::new(test_data_dir()).expect("Failed to init local repository")
    }
}
