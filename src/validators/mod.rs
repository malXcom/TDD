use passwords::analyzer;
use validator::ValidateEmail;

#[must_use]
pub fn is_valid_email(email: &str) -> bool {
    email.validate_email()
}
pub struct PasswordValidation {
    pub valid: bool,
    pub errors: Vec<String>,
}

#[must_use]
pub fn is_valid_password(password: &str) -> PasswordValidation {
    let analyzed = analyzer::analyze(password);
    let mut errors: Vec<String> = Vec::new();

    if analyzed.length() < 8 {
        errors.push("Password must be at least 8 characters long".to_string());
    }
    if analyzed.uppercase_letters_count() < 1 {
        errors.push("Password must contain at least one uppercase letter".to_string());
    }
    if analyzed.lowercase_letters_count() < 1 {
        errors.push("Password must contain at least one lowercase letter".to_string());
    }
    if analyzed.numbers_count() < 1 {
        errors.push("Password must contain at least one number".to_string());
    }
    if analyzed.symbols_count() < 1 {
        errors.push("Password must contain at least one special character".to_string());
    }
    PasswordValidation {
        valid: errors.is_empty(),
        errors,
    }
}

#[must_use]
pub const fn is_valid_age(age: i32) -> bool {
    age > 0 && age < 150
}
