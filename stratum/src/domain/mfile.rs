use git2::{Delta, DiffDelta, Patch};
use lru::LruCache;
use std::{num::NonZeroUsize, path::Path, rc::Rc, sync::Mutex};

use crate::Error;
use crate::domain::{CachedPatch, RcDiff, RcPatch};

const CACHE_KEY: u8 = 1;

/// A file that was touched in a commit
pub struct ModifiedFile<'c> {
    diff: RcDiff<'c>,
    n: usize,
    patch_cache: Mutex<CachedPatch<'c>>,
}

impl<'c> ModifiedFile<'c> {
    /// Instantiate the modified file object from a git diff.
    ///
    /// As a single diff can have > 1 modified/touched file, a single unsigned
    /// integer is provided to specify the delta and/or patch that this file
    /// looks to represent. Hence, the struct will normally be instantiated via
    /// iterating over the diff deltas as they are readily avaliable.
    pub fn new(diff: RcDiff<'c>, n: usize) -> Self {
        let patch_cache = Mutex::new(LruCache::new(NonZeroUsize::new(1).unwrap()));

        ModifiedFile {
            diff,
            n,
            patch_cache,
        }
    }

    /// Return the path of the old file in the diff
    pub fn old_path(&self) -> Option<&Path> {
        self.delta()?.old_file().path()
    }

    /// Return the path of the new file in the diff
    pub fn new_path(&self) -> Option<&Path> {
        self.delta()?.new_file().path()
    }

    /// Return the current filename of the modified file in the commit
    ///
    /// Returns None if neither the old or new filename are valid
    pub fn filename(&self) -> Option<&str> {
        let dev_null = Path::new("/dev/null");
        let path = match self.new_path() {
            Some(p) if p != dev_null => p,
            _ => self.old_path()?,
        };

        path.file_name()?.to_str()
    }

    /// Return the file status of the given patch
    //TODO: Should this return a custom type?? Probably
    pub fn status(&self) -> Option<Delta> {
        Some(self.delta()?.status())
    }

    /// The number of lines added in this modified file
    pub fn insertions(&self) -> Result<usize, Error> {
        // As per git2 docs first entry is insertions
        Ok(match self.patch()? {
            Some(p) => p.line_stats()?.1,
            None => 0,
        })
    }

    /// The number of lines removed in this modified file
    pub fn deletions(&self) -> Result<usize, Error> {
        // As per git2 docs second entry is deletions
        Ok(match self.patch()? {
            Some(p) => p.line_stats()?.2,
            None => 0,
        })
    }

    /// Return the delta associated with the index
    fn delta(&self) -> Option<DiffDelta<'_>> {
        self.diff.get_delta(self.n)
    }

    /// Return the patch given the diff, caching it within the struct
    ///
    /// Returns Ok(None) if the file is unchanged
    fn patch(&self) -> Result<Option<RcPatch<'c>>, Error> {
        let mut guard = self.patch_cache.lock().map_err(|_| Error::PoisonedCache)?;
        let data = guard.try_get_or_insert(CACHE_KEY, || {
            let patch = Patch::from_diff(self.diff.as_ref(), self.n)?;
            match patch {
                Some(p) => Ok::<Option<RcPatch>, Error>(Some(Rc::new(p))),
                None => Ok(None),
            }
        })?;

        Ok(data.as_ref().map(Rc::clone))
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    use super::*;
    use crate::common::init_repo;

    fn mfile_fixture<F, R>(f: F) -> R
    where
        F: FnOnce(&ModifiedFile) -> R,
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

        let diff = Rc::new(diff);
        let mfile = ModifiedFile::new(diff, 0);

        f(&mfile)
    }

    #[test]
    fn test_old_path() {
        mfile_fixture(|mfile| {
            // use mfile here
            assert_eq!(mfile.old_path().unwrap(), "file.txt");
        });
    }

    #[test]
    fn test_new_path() {
        mfile_fixture(|mfile| {
            // use mfile here
            assert_eq!(mfile.new_path().unwrap(), "file.txt");
        });
    }

    #[test]
    fn test_delta() {
        mfile_fixture(|mfile| {
            // use mfile here
            assert_eq!(mfile.new_path().unwrap(), "file.txt");
        });
    }
}
