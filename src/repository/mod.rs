use std::{marker::PhantomData, path::Path};

use crate::{Commit, Error, GitUrl};

mod utils;

/// Unit struct indicating that a repository is remote to the file system
pub struct Remote;
/// Unit struct indicating that a repository is local to the file system
pub struct Local;

/// A Git Repository which can be mined.
///
/// Two marker variations Local and Remote. Local is the variant which exists on
/// the local filesystem and can therefore be mined. Remote, is of course not on
/// the local filesystem and is represented via its remote url. A remote Repository
/// upon instantiation will be cloned and returned as a Local variant such that it
/// can be mined.
///
/// ## Examples
///
/// 1. Traversing a local repository from HEAD
///
/// ```no_run
/// # use std::{path::PathBuf, str::FromStr};
/// use stratum::{Repository, Local};
///
/// let p = PathBuf::from_str("~/repository/").unwrap();
/// let repo = Repository::<Local>::new(p).unwrap();
///
/// for commit in repo.traverse_commits().unwrap() {
///     let commit = commit.unwrap();
///     println!("Commit {} was authored by {:?}", commit.hash(), commit.author().name());
/// }
/// ```
///
/// 2. Traversing a remote repository from HEAD
///
/// ```no_run
/// # use std::{path::PathBuf, str::FromStr};
/// use stratum::{Repository, Remote};
///
/// let repo = Repository::<Remote>::new::<PathBuf>(
///         "https://server.example/owner/repo.git",
///         None
///     ).unwrap();
/// for commit in repo.traverse_commits().unwrap() {
///     let commit = commit.unwrap();
///     println!("Commit {} was authored by {:?}", commit.hash(), commit.author().name());
/// }
/// ```
///
/// 3. Extracting HEAD from a local repository
///
/// ```no_run
/// # use std::{path::PathBuf, str::FromStr};
/// use stratum::{Repository, Local};
///
/// let p = PathBuf::from_str("~/repository/").unwrap();
/// let repo = Repository::<Local>::new(p).unwrap();
///
/// println!("HEAD was authored by {:?}", repo.head().unwrap().author().name());
/// ```
///
/// 4. Using the helper functions
///
/// ```no_run
/// # use std::{path::PathBuf, str::FromStr};
/// use stratum::open_repository;
///
/// let p = PathBuf::from_str("~/repository/").unwrap();
/// // use clone_repository for the remote helper fucntion
/// let repo = open_repository(p);
///
/// println!("HEAD was authored by {:?}", repo.unwrap().head().unwrap().author().name());
/// ```
pub struct Repository<Location = Local> {
    repo: git2::Repository,
    location: PhantomData<Location>,
}

impl Repository<Local> {
    /// Instatiate a new Repository from a path on the local file system
    pub fn new<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        if !path.as_ref().is_dir() {
            return Err(Error::PathError("{path} is not a directory".to_string()));
        }

        let git_repo = git2::Repository::open(path).map_err(Error::Git)?;
        Ok(Self {
            repo: git_repo,
            location: PhantomData::<Local>,
        })
    }

    /// Read access into the underlying git2 object
    pub fn raw(&self) -> &git2::Repository {
        &self.repo
    }

    /// Traverse the repositories commit graph from HEAD
    pub fn traverse_commits(
        &self,
    ) -> Result<impl Iterator<Item = Result<Commit<'_>, Error>>, Error> {
        let mut walker = self.raw().revwalk().map_err(Error::Git)?;
        walker.push_head().map_err(Error::Git)?;
        self.iterate_walker(walker)
    }

    /// Traverse the repositories commit graph from a specified commit hash
    pub fn traverse_from(
        &self,
        oid: &str,
    ) -> Result<impl Iterator<Item = Result<Commit<'_>, Error>>, Error> {
        let oid = git2::Oid::from_str(oid).map_err(Error::Git)?;

        let mut walker = self.raw().revwalk().map_err(Error::Git)?;
        walker.push(oid).map_err(Error::Git)?;
        self.iterate_walker(walker)
    }

    /// Return head as a stratum commit
    pub fn head(&self) -> Result<Commit<'_>, Error> {
        let head = self
            .repo
            .head()
            .map_err(Error::Git)?
            .peel_to_commit()
            .map_err(Error::Git)?;
        Ok(Commit::new(head, self))
    }

    /// Return a single commit object based on a given oid/hash
    pub fn single(&self, oid: &str) -> Result<Commit<'_>, Error> {
        let git_commit = self
            .repo
            .find_commit(git2::Oid::from_str(oid).map_err(Error::Git)?)
            .map_err(Error::Git)?;
        Ok(Commit::new(git_commit, self))
    }

    fn iterate_walker(
        &self,
        walker: git2::Revwalk<'_>,
    ) -> Result<impl Iterator<Item = Result<Commit<'_>, Error>>, Error> {
        Ok(walker.map(|result| {
            result.map_err(Error::Git).and_then(|oid| {
                self.raw()
                    .find_commit(oid)
                    .map_err(Error::Git)
                    .map(|git_commit| Commit::new(git_commit, self))
            })
        }))
    }
}

impl Repository<Remote> {
    /// Instatiate a new Repository from a remote URL, returning the Local
    /// variant after cloning the repository into `dest`.
    ///
    /// Type of clone to perform will be automatically resolved based on the
    /// URL.
    pub fn new<P>(url: &str, dest: Option<P>) -> Result<Repository<Local>, Error>
    where
        P: AsRef<Path>,
    {
        let git_url = GitUrl::parse(url)?;
        // If ok_or block is hit, then scheme is None, hence pass string version
        // of None for a useful error message
        let scheme = git_url
            .scheme()
            .ok_or(Error::UrlScheme("None".to_string()))?;

        match scheme {
            "http" | "https" => Repository::from_https(url, dest),
            "ssh" => Repository::from_ssh(url, dest),
            _ => Err(Error::UrlScheme(scheme.to_string())),
        }
    }

    /// Clone the given repository via the http or https protocol into the given
    /// destination
    pub fn from_https<P>(url: &str, dest: Option<P>) -> Result<Repository<Local>, Error>
    where
        P: AsRef<Path>,
    {
        // Don't shadow url, slice needed to clone repo
        let git_url = GitUrl::parse(url)?;
        let dest = utils::resolve_destination(&git_url, dest);

        let git_repo = git2::Repository::clone(url, dest).map_err(Error::Git)?;

        Ok(Repository {
            repo: git_repo,
            location: PhantomData::<Local>,
        })
    }

    pub fn from_ssh<P>(_url: &str, _dest: Option<P>) -> Result<Repository<Local>, Error>
    where
        P: AsRef<Path>,
    {
        todo!(
            "SSH cloning is not yet supported, attempt cloning via a http/https URL or clone manually"
        )
    }
}

// Define a helper function to be used in testing
#[cfg(test)]
impl Repository<Local> {
    /// Instantiate a repository from a git2::Repository
    pub fn from_repository(repo: git2::Repository) -> Self {
        Self {
            repo,
            location: PhantomData::<Local>,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_fail_on_bad_dir() {
        let fp = NamedTempFile::new().expect("Failed to make tempfile");
        assert!(Repository::<Local>::new(fp.path()).is_err())
    }

    #[test]
    fn test_fail_on_bad_git_dir() {
        let dir = TempDir::new().expect("Failed to make tempdir");
        assert!(Repository::<Local>::new(dir.path()).is_err())
    }

    //TODO: Should I test cloning and so on here or in integration tests?
}
