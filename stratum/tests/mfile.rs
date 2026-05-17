mod common;
use common::repo_fixture;

#[test]
fn test_filename() {
    repo_fixture("diff", |r| {
        for mfile in r.head().unwrap().mod_files().unwrap() {
            assert_eq!(mfile.filename(), Some("A.java"));
        }
    })
}

#[test]
fn test_insertions() {
    repo_fixture("diff", |r| {
        for mfile in r.head().unwrap().mod_files().unwrap() {
            assert_eq!(mfile.insertions(), Ok(0));
        }
    })
}

#[test]
fn test_insertions_with_no_changes() {
    // file is deleted and therefore has no modifications or insertions
    repo_fixture("empty_file_changes", |r| {
        for mfile in r.head().unwrap().mod_files().unwrap() {
            assert_eq!(mfile.insertions(), Ok(0));
        }
    })
}

#[test]
fn test_deletions() {
    repo_fixture("diff", |r| {
        for mfile in r.head().unwrap().mod_files().unwrap() {
            assert_eq!(mfile.deletions(), Ok(12));
        }
    })
}

#[test]
fn test_deletions_with_no_changes() {
    // file is deleted and therefore has no modifications or insertions
    repo_fixture("empty_file_changes", |r| {
        for mfile in r.head().unwrap().mod_files().unwrap() {
            assert_eq!(mfile.deletions(), Ok(0));
        }
    })
}
