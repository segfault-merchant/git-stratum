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
        let name = s.split('<').next().unwrap().trim();
        let email = s.split('<').nth(1).unwrap().trim_end_matches('>');
        // As time is unknown but required, generate the UNIX timestamp and flag
        // in the documentation.
        let time = git2::Time::new(0, 0);

        let sig = Signature::new(name, email, &time).map_err(Error::Git)?;
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
    pub fn name(&self) -> Option<String> {
        self.inner.name().map(|s| s.to_string())
    }

    /// Return the actors email if it exists
    pub fn email(&self) -> Option<String> {
        self.inner.email().map(|s| s.to_string())
    }

    /// Return the timestamp of actor action if it exists
    pub fn timestamp(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp_secs(self.inner.when().seconds())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actor() {
        let sig = Signature::new(
            "test",
            "test@example.com",
            &git2::Time::new(1_600_000_000, 0),
        )
        .unwrap();

        let actor = Actor::new(sig);

        assert_eq!(actor.name(), Some("test".to_string()));
        assert_eq!(actor.email(), Some("test@example.com".to_string()));
        assert_eq!(actor.timestamp().unwrap().timestamp(), 1_600_000_000);
    }
}
