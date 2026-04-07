use axum_api::utils;

// ── capitalize ──────────────────────────────────────────────────────────────

#[test]
fn test_capitalize_should_lowercase_all_but_first_when_given_mixed_case() {
    let str = "WordToBeCapitlize";
    assert_eq!(utils::capitalize(str), "Wordtobecapitlize");
}

#[test]
fn test_capitalize_should_return_single_uppercase_when_given_single_char() {
    let str = "a";
    assert_eq!(utils::capitalize(str), "A");
}

#[test]
fn test_capitalize_should_return_empty_string_when_given_empty_input() {
    let str = "";
    assert_eq!(utils::capitalize(str), "");
}

#[test]
fn test_capitalize_should_uppercase_first_when_given_all_lowercase() {
    let str = "hello world";
    assert_eq!(utils::capitalize(str), "Hello world");
}

// ── average ─────────────────────────────────────────────────────────────────

#[test]
fn test_average_should_return_correct_mean_when_given_mixed_floats() {
    let numbers = vec![1.0, 6.0, 4.0, 3.0, 2.0, 1.0, 4.0, 9.0, 8.0, 7.0];
    assert_eq!(utils::average(&numbers), 4.5);
}

#[test]
fn test_average_should_return_value_itself_when_given_single_element() {
    let numbers = vec![42.0];
    assert_eq!(utils::average(&numbers), 42.0);
}

#[test]
fn test_average_should_return_zero_when_given_all_zeros() {
    let numbers = vec![0.0, 0.0, 0.0];
    assert_eq!(utils::average(&numbers), 0.0);
}

#[test]
fn test_average_should_return_correct_mean_when_given_negative_values() {
    let numbers = vec![-4.0, -2.0, 1.0, 2.0, 4.0];
    assert_eq!(utils::average(&numbers), 0.2);
}

// ── slug ────────────────────────────────────────────────────────────────────

#[test]
fn test_slugify_should_lowercase_and_hyphenate_when_given_mixed_case_words() {
    let str = "Words to be SLUGged";
    assert_eq!(utils::slug(str), "words-to-be-slugged");
}

#[test]
fn test_slugify_should_return_single_word_when_given_no_spaces() {
    let str = "Rust";
    assert_eq!(utils::slug(str), "rust");
}

#[test]
fn test_slugify_should_return_empty_string_when_given_empty_input() {
    let str = "";
    assert_eq!(utils::slug(str), "");
}

#[test]
fn test_slugify_should_strip_special_chars_when_given_punctuation() {
    let str = "Hello, World!";
    assert_eq!(utils::slug(str), "hello-world");
}

// ── clamp ───────────────────────────────────────────────────────────────────

#[test]
fn test_clamp_should_return_min_when_value_is_below_range() {
    assert_eq!(utils::clamp(150, 200, 400), 200);
}

#[test]
fn test_clamp_should_return_max_when_value_is_above_range() {
    assert_eq!(utils::clamp(500, 200, 400), 400);
}

#[test]
fn test_clamp_should_return_value_itself_when_value_is_within_range() {
    assert_eq!(utils::clamp(300, 200, 400), 300);
}

#[test]
fn test_clamp_should_return_boundary_when_value_equals_min_or_max() {
    assert_eq!(utils::clamp(200, 200, 400), 200);
    assert_eq!(utils::clamp(400, 200, 400), 400);
}

// ── sort_students ───────────────────────────────────────────────────────────────────

#[test]
fn test_sort_students_should_sort_by_grade_ascending() {
    let students: Vec<utils::Students> = vec![
        utils::Students {
            name: "Alice".to_string(),
            grade: 14.0,
            age: 20,
        },
        utils::Students {
            name: "Lou".to_string(),
            grade: 7.0,
            age: 7,
        },
        utils::Students {
            name: "Romain".to_string(),
            grade: 10.0,
            age: 7,
        },
    ];
    let sorted_students = utils::sort_students(Some(&students), Some("grade"), Some("asc"));
    assert_eq!(sorted_students[0].name, "Lou");
    assert_eq!(sorted_students[1].name, "Romain");
    assert_eq!(sorted_students[2].name, "Alice");
}

