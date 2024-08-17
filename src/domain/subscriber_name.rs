use unicode_categories::UnicodeCategories;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse<T: AsRef<str>>(s: T) -> Result<Self, String> {
        let subscriber_name = s.as_ref();
        let is_empty_or_whitespace = subscriber_name.trim().is_empty();
        const MAX_NAME_LENGTH: usize = 256;
        let is_too_long = subscriber_name.graphemes(true).count() > MAX_NAME_LENGTH;
        let contains_forbidden_characters = subscriber_name
            .chars()
            .any(|c| c.is_punctuation() || c.is_symbol() || c.is_control());

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!(
                "{} is not a valid subscriber name.",
                subscriber_name
            ))
        } else {
            Ok(Self(subscriber_name.to_string()))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use claims::{assert_err, assert_ok};

    use crate::domain::SubscriberName;

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ё".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }
    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }
    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ";
        assert_err!(SubscriberName::parse(name));
    }
    #[test]
    fn empty_string_is_rejected() {
        let name = "";
        assert_err!(SubscriberName::parse(name));
    }
    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));
        }
    }
    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin";
        assert_ok!(SubscriberName::parse(name));
    }
}
