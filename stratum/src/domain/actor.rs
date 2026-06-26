use chrono::{DateTime, Utc};
use git2::Signature;
use std::str::FromStr;

use crate::Error;

/// A git actor who exists for the inspected repository
pub struct Actor {
    inner: Signature<'static>,
}

impl FromStr for Actor {
    type Err = Error;

    /// Instantiate an Actor from an author string
    ///
    /// Input is expected to be of the form "name <email>", as no time
    /// information necassarily exists, the actors signature is instantiated
    /// to have been created at epoch i.e. the unix timestamp. This is done as
    /// the probability of an actors signature being valid within a repository
    /// at the time of the unix time stamp is extremely unlikely  
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let malformed = || {
            Error::Git(git2::Error::from_str(
                "malformed actor: expected 'name <email>'",
            ))
        };

        let start_idx = s.find('<').ok_or_else(malformed)?;
        let end_idx = s.find('>').ok_or_else(malformed)?;

        if start_idx >= end_idx {
            return Err(malformed());
        }

        let time = git2::Time::new(0, 0);
        let sig = Signature::new(&s[..start_idx], &s[start_idx + 1..end_idx], &time)
            .map_err(Error::Git)?;

        Ok(Self::new(sig))
    }
}

impl Actor {
    /// Instantiate a new Actor from their signature
    pub fn new(signature: Signature<'_>) -> Self {
        Self {
            inner: signature.to_owned(),
        }
    }

    /// Return the actors name if it exists
    pub fn name(&self) -> Option<&str> {
        self.inner.name()
    }

    /// Return the actors email if it exists
    pub fn email(&self) -> Option<&str> {
        self.inner.email()
    }

    /// Return the timestamp of actor action if it exists
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp_secs(self.inner.when().seconds())
    }

    /// Return the offset from the UTC timestamp in minutes
    pub fn offset(&self) -> i32 {
        self.inner.when().offset_minutes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn actor_factory() -> Actor {
        let sig = Signature::new(
            "test",
            "test@example.com",
            &git2::Time::new(1_600_000_000, -60),
        )
        .unwrap();

        Actor::new(sig)
    }

    #[test]
    fn test_name() {
        let actor = actor_factory();
        assert_eq!(actor.name(), Some("test"));
    }

    #[test]
    fn test_email() {
        let actor = actor_factory();
        assert_eq!(actor.email(), Some("test@example.com"));
    }

    #[test]
    fn test_time() {
        let actor = actor_factory();
        assert_eq!(actor.timestamp().unwrap().timestamp(), 1_600_000_000);
    }

    #[test]
    fn test_offset() {
        let actor = actor_factory();
        assert_eq!(actor.offset(), -60);
    }

    #[test]
    fn test_from_str() {
        let actor = Actor::from_str("test <test@example.com>").unwrap();

        assert_eq!(actor.name(), Some("test"));
        assert_eq!(actor.email(), Some("test@example.com"));
        assert_eq!(actor.timestamp().unwrap().timestamp(), 0);
        assert_eq!(actor.offset(), 0);
    }

    #[test]
    fn test_malformed_from_str() {
        let result = Actor::from_str("some nonsense <>");

        assert!(result.is_err())
    }

    #[test]
    fn test_from_str_no_brackets() {
        // Regression for #76: input without angle brackets used to slice a
        // reverse range and panic, rather than returning an error.
        let result = Actor::from_str("anything with no angled brackets");

        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_missing_close_bracket() {
        let result = Actor::from_str("name <email");

        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_missing_open_bracket() {
        let result = Actor::from_str("name email>");

        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_reversed_brackets() {
        let result = Actor::from_str("name > <email");

        assert!(result.is_err());
    }

    #[test]
    fn test_extended_from_str() {
        let actor = Actor::from_str("test <test@example.com> nonsense").unwrap();

        assert_eq!(actor.name(), Some("test"));
        assert_eq!(actor.email(), Some("test@example.com"));
    }

    #[test]
    fn test_surname() {
        let actor = Actor::from_str("test surname <email>").unwrap();

        assert_eq!(actor.name(), Some("test surname"));
    }
}
