use validator::ValidateEmail;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse<T: AsRef<str>>(s: T) -> Result<Self, String> {
        let subscriber_email = s.as_ref();

        if subscriber_email.validate_email() {
            Ok(Self(subscriber_email.to_string()))
        } else {
            Err(format!(
                "{} is not a valid subscriber email.",
                subscriber_email
            ))
        }
    }
}

impl std::fmt::Display for SubscriberEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use claims::{assert_err, assert_ok};
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use quickcheck::Gen;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use super::*;

    #[derive(Debug, Clone)]
    pub struct ValidEmailFixture(String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut Gen) -> Self {
            // Fake expects something which implements rand::Rng, which requires rand::RngCore.
            // quickcheck's Gen does not implement rand::RngCore, so we need a way to interop between the two.
            // We can use the quickcheck's Gen to generate a u64 and then use it to seed a StdRng, which implements rand::RngCore.
            let mut rng = StdRng::seed_from_u64(u64::arbitrary(g));
            let email = SafeEmail().fake_with_rng(&mut rng);

            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_email_is_parsed_successfully(valid_email: ValidEmailFixture) {
        assert_ok!(SubscriberEmail::parse(valid_email.0));
    }

    #[test]
    fn empty_string_is_rejected() {
        let email = "";
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursula.com";
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "ursula@";
        assert_err!(SubscriberEmail::parse(email));
    }
}
