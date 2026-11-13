use csvsentry::validation::column_types::ColumnType;

mod common;
#[test]
fn test_integer() {
    // Valid values
    assert!(ColumnType::Integer.validate("123").is_ok());
    assert!(ColumnType::Integer.validate("-12").is_ok());
    assert!(ColumnType::Integer.validate("012").is_ok());
    assert!(ColumnType::Integer.validate("0").is_ok());

    // Invalid values
    assert!(ColumnType::Integer.validate("12a").is_err());
    assert!(ColumnType::Integer.validate("!12").is_err());
    assert!(ColumnType::Integer.validate("!_12").is_err());
}
fn test_float() {
    // Valid values
    assert!(ColumnType::Float.validate("12.0").is_ok());
    assert!(ColumnType::Float.validate("12.1").is_ok());
    assert!(ColumnType::Float.validate("-12.0").is_ok());
    assert!(ColumnType::Float.validate("-.1").is_ok());
    assert!(ColumnType::Float.validate("-1").is_ok());
    assert!(ColumnType::Float.validate("1").is_ok());

    // Invalid values
    assert!(ColumnType::Float.validate("12.0").is_err());
    assert!(ColumnType::Float.validate("12.1").is_err());
}

#[test]
fn test_text() {
    let invalid_strings = vec![
        String::from("/"),
        String::from("_"),
        String::from(","),
        String::from("!"),
        String::from("?"),
        String::from("Test"),
        String::from("TEST"),
        String::from("test"),
    ];

    // Valid values
    assert!(
        ColumnType::Text(invalid_strings.clone())
            .validate("123")
            .is_ok()
    );
    assert!(
        ColumnType::Text(invalid_strings.clone())
            .validate("This is a trick")
            .is_ok()
    );
    assert!(
        ColumnType::Text(invalid_strings.clone())
            .validate("Herr")
            .is_ok()
    );
    assert!(ColumnType::Text(Vec::new()).validate("test").is_ok());
    assert!(ColumnType::Text(Vec::new()).validate("TEST").is_ok());
    assert!(ColumnType::Text(Vec::new()).validate("Test").is_ok());

    // Invalid values

    for invalid in &invalid_strings {
        assert!(
            ColumnType::Text(invalid_strings.clone())
                .validate(invalid)
                .is_err(),
            "expected invalid option to be rejected: {invalid}"
        );
    }
    assert!(
        ColumnType::Text(invalid_strings.clone())
            .validate("12Test")
            .is_err()
    );
    assert!(
        ColumnType::Text(invalid_strings.clone())
            .validate("hello/world")
            .is_err()
    );
    assert!(
        ColumnType::Text(invalid_strings.clone())
            .validate("a_b,c")
            .is_err()
    );
    assert!(
        ColumnType::Text(invalid_strings.clone())
            .validate("what?")
            .is_err()
    );
}

#[test]
fn test_email() {}

#[test]
fn test_phone() {}
