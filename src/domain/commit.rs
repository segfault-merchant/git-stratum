use regex::Regex;
use std::sync::LazyLock;
use std::{cell::OnceCell, str::FromStr};

use crate::{Actor, Error, ModifiedFile, Repository};

/// Iterate all co-author matches in the haystack string formatting the return
/// string to be formatted as "Name <Email>"
fn iter_co_authors(haystack: &str) -> impl Iterator<Item = &str> {
    const CO_AUTHOR_REGEX: &str = r"(?m)^Co-authored-by: (.*) <(.*?)>$";
    // Regex should always compile hence bare unwrap.
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(CO_AUTHOR_REGEX).unwrap());

    let prefix = "Co-authored-by:";
    RE.find_iter(haystack).map(move |re_match| {
        re_match
            .as_str()
            .strip_prefix(prefix)
            .unwrap_or_default()
            .trim()
    })
}

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
    pub fn msg(&self) -> Option<&str> {
        self.inner.message()
    }

    /// Return the commit author
    pub fn author(&self) -> Actor {
        Actor::new(self.inner.author())
    }

    /// Return the co-authors as listed in the commit message
    ///
    /// Lazilly returning as an iterator means the co-authors, if entered more
    /// than once, will **not** be de-duplicated.
    pub fn co_authors(&self) -> impl Iterator<Item = Result<Actor, Error>> {
        let commit_msg = self.msg().unwrap_or_default();
        iter_co_authors(commit_msg).map(Actor::from_str)
    }

    /// Return the commit committer
    pub fn committer(&self) -> Actor {
        Actor::new(self.inner.committer())
    }

    /// Iterate all utf-8 branch names that the current commit is contained in
    ///
    /// ## Note
    ///
    /// Potentially expensive method. Take caution when using within a loop.
    pub fn branches(&self) -> Result<impl Iterator<Item = Result<String, Error>>, Error> {
        self.branch_iterator(None)
    }

    /// Iterate all **local** utf-8 branch names that the current commit is contained in
    ///
    /// ## Note
    ///
    /// Potentially expensive method. Take caution when using within a loop.
    pub fn local_branches(&self) -> Result<impl Iterator<Item = Result<String, Error>>, Error> {
        let flag = Some(git2::BranchType::Local);
        self.branch_iterator(flag)
    }

    /// Iterate all **remote** utf-8 branch names that the current commit is contained in
    ///
    /// ## Note
    ///
    /// Potentially expensive method. Take caution when using within a loop.
    pub fn remote_branches(&self) -> Result<impl Iterator<Item = Result<String, Error>>, Error> {
        let flag = Some(git2::BranchType::Remote);
        self.branch_iterator(flag)
    }

    /// Retrun the hashes of all commit parents
    pub fn parents(&self) -> impl Iterator<Item = String> {
        self.inner.parent_ids().map(|id| id.to_string())
    }

    /// Return whether the commit is a merge commit
    pub fn is_merge(&self) -> bool {
        self.inner.parent_count() > 1
    }

    /// Checks if the current commit is reachable from "main" or "master"
    pub fn in_main(&self) -> Result<bool, Error> {
        let b = self
            .local_branches()?
            .collect::<Vec<Result<String, Error>>>();
        Ok(b.contains(&Ok("main".to_string())) || b.contains(&Ok("master".to_string())))
    }

    /// Return an iterator over the modified files that belong to a commit
    pub fn mod_files(&self) -> Result<impl Iterator<Item = ModifiedFile<'_>>, Error> {
        let diff = self.diff()?;

        Ok((0..diff.deltas().len()).map(move |n| ModifiedFile::new(diff, n)))
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
    //TODO: https://github.com/segfault-merchant/git-stratum/issues/32
    fn diff(&self) -> Result<&git2::Diff<'repo>, Error> {
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

    /// Check if a commit contains a branch
    ///
    /// If an error occurs returns false, this is done so any erroring branches
    /// are filtered out of any dependant processes
    fn commit_contains_branch(&self, branch: git2::Oid, commit: git2::Oid) -> bool {
        self.ctx.raw().graph_descendant_of(branch, commit).is_ok()
    }

    /// Iterate over the specified branch types, None will return all branches
    fn branch_iterator(
        &self,
        bt: Option<git2::BranchType>,
    ) -> Result<impl Iterator<Item = Result<String, Error>>, Error> {
        let commit_id = self.inner.id();
        let branches = self.ctx.raw().branches(bt).map_err(Error::Git)?;

        Ok(branches.filter_map(move |res| {
            let branch = match res {
                Ok(v) => v.0,
                Err(e) => return Some(Err(Error::Git(e))),
            };

            // If a branch does not have a valid target then filter that
            // branch out
            // TODO: Is this excluding a subset of symbolic references
            let oid = match branch.get().target() {
                Some(v) => v,
                None => return None,
            };

            // Filter out a branch if the commit does NOT contain it
            if !self.commit_contains_branch(oid, commit_id) {
                return None;
            }

            match branch.name() {
                Ok(Some(name)) => Some(Ok(name.to_string())),
                Ok(None) => None, // drop non-utf8 branches
                Err(e) => Some(Err(Error::Git(e))),
            }
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        Local, Repository,
        common::{EXPECTED_ACTOR_EMAIL, EXPECTED_ACTOR_NAME, EXPECTED_MSG, init_repo},
    };

    fn commit_fixture<F, R>(f: F) -> R
    where
        F: FnOnce(&Repository<Local>, &Commit) -> R,
    {
        let repo = init_repo();

        let repo = Repository::<Local>::from_repository(repo);
        let commit = repo.head().expect("Failed to get HEAD");

        f(&repo, &commit)
    }

    #[test]
    fn test_msg() {
        commit_fixture(|_, commit| {
            // use mfile here
            assert_eq!(commit.msg(), Some(EXPECTED_MSG));
        });
    }

    #[test]
    fn test_author() {
        commit_fixture(|_, commit| {
            assert_eq!(
                commit.author().name().unwrap(),
                EXPECTED_ACTOR_NAME.to_string()
            );
            assert_eq!(
                commit.author().email().unwrap(),
                EXPECTED_ACTOR_EMAIL.to_string()
            );
        });
    }

    #[test]
    fn test_co_authors() {
        commit_fixture(|_, commit| {
            for co_auth in commit.co_authors() {
                assert!(co_auth.is_ok());
            }
        });
    }

    #[test]
    fn test_committer() {
        commit_fixture(|_, commit| {
            assert_eq!(
                commit.committer().name().unwrap(),
                EXPECTED_ACTOR_NAME.to_string()
            );
            assert_eq!(
                commit.committer().email().unwrap(),
                EXPECTED_ACTOR_EMAIL.to_string()
            );
        });
    }

    #[test]
    fn test_parents() {
        commit_fixture(|_, commit| {
            assert_eq!(commit.parents().collect::<Vec<String>>().len(), 1);
        });
    }

    #[test]
    fn test_is_merge() {
        commit_fixture(|_, commit| {
            assert!(!commit.is_merge());
        });
    }

    #[test]
    fn test_insertions() {
        commit_fixture(|_, commit| {
            assert_eq!(commit.insertions().unwrap(), 1);
        });
    }

    #[test]
    fn test_deletions() {
        commit_fixture(|_, commit| {
            assert_eq!(commit.deletions().unwrap(), 0);
        });
    }

    #[test]
    fn test_lines() {
        commit_fixture(|_, commit| {
            assert_eq!(commit.lines().unwrap(), 1);
        });
    }

    #[test]
    fn test_stat() {
        commit_fixture(|_, commit| {
            // Won't compile if return type is bad, stat otherwise checked in insertions
            // and deletions test functions
            let _: git2::DiffStats = commit
                .stats()
                .expect("Failed to construct git2 Stats object");
        });
    }

    #[test]
    fn test_iter_matches() {
        let haystack = "Co-authored-by: John <john@example.com>";
        assert_eq!(iter_co_authors(haystack).collect::<Vec<&str>>().len(), 1);

        let haystack = "No matches expected";
        assert_eq!(iter_co_authors(haystack).collect::<Vec<&str>>().len(), 0);
    }
}
