use git_url_parse::GitUrl;
use git2::{Cred, RemoteCallbacks};
use std::{
    env,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::Error;

/// Resolve the given destination if None is given in the option
pub fn resolve_destination<P>(url: &GitUrl, dest: Option<P>) -> PathBuf
where
    P: AsRef<Path>,
{
    match dest {
        // Bare unwrap as PathBuf::from_str claims to be infallible
        Some(p) => p.as_ref().to_path_buf(),
        None => {
            // For a repository structure such as https://github.com/jordan-314/git-stratum/tree/main
            // the path begins after the `/`. When split this will create the format
            // ["", "Owner", "Name", ...]
            let repo_name = url.path().split('/').collect::<Vec<&str>>()[2];
            PathBuf::from_str("/tmp").unwrap().join(repo_name)
        }
    }
}

/// Clone the given repository via the ssh protocol into the given destination
///
/// Fetches the ssh credentials from `${HOME}/.ssh/id_rsa` automatically.
/// Currently no other form of ssh credntials are supported.
pub fn clone_ssh(repo: &str, dest: &Path) -> Result<git2::Repository, Error> {
    // Define callback so SSH credentials can be fetched when needed
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });

    // Prepare fetch options.
    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fo);

    // Clone the project.
    builder.clone(repo, dest).map_err(Error::Git)
}

#[cfg(test)]
mod test {
    use super::*;

    fn url_factory() -> GitUrl {
        GitUrl::parse("https://github.com/jordan-314/git-stratum")
            .expect("Failed to parse valid URL")
    }

    #[test]
    fn test_known_dest_resolution() {
        let url = url_factory();
        let expected_path = std::env::current_dir().expect("Failed to get CWD");

        assert_eq!(
            resolve_destination(&url, Some(&expected_path)),
            expected_path
        );
    }

    #[test]
    fn test_unknown_dest_resolution() {
        let url = url_factory();
        let expected_path = PathBuf::from_str("/tmp/git-stratum").unwrap();

        assert_eq!(resolve_destination(&url, None::<&str>), expected_path);
    }

    //TODO: How to test SSH clone properly??
    #[allow(dead_code)]
    fn test_ssh_clone() {
        todo!();
    }
}
