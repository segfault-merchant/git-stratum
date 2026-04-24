use std::cell::OnceCell;

use crate::Actor;
use crate::{Error, Repository};

/// A singular git commit for the repository being inspected
// #[derive(Clone)]
pub struct Commit<'repo> {
    inner: git2::Commit<'repo>,
    cache: OnceCell<git2::Diff<'repo>>,
}

impl<'repo> Commit<'repo> {
    /// Instantiate a new Commit object from a git2 commit
    pub fn new(commit: git2::Commit<'repo>) -> Self {
        Self {
            inner: commit.to_owned(),
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
    pub fn insertions(&self, ctx: &'repo Repository) -> Result<usize, Error> {
        Ok(self.stats(ctx)?.insertions())
    }

    /// The number of deletions in the commit
    pub fn deletions(&self, ctx: &'repo Repository) -> Result<usize, Error> {
        Ok(self.stats(ctx)?.deletions())
    }

    /// The total number of lines modified in the commit
    pub fn lines(&self, ctx: &'repo Repository) -> Result<usize, Error> {
        Ok(self.insertions(ctx)? + self.deletions(ctx)?)
    }

    /// The number of files modified in the commit
    pub fn files(&self, ctx: &'repo Repository) -> Result<usize, Error> {
        Ok(self.stats(ctx)?.files_changed())
    }

    //TODO: Should stats also be cached?
    /// Return the git2 Stats from the commits diff
    fn stats(&self, ctx: &'repo Repository) -> Result<git2::DiffStats, Error> {
        let diff = self.diff(ctx)?;
        diff.stats().map_err(Error::Git)
    }

    /// Return the git diff for the current commit within the context of a
    /// repository.
    fn diff(&self, ctx: &'repo Repository) -> Result<&git2::Diff<'repo>, Error> {
        let diff = self.calculate_diff(ctx)?;
        Ok(self.cache.get_or_init(|| diff))
    }

    /// Diff the current commit to it's parent(s) adjusting strategy based on the
    /// number of parents
    fn calculate_diff(&self, ctx: &'repo Repository) -> Result<git2::Diff<'repo>, Error> {
        let this_tree = self.inner.tree().ok();
        let parent_tree = match self.inner.parent_count() {
            0 => None,
            1 => self.inner.parent(0).map_err(Error::Git)?.tree().ok(),
            //TODO: Resolve merge commit process
            _ => return Err(Error::PathError("Placeholder error".to_string())),
        };

        ctx.raw()
            //TODO: Expose opts?
            .diff_tree_to_tree(parent_tree.as_ref(), this_tree.as_ref(), None)
            .map_err(Error::Git)
    }
}

//TODO: Unit test the private functions, requires context of the repository, how?
