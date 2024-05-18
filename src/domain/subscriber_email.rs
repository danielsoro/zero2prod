use validator::ValidateEmail;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if ValidateEmail::validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not a valid subscriber email", s))
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

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn a_valid_email_is_parsed_succesfull() {
        let email = "daniel@domain.com".to_string();
        assert_ok!(SubscriberEmail::parse(email));
    }
}
