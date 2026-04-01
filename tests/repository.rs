// Integration tests associated with repository.rs

mod common;
use std::str::FromStr;

use common::make_repo;

use stratum::{MinedActor, MinedCommit, Repository, repository::RepositoryMiner};

#[test]
fn init_repo_from_path() {
    let repo_path = make_repo();
    Repository::from_str(repo_path.path().to_str().unwrap()).expect("Failed to open as repo");
}

#[test]
fn test_commit_traversal() {
    let repo_path = make_repo();
    let repo =
        Repository::from_str(repo_path.path().to_str().unwrap()).expect("Failed to open as repo");

    // Expecting an iter of length one
    let iter = repo.iter_commits().expect("Iterator Error");
    for commit in iter {
        let commit = commit.expect("Expected a valid stratum commit");

        assert_eq!(commit.msg(), Some(common::EXPECTED_MSG.to_string()));

        assert!(!commit.is_merge());
        assert_eq!(commit.parents(), None);
        // Check author equivalence
        assert_eq!(
            commit.author().name(),
            Some(common::EXPECTED_ACTOR_NAME.to_string())
        );
        assert_eq!(
            commit.author().email(),
            Some(common::EXPECTED_ACTOR_EMAIL.to_string())
        );
        // Check committer equivalence
        assert_eq!(
            commit.committer().name(),
            Some(common::EXPECTED_ACTOR_NAME.to_string())
        );
        assert_eq!(
            commit.committer().email(),
            Some(common::EXPECTED_ACTOR_EMAIL.to_string())
        );
    }
}
