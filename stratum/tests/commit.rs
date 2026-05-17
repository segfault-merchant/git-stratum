mod common;
use common::repo_fixture;

const SMALL_REPO_COMMIT: &str = "1f99848edadfffa903b8ba1286a935f1b92b2845";

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

#[test]
fn test_project_path() {
    repo_fixture("small_repo", |r| {
        let commit = r.head().unwrap();
        let p = commit.project_path();

        assert!(p.is_dir());
        assert!(p.ends_with("small_repo"));
    })
}

#[test]
fn test_project_name() {
    repo_fixture("small_repo", |r| {
        let commit = r.head().unwrap();

        assert_eq!(commit.project_name(), Some("small_repo"));
    })
}

#[test]
fn test_commit_message() {
    repo_fixture("small_repo", |r| {
        let commit = r.single(SMALL_REPO_COMMIT).unwrap();

        assert_eq!(commit.msg(), Some("add file3\n"));
    })
}

#[test]
fn test_commit_sha() {
    repo_fixture("small_repo", |r| {
        let commit = r.single(SMALL_REPO_COMMIT).unwrap();

        assert_eq!(commit.hash(), SMALL_REPO_COMMIT.to_string());
    })
}