#[test]
fn test_sort_students_should_sort_by_grade_descending() {
    let students: Vec<utils::Students> = vec![
        utils::Students {
            name: "Alice".to_string(),
            grade: 14.0,
            age: 20,
        },
        utils::Students {
            name: "Lou".to_string(),
            grade: 7.0,
            age: 7,
        },
        utils::Students {
            name: "Romain".to_string(),
            grade: 10.0,
            age: 14,
        },
    ];
    let sorted_students = utils::sort_students(Some(&students), Some("grade"), Some("desc"));
    assert_eq!(sorted_students[2].name, "Lou");
    assert_eq!(sorted_students[1].name, "Romain");
    assert_eq!(sorted_students[0].name, "Alice");
}

#[test]
fn test_sort_students_should_sort_by_name_ascending() {
    let students: Vec<utils::Students> = vec![
        utils::Students {
            name: "Alice".to_string(),
            grade: 14.0,
            age: 20,
        },
        utils::Students {
            name: "Lou".to_string(),
            grade: 7.0,
            age: 7,
        },
        utils::Students {
            name: "Romain".to_string(),
            grade: 10.0,
            age: 14,
        },
    ];
    let sorted_students = utils::sort_students(Some(&students), Some("name"), Some("asc"));
    assert_eq!(sorted_students[0].name, "Alice");
    assert_eq!(sorted_students[1].name, "Lou");
    assert_eq!(sorted_students[2].name, "Romain");
}

#[test]
fn test_sort_students_should_sort_by_age_ascending() {
    let students: Vec<utils::Students> = vec![
        utils::Students {
            name: "Alice".to_string(),
            grade: 14.0,
            age: 20,
        },
        utils::Students {
            name: "Lou".to_string(),
            grade: 7.0,
            age: 7,
        },
        utils::Students {
            name: "Romain".to_string(),
            grade: 10.0,
            age: 14,
        },
    ];
    let sorted_students = utils::sort_students(Some(&students), Some("age"), Some("asc"));
    assert_eq!(sorted_students[0].name, "Lou");
    assert_eq!(sorted_students[1].name, "Romain");
    assert_eq!(sorted_students[2].name, "Alice");
}

#[test]
fn test_sort_students_should_return_empty_array_for_empty_input() {
    let students = vec![];
    let sorted_students = utils::sort_students(Some(&students), Some("name"), Some("asc"));
    assert!(sorted_students.is_empty());
}

#[test]
fn test_sort_students_should_return_empty_array_for_null_input() {
    let students: Vec<utils::Students> = vec![
        utils::Students {
            name: "Alice".to_string(),
            grade: 14.0,
            age: 20,
        },
        utils::Students {
            name: "Lou".to_string(),
            grade: 7.0,
            age: 7,
        },
        utils::Students {
            name: "Romain".to_string(),
            grade: 10.0,
            age: 14,
        },
    ];
    let sorted_students = utils::sort_students(Some(&students), None, Some("asc"));
    assert!(sorted_students.is_empty());
}

#[test]
fn test_sort_students_should_default_to_ascending_order() {
    let students: Vec<utils::Students> = vec![
        utils::Students {
            name: "Alice".to_string(),
            grade: 14.0,
            age: 20,
        },
        utils::Students {
            name: "Lou".to_string(),
            grade: 7.0,
            age: 7,
        },
        utils::Students {
            name: "Romain".to_string(),
            grade: 10.0,
            age: 14,
        },
    ];
    let sorted_students = utils::sort_students(Some(&students), Some("grade"), None);
    assert_eq!(sorted_students[0].name, "Lou");
    assert_eq!(sorted_students[1].name, "Romain");
    assert_eq!(sorted_students[2].name, "Alice");
}
