use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;

        if is_empty_or_whitespace || is_too_long {
            Err(format!("{} is not a valid subscriber email", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use claims::{assert_err, assert_ok};

    use crate::domain::SubscriberEmail;

    #[test]
    fn a_256_grapheme_long_email_is_valid() {
        let email = format!("{}@gmail.com", "ё".repeat(100));
        assert_ok!(SubscriberEmail::parse(email));
    }

    #[test]
    fn a_email_longer_than_256_grapheme_is_rejected() {
        let email = "a".repeat(257);
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn whitespace_only_emails_are_rejected() {
        let email = " ".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberEmail::parse(name));
    }
}
