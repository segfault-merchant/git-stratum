// For now include everything as pub mod to get errors in IDE.
use thiserror::Error;

pub mod actor;
pub mod commit;
pub mod repository;

pub use actor::MinedActor;
pub use commit::MinedCommit;
pub use repository::Repository;

#[derive(Debug, Error)]
pub enum StratumError {
    /// An abstraction of git2::Error to raise the error effectively
    #[error("{0}")]
    GitError(git2::Error),

    /// An error associated with a bad path
    #[error("{0}")]
    PathError(String),
}
