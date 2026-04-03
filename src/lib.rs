// For now include everything as pub mod to get errors in IDE.
use git_url_parse::GitUrlParseError;
use thiserror::Error;

pub mod actor;
pub mod commit;
pub mod repository;

pub use actor::Actor;
pub use commit::Commit;
pub use repository::{Local, Remote, Repository};

#[derive(Debug, Error)]
pub enum Error {
    /// An abstraction of git2::Error to raise the error effectively
    #[error(transparent)]
    Git(#[from] git2::Error),

    /// An abstraction of git-url-parse::GitUrlParseError
    #[error(transparent)]
    GitUrlError(#[from] GitUrlParseError),

    /// If a URL can be parsed but is not a valid GitUrl schem
    #[error("URL scheme was {0}, cannot clone URL.")]
    UrlScheme(String),

    /// An error associated with a bad path
    #[error("{0}")]
    PathError(String),
}
