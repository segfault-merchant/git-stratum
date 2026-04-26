use std::str::Split;

use crate::Error;

/// A thin wrapper around git_url_parse::GitUrl.
///
/// Designed to implement only what is needed within git-stratum and to preserve
/// the original url string parsed as it is not traditionally preserved.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GitUrl {
    inner: git_url_parse::GitUrl,
    url_str: String,
}

impl GitUrl {
    /// Parse the input string into a URL || Error
    pub fn parse(s: &str) -> Result<Self, Error> {
        let url = git_url_parse::GitUrl::parse(s).map_err(Error::GitUrlError)?;
        Ok(Self {
            inner: url,
            url_str: s.to_string(),
        })
    }

    /// Return the input URL string
    pub fn raw(&self) -> &str {
        &self.url_str
    }

    /// Return the URL scheme
    pub fn scheme(&self) -> Option<&str> {
        self.inner.scheme()
    }

    /// Return the URL path
    pub fn path(&self) -> &str {
        self.inner.path()
    }

    /// Split the path on the delimeter '/'
    pub fn split_path(&self) -> Split<'_, char> {
        self.inner.path().trim_start_matches('/').split('/')
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        assert!(GitUrl::parse("https://server.example/owner/git-stratum.git").is_ok());
        assert!(GitUrl::parse("git@server.example:owner/git-stratum.git").is_ok());
        assert!(GitUrl::parse("rubbish-url$@./").is_err());
        assert!(GitUrl::parse("https://server.example").is_err());
    }

    #[test]
    fn test_raw() {
        let raw = "https://server.example/owner/git-stratum.git";
        let url = GitUrl::parse(raw).unwrap();

        assert_eq!(url.raw(), raw)
    }

    #[test]
    fn test_scheme() {
        let raw = "https://server.example/owner/git-stratum.git";
        let url = GitUrl::parse(raw).unwrap();

        assert_eq!(url.scheme(), Some("https"))
    }

    #[test]
    fn test_path() {
        let raw = "https://server.example/owner/git-stratum.git";
        let url = GitUrl::parse(raw).unwrap();

        assert_eq!(url.path(), "/owner/git-stratum.git")
    }

    #[test]
    fn test_split_path() {
        let raw = "https://server.example/owner/git-stratum.git";
        let url = GitUrl::parse(raw).unwrap();

        let path_components: Vec<&str> = url.split_path().collect();
        let expected_components = vec!["owner", "git-stratum.git"];
        assert_eq!(path_components, expected_components)
    }
}
