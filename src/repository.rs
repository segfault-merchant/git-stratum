use std::{path::PathBuf, str::FromStr};

use crate::StratumError;
use crate::commit::Commit;

/// Define state to keep track of during commit iteration
pub struct CommitIterator<'iter> {
    repo: &'iter git2::Repository,
    walker: git2::Revwalk<'iter>,
}

impl<'iter> CommitIterator<'iter> {
    pub fn new(repo: &'iter git2::Repository) -> Result<Self, git2::Error> {
        let mut walker = repo.revwalk()?;
        //TODO: Allow pushing of any arbitrary commit
        walker.push_head()?;

        Ok(Self { repo, walker })
    }
}

impl<'a> Iterator for CommitIterator<'a> {
    type Item = Result<Commit<'a>, git2::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.walker.next()? {
            Ok(oid) => Some(Commit::from_oid(self.repo, oid)),
            Err(e) => Some(Err(e)),
        }
    }
}

pub trait RepositoryMiner {
    fn iter_commits<'repo>(
        &'repo self,
    ) -> Result<impl Iterator<Item = Result<Commit<'repo>, git2::Error>> + 'repo, git2::Error>;
}

pub struct Repository(git2::Repository);

impl Repository {
    pub fn new(repo: git2::Repository) -> Self {
        Self(repo)
    }

    pub fn from_path(path: &str) -> Result<Self, StratumError> {
        // Bare unwarp as PathBuf construction is infallible
        let path = PathBuf::from_str(path).unwrap();
        if !path.is_dir() {
            return Err(StratumError::PathError(
                "{path} is not a directory".to_string(),
            ));
        }

        //TODO: Decide on custom errors
        let repo = git2::Repository::open(path).map_err(StratumError::GitError)?;
        Ok(Self::new(repo))
    }

    pub fn from_remote(_url: &str) -> Self {
        todo!()
    }
}

impl RepositoryMiner for Repository {
    fn iter_commits<'repo>(
        &'repo self,
    ) -> Result<impl Iterator<Item = Result<Commit<'repo>, git2::Error>> + 'repo, git2::Error> {
        CommitIterator::new(&self.0)
    }
}
