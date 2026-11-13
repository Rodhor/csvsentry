// tests/common.rs

pub fn create_valid_csv() -> &'static str {
    "\
id,name,age,mail,phone
1,Alice,30,alice@example.com,555-0100
2,Bob,25,bob@example.com,555-0199
3,Charlie,35,charlie@example.com,555-0200
"
}

pub fn create_invalid_csv() -> &'static str {
    "\
id,name,age,mail,phone
1,Alice,thirty,alice@example.com,555-0100
2,Bob,25,invalid-email,555-0199
,Charlie,40,charlie@example.com,555-0200
"
}

pub fn create_csv_with_caret_comma() -> &'static str {
    "\
id^name^age^mail^phone
1^Diana^28^diana@example.com^555-0300
2^Eve^22^eve@example.com^555-0311
"
}