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
