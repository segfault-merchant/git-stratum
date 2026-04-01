use chrono::{DateTime, Utc};
use git2::Signature;

#[cfg_attr(test, mockall::automock)]
pub trait MinedActor {
    /// Return the actors name if it exists
    fn name(&self) -> Option<String>;

    /// Return the actors email if it exists
    fn email(&self) -> Option<String>;

    /// Return the timestamp of the actors action (authorship/commit)
    fn timestamp(&self) -> Option<DateTime<Utc>>;
}

/// A git actor who exists for the inspected repository
pub struct Actor<'a>(Signature<'a>);

impl<'a> Actor<'a> {
    /// Instantiate a new Actor from their signature
    pub fn new(s: Signature<'a>) -> Self {
        Self(s)
    }
}

impl<'a> MinedActor for Actor<'a> {
    fn name(&self) -> Option<String> {
        self.0.name().map(|s| s.to_string())
    }

    fn email(&self) -> Option<String> {
        self.0.email().map(|s| s.to_string())
    }

    fn timestamp(&self) -> Option<DateTime<Utc>> {
        DateTime::from_timestamp_secs(self.0.when().seconds())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn actor_factory() -> MockMinedActor {
        MockMinedActor::new()
    }

    #[test]
    fn test_name() {
        let mut actor = actor_factory();
        actor.expect_name().return_const("john".to_string());

        assert_eq!(actor.name(), Some("john".to_string()))
    }

    #[test]
    fn test_email() {
        let mut actor = actor_factory();
        actor
            .expect_email()
            .return_const("john@example.com".to_string());

        assert_eq!(actor.email(), Some("john@example.com".to_string()))
    }

    #[test]
    fn test_timestamp() {
        let time = Utc::now();

        let mut actor = actor_factory();
        actor.expect_timestamp().return_const(time);

        assert_eq!(actor.timestamp(), Some(time))
    }
}
