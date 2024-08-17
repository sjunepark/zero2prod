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

    use super::*;

    #[test]
    fn valid_email_is_parsed_successfully() {
        let email = SafeEmail().fake::<String>();
        assert_ok!(SubscriberEmail::parse(email));
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
