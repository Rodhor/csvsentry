use csvsentry::validation::column_types::ColumnType;

mod common;
#[test]
fn test_integer() {
    let valid_cases = vec![
        "",
        "123",
        "-12",
        "012",
        "0",
        "-0",
        "000000000000000000",
        "-1",
        "+1",
    ];
    let invalid_cases = vec!["12a", "!12", "!_12", "9999999999999999999999999999"];

    for val in valid_cases {
        assert!(
            ColumnType::Integer.validate(val).is_ok(),
            "Expected integer '{val}' to be valid"
        );
    }

    for val in invalid_cases {
        assert!(
            ColumnType::Integer.validate(val).is_err(),
            "Expected integer '{val}' to be invalid"
        );
    }
}

#[test]
fn test_float() {
    let valid_cases = vec!["12.0", "12.1", "-12.0", "-.1", "-1", "1", ""];
    let invalid_cases = vec!["t12", "t", "12.!", "test", "inf", "1e3", "NaN"];

    for val in valid_cases {
        assert!(
            ColumnType::Float.validate(val).is_ok(),
            "Expected float '{val}' to be valid"
        );
    }

    for val in invalid_cases {
        assert!(
            ColumnType::Float.validate(val).is_err(),
            "Expected float '{val}' to be invalid"
        );
    }
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

    let with_invalid_substrings = ColumnType::Text(invalid_strings.clone());
    let without_invalid_substrings = ColumnType::Text(Vec::new());

    let valid_cases = vec![
        ("", &with_invalid_substrings),
        ("", &without_invalid_substrings),
        ("123", &with_invalid_substrings),
        ("This is a trick", &with_invalid_substrings),
        ("Herr", &with_invalid_substrings),
        ("test", &without_invalid_substrings),
        ("TEST", &without_invalid_substrings),
        ("Test", &without_invalid_substrings),
    ];

    let mut invalid_cases = invalid_strings.clone();
    invalid_cases.extend(vec![
        String::from("12Test"),
        String::from("hello/world"),
        String::from("a_b,c"),
        String::from("what?"),
    ]);

    for (val, col) in valid_cases {
        assert!(
            col.validate(val).is_ok(),
            "Expected text '{val}' to be valid"
        );
    }

    for val in &invalid_cases {
        assert!(
            with_invalid_substrings.validate(val).is_err(),
            "Expected text '{val}' to be invalid"
        );
    }
}
#[test]
fn test_email() {
    let valid_cases = vec![
        "",
        "user@example.com",
        "test.name+tag@example.co.uk",
        "12345@example.com",
        "user-name@example.org",
    ];

    let invalid_cases = vec![
        "user-name@example.org.",
        "plainaddress",
        "@example.com",
        "user@",
        "user..name@example.com",
        ".user@example.com",
        "user.@example.com",
        "user name@example.com",
        "user@ex..ample.com",
    ];

    for val in valid_cases {
        assert!(
            ColumnType::Email.validate(val).is_ok(),
            "Expected email '{val}' to be valid"
        );
    }

    for val in invalid_cases {
        assert!(
            ColumnType::Email.validate(val).is_err(),
            "Expected email '{val}' to be invalid"
        );
    }
}

#[test]
fn test_phone() {
    let valid_cases = vec![
        "",
        "+49 123 456789",
        "+44 20 7946 0999",
        "1234567890",
        "+1 555 0199",
        "+123",
        "+ 123",
    ];

    let invalid_cases = vec![
        "+49-123-456",
        "(555) 123-4567",
        "49+123456",
        "+49 123a 456",
        "+49.123.456",
    ];

    // Valid cases
    for val in valid_cases {
        let result = ColumnType::PhoneNumber.validate(val);
        assert!(result.is_ok(), "Expected '{val}' to be valid")
    }

    // Invalid cases
    for val in invalid_cases {
        let result = ColumnType::PhoneNumber.validate(val);
        assert!(result.is_err(), "Expected '{val}' to be invalid")
    }
}

#[test]
fn test_salutation() {
    let valid_cases = vec![
        "", "Herr", "herr", "HERR", "Frau", "frau", "FRAU", "Firma", "firma", "FIRMA",
    ];

    let invalid_cases = vec![
        "Name",
        "+44 20 7946 0999",
        "user@example.com",
        "test",
        "random string",
    ];

    // Valid cases
    for val in valid_cases {
        let result = ColumnType::Salutation.validate(val);
        assert!(result.is_ok(), "Expected '{val}' to be valid")
    }

    // Invalid cases
    for val in invalid_cases {
        let result = ColumnType::Salutation.validate(val);
        assert!(result.is_err(), "Expected '{val}' to be invalid")
    }
}
#[test]
fn test_website() {
    let valid_cases = vec![
        "",
        "www.google.com",
        "https://example.com/",
        "http://example.com",
        "https://www.example.co.uk/path?query=1",
        "https://subdomain.example.org",
        "http://example.com:8080",
    ];

    let invalid_cases = vec![
        "example.com",
        "http://",
        "not-a-url",
        "http://ex..ample.com",
        "https://",
        "Name",
        "+44 20 7946 0999",
        "user@example.com",
        "test",
        "random string",
        "http://user:pass@example.com",
        "https://example.com.",
    ];

    // Valid cases
    for val in valid_cases {
        let result = ColumnType::Website.validate(val);
        assert!(result.is_ok(), "Expected '{val}' to be valid")
    }

    // Invalid cases
    for val in invalid_cases {
        let result = ColumnType::Website.validate(val);
        assert!(result.is_err(), "Expected '{val}' to be invalid")
    }
}
