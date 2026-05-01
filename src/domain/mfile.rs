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
    /// iterating over the diff deltas as they are readily avaliable. For
    /// example:
    ///
    /// ```no_run
    /// for idx in 0..diff.deltas().len() {
    ///     let mfile = ModifiedFile::new(&diff, idx)
    /// }
    /// ```
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
mod test {}
