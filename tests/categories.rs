use std::collections::HashSet;

use naughty_strings::{categories, BLNS, CATEGORIES};

#[test]
fn categories_partition_the_list() {
    let concatenated: Vec<&str> = CATEGORIES
        .iter()
        .flat_map(|c| c.strings.iter().copied())
        .collect();
    assert_eq!(concatenated, BLNS);
}

#[test]
fn categories_are_named_and_not_empty() {
    let mut names = HashSet::new();
    for category in CATEGORIES {
        assert!(!category.name.is_empty());
        assert!(!category.strings.is_empty(), "{} is empty", category.name);
        assert!(names.insert(category.name), "{} repeats", category.name);
    }
}

#[test]
fn empty_string_is_reserved() {
    assert_eq!(BLNS[0], "");
    assert_eq!(categories::RESERVED_STRINGS.strings[0], "");
}

#[test]
fn constants_match_their_section() {
    assert_eq!(categories::SQL_INJECTION.name, "SQL Injection");
    assert!(categories::SQL_INJECTION
        .strings
        .contains(&"1;DROP TABLE users"));
}
