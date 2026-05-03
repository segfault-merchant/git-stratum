use git2::{Delta, Diff, DiffDelta, Patch};
use std::{path::Path, sync::OnceLock};

use crate::Error;

/// A file that was touched in a commit
pub struct ModifiedFile<'c> {
    cache: OnceLock<Option<Patch<'c>>>,
    diff: &'c Diff<'c>,
    n: usize,
}

impl<'c> ModifiedFile<'c> {
    /// Instantiate the modified file object from a git diff.
    ///
    /// As a single diff can have > 1 modified/touched file, a single unsigned
    /// integer is provided to specify the delta and/or patch that this file
    /// looks to represent. Hence, the struct will normally be instantiated via
    /// iterating over the diff deltas as they are readily avaliable.
    pub fn new(diff: &'c Diff<'_>, n: usize) -> Self {
        ModifiedFile {
            cache: OnceLock::new(),
            diff,
            n,
        }
    }

    /// Return the path of the old file in the diff
    pub fn old_path(&self) -> Option<&Path> {
        self.delta()?.old_file().path()
    }

    /// Return the path of the new file in the diff
    pub fn new_file(&self) -> Option<&Path> {
        self.delta()?.new_file().path()
    }

    /// Return the file status of the given patch
    //TODO: Should this return a custom type?? Probably
    pub fn status(&self) -> Option<Delta> {
        Some(self.delta()?.status())
    }

    /// Return the delta associated with the index
    fn delta(&self) -> Option<DiffDelta<'_>> {
        self.diff.get_delta(self.n)
    }

    /// Return the patch given the diff, caching it within the struct
    //TODO: https://github.com/segfault-merchant/git-stratum/issues/32
    #[allow(dead_code)]
    fn patch(&self) -> Result<Option<&Patch<'_>>, Error> {
        let patch = Patch::from_diff(self.diff, self.n)?;
        Ok(self.cache.get_or_init(|| patch).as_ref())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::init_repo;

    fn mfile_fixture<F, R>(f: F) -> R
    where
        F: FnOnce(&git2::Diff, &ModifiedFile) -> R,
    {
        let repo = init_repo();

        let c1 = repo
            .head()
            .expect("Failed to fetch HEAD")
            .peel_to_commit()
            .expect("Failed to peel HEAD to commit");
        let c2 = c1.parent(0).expect("Couldn't get parent");

        let diff = repo
            .diff_tree_to_tree(
                Some(&c2.tree().expect("Failed to get tree")),
                Some(&c1.tree().expect("Failed to get tree")),
                None,
            )
            .expect("Failed to make diff");

        let mfile = ModifiedFile::new(&diff, 0);

        f(&diff, &mfile)
    }

    #[test]
    fn test_old_path() {
        mfile_fixture(|_, mfile| {
            // use mfile here
            assert_eq!(mfile.old_path().unwrap(), "file.txt");
        });
    }

    #[test]
    fn test_new_path() {
        mfile_fixture(|_, mfile| {
            // use mfile here
            assert_eq!(mfile.new_file().unwrap(), "file.txt");
        });
    }

    #[test]
    fn test_delta() {
        mfile_fixture(|_, mfile| {
            // use mfile here
            assert_eq!(mfile.new_file().unwrap(), "file.txt");
        });
    }
}
