use git2::{Oid, Repository};

use crate::actor::{Actor, MinedActor};

// Can't use mockall::automock because of lifetimes :(
pub trait MinedCommit<'a> {
    type Actor: MinedActor + 'a;

    /// Return the commit hash (AKA Oid)
    fn hash(&self) -> String;

    /// Return the commit message if it exists
    fn msg(&self) -> Option<String>;

    /// Return the commit author
    fn author(&'a self) -> Self::Actor;

    /// Return the commit committer
    fn committer(&'a self) -> Self::Actor;

    /// Return the commit parent hashes if they exits
    fn parents(&self) -> Option<Vec<String>>;

    /// Return whether or not the commit is a merge commit
    ///
    /// A merge commit is defined as a commit with more than one parent
    fn is_merge(&self) -> bool;
}

/// A singular git commit for the repository being inspected
#[allow(dead_code)]
pub(crate) struct Commit<'a>(git2::Commit<'a>);

impl<'a> Commit<'a> {
    /// Instantiate a new Commit object from a git2 commit
    #[allow(dead_code)]
    pub fn new(c: git2::Commit<'a>) -> Self {
        Self(c)
    }

    /// Instantiate a new commit object from it's oid
    #[allow(dead_code)]
    pub fn from_oid(repo: &'a Repository, oid: Oid) -> Result<Self, git2::Error> {
        Ok(Self(repo.find_commit(oid)?))
    }
}

impl<'a> MinedCommit<'a> for Commit<'a> {
    type Actor = Actor<'a>;

    fn hash(&self) -> String {
        self.0.id().to_string()
    }

    fn msg(&self) -> Option<String> {
        self.0.message().map(|s| s.to_string())
    }

    fn author(&'a self) -> Self::Actor {
        Actor::new(self.0.author())
    }

    fn committer(&'a self) -> Self::Actor {
        Actor::new(self.0.committer())
    }

    fn parents(&self) -> Option<Vec<String>> {
        if self.0.parent_count() == 0 {
            return None;
        }

        let mut parents: Vec<String> = Vec::new();
        for id in self.0.parent_ids() {
            parents.push(id.to_string());
        }
        Some(parents)
    }

    fn is_merge(&self) -> bool {
        self.0.parent_count() > 1
    }
}

// Manually mock objects necassary for testing the MinedCommit trait.
#[cfg(test)]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MockActor;

#[cfg(test)]
impl crate::actor::MinedActor for MockActor {
    fn name(&self) -> Option<String> {
        Some("test".to_string())
    }
    fn email(&self) -> Option<String> {
        Some("test@example.com".to_string())
    }
    fn timestamp(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use sha1::{Digest, Sha1};

    mock! {
        pub MinedCommit {}

        impl MinedCommit<'_> for MinedCommit {
            type Actor = MockActor;

            fn hash(&self) -> String;
            fn msg(&self) -> Option<String>;
            fn author(&'_ self) -> MockActor;
            fn committer(&'_ self) -> MockActor;
            fn parents(&self) -> Option<Vec<String>>;
            fn is_merge(&self) -> bool;
        }
    }

    fn sha1_factory(v: &[u8]) -> String {
        let mut hasher = Sha1::new();
        hasher.update(v);

        format!("{:x}", hasher.finalize())
    }

    fn commit_factory() -> MockMinedCommit {
        MockMinedCommit::new()
    }

    #[test]
    fn test_hash() {
        let hash = sha1_factory(b"Hello World");

        let v = hash.clone();
        let mut commit = commit_factory();
        commit.expect_hash().return_once(move || v);

        assert_eq!(commit.hash(), hash)
    }

    #[test]
    fn test_msg() {
        let msg = Some("Hello World".to_string());

        let v = msg.clone();
        let mut commit = commit_factory();
        commit.expect_msg().return_once(move || v);

        assert_eq!(commit.msg(), msg)
    }

    #[test]
    fn test_author() {
        let actor = MockActor;

        let v = actor.clone();
        let mut commit = commit_factory();
        commit.expect_author().return_once(move || v);

        assert_eq!(commit.author(), actor)
    }

    #[test]
    fn test_committer() {
        let actor = MockActor;

        let v = actor.clone();
        let mut commit = commit_factory();
        commit.expect_committer().return_once(move || v);

        assert_eq!(commit.committer(), actor)
    }

    #[test]
    fn test_parents() {
        let parents = Some(vec![
            sha1_factory(b"Hello World"),
            sha1_factory(b"Hello There"),
        ]);

        let v = parents.clone();
        let mut commit = commit_factory();
        commit.expect_parents().return_once(move || v);

        assert_eq!(commit.parents(), parents)
    }

    #[test]
    fn test_is_merge() {
        let mut commit = commit_factory();
        commit.expect_is_merge().return_once(|| true);

        assert!(commit.is_merge())
    }
}
