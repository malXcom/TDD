use axum_api::validators;

// ── is_valid_email ───────────────────────────────────────────────────────────

#[test]
fn test_is_valid_email_should_return_true_when_given_valid_email() {
    assert!(validators::is_valid_email("user@google.com"));
}

#[test]
fn test_is_valid_email_should_return_false_when_given_missing_at_symbol() {
    assert!(!validators::is_valid_email("usergoogle.com"));
}

#[test]
fn test_is_valid_email_should_return_false_when_given_missing_domain() {
    assert!(!validators::is_valid_email("user@"));
}

#[test]
fn test_is_valid_email_should_return_false_when_given_empty_string() {
    assert!(!validators::is_valid_email(""));
}

#[test]
fn test_is_valid_email_should_return_true_when_given_subdomain_email() {
    assert!(validators::is_valid_email("user@mail.google.com"));
}

#[test]
fn test_is_valid_email_should_return_false_when_given_double_at_symbol() {
    assert!(!validators::is_valid_email("user@@google.com"));
}

#[test]
fn test_is_valid_email_should_return_false_when_given_spaces_in_email() {
    assert!(!validators::is_valid_email("us er@google.com"));
}

// ── is_valid_password ────────────────────────────────────────────────────────

#[test]
fn test_is_valid_password_should_return_valid_when_given_strong_password() {
    let result = validators::is_valid_password("Str0ng!Pass");
    assert!(result.valid);
    assert!(result.errors.is_empty());
}

#[test]
fn test_is_valid_password_should_return_error_when_given_too_short_password() {
    let result = validators::is_valid_password("Sh0rt!");
    assert!(!result.valid);
    assert!(
        result
            .errors
            .contains(&"Password must be at least 8 characters long".to_string())
    );
}

#[test]
fn test_is_valid_password_should_return_error_when_given_no_uppercase_letter() {
    let result = validators::is_valid_password("str0ng!pass");
    assert!(!result.valid);
    assert!(
        result
            .errors
            .contains(&"Password must contain at least one uppercase letter".to_string())
    );
}

#[test]
fn test_is_valid_password_should_return_error_when_given_no_lowercase_letter() {
    let result = validators::is_valid_password("STR0NG!PASS");
    assert!(!result.valid);
    assert!(
        result
            .errors
            .contains(&"Password must contain at least one lowercase letter".to_string())
    );
}

#[test]
fn test_is_valid_password_should_return_error_when_given_no_number() {
    let result = validators::is_valid_password("Strong!Pass");
    assert!(!result.valid);
    assert!(
        result
            .errors
            .contains(&"Password must contain at least one number".to_string())
    );
}

#[test]
fn test_is_valid_password_should_return_error_when_given_no_special_character() {
    let result = validators::is_valid_password("Str0ngPass");
    assert!(!result.valid);
    assert!(
        result
            .errors
            .contains(&"Password must contain at least one special character".to_string())
    );
}

#[test]
fn test_is_valid_password_should_return_multiple_errors_when_given_empty_string() {
    let result = validators::is_valid_password("");
    assert!(!result.valid);
    assert_eq!(result.errors.len(), 5);
}

// ── is_valid_age ─────────────────────────────────────────────────────────────

#[test]
fn test_is_valid_age_should_return_true_when_given_normal_age() {
    assert!(validators::is_valid_age(25));
}

#[test]
fn test_is_valid_age_should_return_false_when_given_zero() {
    assert!(!validators::is_valid_age(0));
}

#[test]
fn test_is_valid_age_should_return_false_when_given_negative_age() {
    assert!(!validators::is_valid_age(-5));
}

#[test]
fn test_is_valid_age_should_return_false_when_given_age_above_150() {
    assert!(!validators::is_valid_age(151));
}

#[test]
fn test_is_valid_age_should_return_true_when_given_boundary_age_of_1() {
    assert!(validators::is_valid_age(1));
}

#[test]
fn test_is_valid_age_should_return_true_when_given_boundary_age_of_149() {
    assert!(validators::is_valid_age(149));
}

#[test]
fn test_is_valid_age_should_return_false_when_given_very_large_negative_number() {
    assert!(!validators::is_valid_age(i32::MIN));
}

#[test]
fn test_is_valid_age_should_return_false_when_given_very_large_positive_number() {
    assert!(!validators::is_valid_age(i32::MAX));
}
