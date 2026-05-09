// Integration tests associated with repository.rs

mod common;
use std::path::Path;

use common::test_data_dir;

use stratum::{Local, Repository};

/// Open a test repository given it's relative path
fn repo_fixture<F, R, P>(path: P, f: F) -> R
where
    F: FnOnce(&Repository<Local>) -> R,
    P: AsRef<Path>,
{
    let path = test_data_dir().join(path);
    let repo = Repository::<Local>::new(path).expect("Expected valid repository");
    f(&repo)
}

#[test]
/// Should capture all local branches, no remote
fn test_branches() {
    repo_fixture("branches_not_merged", |r| {
        let head = r.head().unwrap();
        let b = head
            .branches()
            .unwrap()
            .map(|v| v.unwrap())
            .collect::<Vec<String>>();

        dbg!(&b);

        assert!(b.contains(&"master".to_string()));
        assert!(b.contains(&"b1".to_string()));
        assert!(b.contains(&"b1".to_string()));
    })
}

#[test]
/// Should capture all local branches, no remote
fn test_local_branches() {
    repo_fixture("branches_not_merged", |r| {
        let head = r.head().unwrap();
        let b = head
            .branches()
            .unwrap()
            .map(|v| v.unwrap())
            .collect::<Vec<String>>();

        dbg!(&b);

        assert!(b.contains(&"master".to_string()));
        assert!(b.contains(&"b1".to_string()));
        assert!(b.contains(&"b1".to_string()));
    })
}

#[test]
/// Should capture no branches
fn test_remote_branches() {
    repo_fixture("branches_not_merged", |r| {
        let head = r.head().unwrap();
        let b = head
            .remote_branches()
            .unwrap()
            .map(|v| v.unwrap())
            .collect::<Vec<String>>();

        dbg!(&b);

        assert_eq!(b.len(), 0)
    })
}

#[test]
fn test_in_main_branch() {
    repo_fixture("branches_not_merged", |r| {
        let commit = r.head().expect("Failed to fetch HEAD");
        assert!(commit.in_main().unwrap());
    })
}

#[test]
fn test_not_in_main_branch() {
    repo_fixture("branches_not_merged", |r| {
        // The commit here is the HEAD of branch "b1" which has not been merged
        // into main
        let commit = r
            .single("702d469710d2087e662c210fd0e4f9418ec813fd")
            .expect("Failed to fetch commit");
        assert!(commit.in_main().unwrap());
    })
}
