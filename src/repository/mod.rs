use std::{marker::PhantomData, path::Path};

use crate::{Commit, Error};
use git_url_parse::GitUrl;
use git2;

mod iter;
use iter::CommitIterator;

mod remote_utils;

pub struct Remote;
pub struct Local;

/// A Git Repository which can be mined.
///
/// Two marker variations Local and Remote. Local is the variant which exists on
/// the local filesystem and can therefore be mined. Remote, is of course not on
/// the local filesystem and is represented via its remote url. A remote Repository
/// upon instantiation will be cloned and returned as a Local variant such that it
/// can be mined.
pub struct Repository<Location = Local> {
    #[allow(dead_code)]
    git_repo: git2::Repository,
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
            git_repo,
            location: PhantomData::<Local>,
        })
    }

    #[allow(dead_code)]
    pub fn iter_commits<'repo>(
        &'repo self,
    ) -> Result<impl Iterator<Item = Result<Commit<'repo>, git2::Error>> + 'repo, git2::Error> {
        CommitIterator::new(&self.git_repo)
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
        let git_url = GitUrl::parse(url).map_err(Error::GitUrlError)?;
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
        let git_url = GitUrl::parse(url).map_err(Error::GitUrlError)?;
        let dest = remote_utils::resolve_destination(&git_url, dest);

        let git_repo = git2::Repository::clone(url, dest).map_err(Error::Git)?;

        Ok(Repository {
            git_repo,
            location: PhantomData::<Local>,
        })
    }

    pub fn from_ssh<P>(url: &str, dest: Option<P>) -> Result<Repository<Local>, Error>
    where
        P: AsRef<Path>,
    {
        let git_url = GitUrl::parse(url).map_err(Error::GitUrlError)?;
        let dest = remote_utils::resolve_destination(&git_url, dest);

        let git_repo = remote_utils::clone_ssh(url, &dest)?;

        Ok(Repository {
            git_repo,
            location: PhantomData::<Local>,
        })
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
