use crate::GitUrl;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

/// Resolve the given dest so be a valid path.
pub fn resolve_destination<P>(url: &GitUrl, dest: Option<P>) -> PathBuf
where
    P: AsRef<Path>,
{
    match dest {
        Some(p) => p.as_ref().to_path_buf(),
        None => PathBuf::from_str("/tmp")
            .unwrap()
            // Bare unwrap as GitUrl should validate that a path exists
            .join(url.split_path().next_back().unwrap()),
    }
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
