// Integration tests associated with repository.rs

mod common;
use std::path::PathBuf;

use common::test_data_dir;

use stratum::{Local, Repository};

fn small_repo() -> PathBuf {
    test_data_dir().join("small_repo")
}

/// The hash of HEAD for small_repo
fn head_hash() -> String {
    "da39b1326dbc2edfe518b90672734a08f3c13458".to_string()
}

fn first_hash() -> String {
    "a88c84ddf42066611e76e6cb690144e5357d132c".to_string()
}

#[test]
fn init_repo_from_path() {
    // Same content as 'make_repo' function, this will flag if that causes
    // the other tests to fail
    let p = small_repo();
    Repository::<Local>::new(p).expect("Failed to open as repo");
}

#[test]
fn test_traversal_from_head() {
    let repo_path = small_repo();
    let repo = Repository::<Local>::new(repo_path).expect("Failed to open as repo");

    let mut count: usize = 0;
    for _ in repo.traverse_commits().expect("Iterator Error") {
        count += 1;
    }

    assert_eq!(count, 5)
}

#[test]
fn test_traversal_from_commit() {
    let repo_path = small_repo();
    let repo = Repository::<Local>::new(repo_path).expect("Failed to open as repo");

    let mut count: usize = 0;
    for _ in repo.traverse_from(&first_hash()).expect("Iterator Error") {
        count += 1;
    }

    assert_eq!(count, 1)
}

#[test]
fn test_head() {
    let repo_path = small_repo();
    let repo = Repository::<Local>::new(repo_path).expect("Failed to open as repo");

    let head = repo.head().expect("Failed to create HEAD commit");
    assert_eq!(head.hash(), head_hash());
}

#[test]
fn test_single() {
    let repo_path = small_repo();
    let repo = Repository::<Local>::new(repo_path).expect("Failed to open as repo");

    let commit = repo
        .single(&head_hash())
        .expect("Expected valid hash string");
    assert_eq!(commit.hash(), head_hash());
}

#[test]
fn test_insertions() {
    let repo_path = small_repo();
    let repo = Repository::<Local>::new(repo_path).expect("Failed to open as repo");

    let head = repo.head().expect("Failed to create HEAD commit");
    assert_eq!(head.insertions().unwrap(), 60);
}

#[test]
fn test_deletions() {
    let repo_path = small_repo();
    let repo = Repository::<Local>::new(repo_path).expect("Failed to open as repo");

    let head = repo.head().expect("Failed to create HEAD commit");
    assert_eq!(head.deletions().unwrap(), 63);
}

#[test]
fn test_lines() {
    let repo_path = small_repo();
    let repo = Repository::<Local>::new(repo_path).expect("Failed to open as repo");

    let head = repo.head().expect("Failed to create HEAD commit");
    assert_eq!(head.lines().unwrap(), 123);
}

#[test]
fn test_files_changed() {
    let repo_path = small_repo();
    let repo = Repository::<Local>::new(repo_path).expect("Failed to open as repo");

    let head = repo.head().expect("Failed to create HEAD commit");
    assert_eq!(head.files().unwrap(), 2);
}
