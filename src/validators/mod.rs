use emval::validate_email;
use passwords::analyzer;


pub fn is_valid_email(email: &str) -> bool {
    validate_email(email).is_ok()
}
pub struct PasswordValidation {
    pub valid: bool,
    pub errors: Vec<String>,
}

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

pub fn is_valid_age(age: i32) -> bool {
    return age > 0 && age < 150;
}
