use password_validator::Password;

#[test]
fn password_is_valid_if_meets_all_requirements() {
    assert_eq!("abcDeFgHiJ*1".to_string().validate(), true);
}

#[test]
fn password_requires_to_be_12_characters_or_longer() {
    assert_eq!("cDeFgHiJ*1".to_string().validate(), false);
}

#[test]
fn password_requires_uppercase_letters() {
    assert_eq!("abcdefghij*1".to_string().validate(), false);
}

#[test]
fn password_requires_lowercase_letters() {
    assert_eq!("ABCDEFGHIJ*1".to_string().validate(), false);
}

#[test]
fn password_requires_special_characted() {
    assert_eq!("abcDeFgHiJs1".to_string().validate(), false);
}