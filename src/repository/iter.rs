// Define any and all iterators relating to the repository class.
use crate::commit::Commit;

/// Define state to keep track of during commit iteration
pub struct CommitIterator<'iter> {
    repo: &'iter git2::Repository,
    walker: git2::Revwalk<'iter>,
}

impl<'iter> CommitIterator<'iter> {
    pub fn new(repo: &'iter git2::Repository) -> Result<Self, git2::Error> {
        let mut walker = repo.revwalk()?;
        //TODO: Allow pushing of any arbitrary commit
        walker.push_head()?;

        Ok(Self { repo, walker })
    }
}

impl<'a> Iterator for CommitIterator<'a> {
    type Item = Result<Commit<'a>, git2::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.walker.next()? {
            Ok(oid) => Some(Commit::from_oid(self.repo, oid)),
            Err(e) => Some(Err(e)),
        }
    }
}
