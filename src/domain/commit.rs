use std::cell::OnceCell;

use crate::Actor;
use crate::{Error, Repository};

/// A singular git commit for the repository being inspected
pub struct Commit<'repo> {
    inner: git2::Commit<'repo>,
    ctx: &'repo Repository,
    cache: OnceCell<git2::Diff<'repo>>,
}

impl<'repo> Commit<'repo> {
    /// Instantiate a new Commit object from a git2 commit
    pub fn new(commit: git2::Commit<'repo>, repository: &'repo Repository) -> Self {
        Self {
            inner: commit.to_owned(),
            ctx: repository,
            cache: OnceCell::new(),
        }
    }

    /// Return the commit hash
    pub fn hash(&self) -> String {
        self.inner.id().to_string()
    }

    /// Return the commit message if it exists
    pub fn msg(&self) -> Option<String> {
        self.inner.message().map(|s| s.to_string())
    }

    /// Return the commit author
    pub fn author(&self) -> Actor {
        Actor::new(self.inner.author())
    }

    /// Return the commit committer
    pub fn committer(&self) -> Actor {
        Actor::new(self.inner.committer())
    }

    /// Retrun the hashes of all commit parents
    pub fn parents(&self) -> impl Iterator<Item = String> {
        self.inner.parent_ids().map(|id| id.to_string())
    }

    /// Return whether the commit is a merge commit
    pub fn is_merge(&self) -> bool {
        self.inner.parent_count() > 1
    }

    /// The number of insertions in the commit
    pub fn insertions(&self) -> Result<usize, Error> {
        Ok(self.stats()?.insertions())
    }

    /// The number of deletions in the commit
    pub fn deletions(&self) -> Result<usize, Error> {
        Ok(self.stats()?.deletions())
    }

    /// The total number of lines modified in the commit
    pub fn lines(&self) -> Result<usize, Error> {
        Ok(self.insertions()? + self.deletions()?)
    }

    /// The number of files modified in the commit
    pub fn files(&self) -> Result<usize, Error> {
        Ok(self.stats()?.files_changed())
    }

    //TODO: Should stats also be cached?
    /// Return the git2 Stats from the commits diff
    fn stats(&self) -> Result<git2::DiffStats, Error> {
        let diff = self.diff()?;
        diff.stats().map_err(Error::Git)
    }

    /// Return the git diff for the current commit within the context of a
    /// repository.
    fn diff(&self) -> Result<&git2::Diff<'repo>, Error> {
        //TODO: mv diff calc into cache get_or_init
        // The above TODO will require https://github.com/rust-lang/rust/issues/109737
        // to be brought into stable, i.e. the fn OnceCell::get_or_try_init is
        // made stable. This is because stratum::Error can't implement `Clone`
        // because of `git2::Error` :(
        let diff = self.calculate_diff()?;
        Ok(self.cache.get_or_init(|| diff))
    }

    /// Diff the current commit to it's parent(s) adjusting strategy based on the
    /// number of parents
    fn calculate_diff(&self) -> Result<git2::Diff<'repo>, Error> {
        let this_tree = self.inner.tree().ok();
        let parent_tree = self.resolve_parent_tree()?;

        self.ctx
            .raw()
            //TODO: Expose opts?
            .diff_tree_to_tree(parent_tree.as_ref(), this_tree.as_ref(), None)
            .map_err(Error::Git)
    }

    /// Resolve to the correct parent tree changing strategies based on number
    /// of parents.
    fn resolve_parent_tree(&self) -> Result<Option<git2::Tree<'_>>, Error> {
        Ok(match self.inner.parent_count() {
            0 => None,
            1 => self.inner.parent(0).map_err(Error::Git)?.tree().ok(),
            //TODO: Resolve merge commit process
            _ => return Err(Error::PathError("Placeholder error".to_string())),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::{EXPECTED_ACTOR_EMAIL, EXPECTED_ACTOR_NAME, EXPECTED_MSG, init_repo};

    #[test]
    fn test_msg() {
        let repo = init_repo();
        let git_commit = repo
            .raw()
            .head()
            .expect("Expected a valid reference")
            .peel_to_commit()
            .expect("Expected a valid git2 commit");
        let commit = Commit::new(git_commit, &repo);

        assert_eq!(commit.msg().unwrap(), EXPECTED_MSG.to_string());
    }

    #[test]
    fn test_author() {
        let repo = init_repo();
        let git_commit = repo
            .raw()
            .head()
            .expect("Expected a valid reference")
            .peel_to_commit()
            .expect("Expected a valid git2 commit");
        let commit = Commit::new(git_commit, &repo);

        assert_eq!(
            commit.author().name().unwrap(),
            EXPECTED_ACTOR_NAME.to_string()
        );
        assert_eq!(
            commit.author().email().unwrap(),
            EXPECTED_ACTOR_EMAIL.to_string()
        );
    }

    #[test]
    fn test_committer() {
        let repo = init_repo();
        let git_commit = repo
            .raw()
            .head()
            .expect("Expected a valid reference")
            .peel_to_commit()
            .expect("Expected a valid git2 commit");
        let commit = Commit::new(git_commit, &repo);

        assert_eq!(
            commit.committer().name().unwrap(),
            EXPECTED_ACTOR_NAME.to_string()
        );
        assert_eq!(
            commit.committer().email().unwrap(),
            EXPECTED_ACTOR_EMAIL.to_string()
        );
    }

    #[test]
    fn test_parents() {
        let repo = init_repo();
        let git_commit = repo
            .raw()
            .head()
            .expect("Expected a valid reference")
            .peel_to_commit()
            .expect("Expected a valid git2 commit");
        let commit = Commit::new(git_commit, &repo);

        assert_eq!(commit.parents().collect::<Vec<String>>().len(), 1);
    }

    #[test]
    fn test_is_merge() {
        let repo = init_repo();
        let git_commit = repo
            .raw()
            .head()
            .expect("Expected a valid reference")
            .peel_to_commit()
            .expect("Expected a valid git2 commit");
        let commit = Commit::new(git_commit, &repo);

        assert!(!commit.is_merge());
    }

    #[test]
    fn test_insertions() {
        let repo = init_repo();
        let git_commit = repo
            .raw()
            .head()
            .expect("Expected a valid reference")
            .peel_to_commit()
            .expect("Expected a valid git2 commit");
        let commit = Commit::new(git_commit, &repo);

        assert_eq!(commit.insertions().unwrap(), 1)
    }

    #[test]
    fn test_deletions() {
        let repo = init_repo();
        let git_commit = repo
            .raw()
            .head()
            .expect("Expected a valid reference")
            .peel_to_commit()
            .expect("Expected a valid git2 commit");
        let commit = Commit::new(git_commit, &repo);

        assert_eq!(commit.deletions().unwrap(), 0)
    }

    #[test]
    fn test_lines() {
        let repo = init_repo();
        let git_commit = repo
            .raw()
            .head()
            .expect("Expected a valid reference")
            .peel_to_commit()
            .expect("Expected a valid git2 commit");
        let commit = Commit::new(git_commit, &repo);

        assert_eq!(commit.lines().unwrap(), 1)
    }

    #[test]
    fn test_stat() {
        let repo = init_repo();
        let git_commit = repo
            .raw()
            .head()
            .expect("Expected a valid reference")
            .peel_to_commit()
            .expect("Expected a valid git2 commit");
        let commit = Commit::new(git_commit, &repo);

        // Won't compile if return type is bad, stat otherwise checked in insertions
        // and deletions test functions
        let _: git2::DiffStats = commit
            .stats()
            .expect("Failed to construct git2 Stats object");
    }
}
