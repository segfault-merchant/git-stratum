use std::{
    path::{Path, PathBuf},
    str::FromStr,
};
use tempfile::TempDir;

use std::fs::File;
use std::sync::LazyLock;
use zip::ZipArchive;

use stratum::{Local, Repository};

const ZIP_NAME: &str = "test-repos.zip";
static MAIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

/// Find the zip located alongside the Cargo.toml
fn zip_path() -> PathBuf {
    let man_path = PathBuf::from_str(MAIFEST_DIR).unwrap();
    let p = man_path.parent().unwrap().join(ZIP_NAME);

    dbg!(&p);
    assert!(p.exists());

    p
}

/// Unzip the given zip file and deposit its contents directly into dest
fn unzip_archive(zip_path: &Path, dest: &Path) {
    let file = File::open(zip_path).expect("Expected a valid zip");
    let mut archive = ZipArchive::new(file).expect("Expected to read zip");

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        // Without skipping the first element in the path everything is extracted into
        // /tmp/temp_dir/test-repos/*, brittle if the zip file name ever changed
        //
        // So we strip the `test-repos` from the path, extracting everything out
        // into /tmp/temp_dir
        let out_path = dest.join(
            file.mangled_name()
                .components()
                .skip(1)
                .collect::<PathBuf>(),
        );

        if file.is_dir() {
            std::fs::create_dir_all(&out_path).unwrap();
        } else {
            if let Some(parent) = out_path.parent() {
                std::fs::create_dir_all(parent).unwrap();
            }
            let mut out = std::fs::File::create(&out_path).unwrap();
            std::io::copy(&mut file, &mut out).unwrap();
        }
    }
}

/// Lazily construct the test data into a temp fir that will last the length of
/// a single modules test span.
static TEST_DATA_DIR: LazyLock<TempDir> = LazyLock::new(|| {
    let dir = TempDir::new().expect("Create temp dir");
    let zp = zip_path();

    unzip_archive(&zp, dir.path());

    dir
});

/// The path to the test data directory
pub fn test_data_dir() -> &'static Path {
    TEST_DATA_DIR.path()
}

/// Open a test repository given it's relative path
pub fn repo_fixture<F, R, P>(path: P, f: F) -> R
where
    F: FnOnce(&Repository<Local>) -> R,
    P: AsRef<Path>,
{
    let path = test_data_dir().join(path);
    let repo = Repository::<Local>::new(path).expect("Expected valid repository");
    f(&repo)
}
