use std::{path::Path, sync::OnceLock};

use crate::Error;

/// A file that was touched in a commit
pub struct ModifiedFile<'c> {
    cache: OnceLock<Option<git2::Patch<'c>>>,
    diff: &'c git2::Diff<'c>,
    n: usize,
}

impl<'c> ModifiedFile<'c> {
    /// Instantiate the modified file object from a git diff.
    ///
    /// As a single diff can have > 1 modified/touched file, a single unsigned
    /// integer is provided to specify the delta and/or patch that this file
    /// looks to represent. Hence, the struct will normally be instantiated via
    /// iterating over the diff deltas as they are readily avaliable. For
    /// example:
    ///
    /// ```no_run
    /// for idx in 0..diff.deltas().len() {
    ///     let mfile = ModifiedFile::new(&diff, idx)
    /// }
    /// ```
    pub fn new(diff: &'c git2::Diff<'_>, n: usize) -> Self {
        ModifiedFile {
            cache: OnceLock::new(),
            diff,
            n,
        }
    }

    /// Return the path of the old file in the diff
    pub fn old_path(&self) -> Result<Option<&Path>, Error> {
        Ok(self.patch()?.and_then(|p| p.delta().old_file().path()))
    }

    /// Return the path of the new file in the diff
    pub fn new_file(&self) -> Result<Option<&Path>, Error> {
        Ok(self.patch()?.and_then(|p| p.delta().new_file().path()))
    }

    /// Return the file status of the given patch
    pub fn status(&self) -> Result<Option<git2::Delta>, Error> {
        Ok(self.patch()?.map(|p| p.delta().status()))
    }

    /// Return the patch given the diff
    //TODO: https://github.com/segfault-merchant/git-stratum/issues/32
    fn patch(&self) -> Result<Option<&git2::Patch<'_>>, Error> {
        let patch = git2::Patch::from_diff(self.diff, self.n)?;
        Ok(self.cache.get_or_init(|| patch).as_ref())
    }
}
