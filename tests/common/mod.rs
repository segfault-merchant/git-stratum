use std::{fs, path::Path};
use tempfile::{TempDir, tempdir};

pub const EXPECTED_MSG: &str = "commit msg";
pub const EXPECTED_ACTOR_NAME: &str = "test";
pub const EXPECTED_ACTOR_EMAIL: &str = "test@example.com";

pub fn make_repo() -> TempDir {
    let tmpdir = tempdir().expect("Failed to make tmpdir");

    let repo = git2::Repository::init(&tmpdir).expect("Failed to init repo");

    let fp = tmpdir.path().join("file.txt");
    fs::write(&fp, "Hello World\n").expect("Failed to write to file");

    // Stage file for commit
    let mut index = repo.index().expect("Failed to get index");
    index
        .add_path(Path::new("file.txt"))
        .expect("Failed to add file to index");
    index.write().expect("Failed to write index");

    let tree_id = index.write_tree().expect("Failed to write tree");
    let tree = repo.find_tree(tree_id).expect("Failed to find tree");

    // Define author and committer local to this repo
    let sig = git2::Signature::now(EXPECTED_ACTOR_NAME, EXPECTED_ACTOR_EMAIL)
        .expect("Failed to create actor signature");

    repo.commit(Some("HEAD"), &sig, &sig, EXPECTED_MSG, &tree, &[])
        .expect("Failed to create commit");

    tmpdir
}
