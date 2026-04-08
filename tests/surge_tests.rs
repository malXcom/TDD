use axum_api::surge::{DayOfWeek, calculate_surge};

// ── each multiplier ───────────────────────────────────────────────────────────

#[test]
fn test_calculate_surge_should_return_1_0_when_given_weekday_normal_hours() {
    assert_eq!(calculate_surge(15.0, DayOfWeek::Tuesday), 1.0);
}

#[test]
fn test_calculate_surge_should_return_1_3_when_given_weekday_lunch_hours() {
    assert_eq!(calculate_surge(12.5, DayOfWeek::Wednesday), 1.3);
}

#[test]
fn test_calculate_surge_should_return_1_5_when_given_weekday_dinner_hours() {
    assert_eq!(calculate_surge(20.0, DayOfWeek::Thursday), 1.5);
}

#[test]
fn test_calculate_surge_should_return_1_8_when_given_friday_evening() {
    assert_eq!(calculate_surge(21.0, DayOfWeek::Friday), 1.8);
}

#[test]
fn test_calculate_surge_should_return_1_8_when_given_saturday_evening() {
    assert_eq!(calculate_surge(20.0, DayOfWeek::Saturday), 1.8);
}

#[test]
fn test_calculate_surge_should_return_1_2_when_given_sunday() {
    assert_eq!(calculate_surge(14.0, DayOfWeek::Sunday), 1.2);
}

// ── closed hours ──────────────────────────────────────────────────────────────

#[test]
fn test_calculate_surge_should_return_0_when_given_before_opening() {
    assert_eq!(calculate_surge(9.0, DayOfWeek::Monday), 0.0);
}

#[test]
fn test_calculate_surge_should_return_0_when_given_after_closing() {
    assert_eq!(calculate_surge(23.0, DayOfWeek::Monday), 0.0);
}

// ── boundary transitions ──────────────────────────────────────────────────────

#[test]
fn test_calculate_surge_should_return_1_0_when_given_exactly_10h() {
    assert_eq!(calculate_surge(10.0, DayOfWeek::Monday), 1.0);
}

#[test]
fn test_calculate_surge_should_return_0_when_given_exactly_22h() {
    assert_eq!(calculate_surge(22.0, DayOfWeek::Monday), 0.0);
}

#[test]
fn test_calculate_surge_should_return_1_3_when_given_exactly_12h() {
    assert_eq!(calculate_surge(12.0, DayOfWeek::Monday), 1.3);
}

#[test]
fn test_calculate_surge_should_return_1_0_when_given_exactly_13h30() {
    assert_eq!(calculate_surge(13.5, DayOfWeek::Tuesday), 1.0);
}

#[test]
fn test_calculate_surge_should_return_1_5_when_given_exactly_19h_on_weekday() {
    assert_eq!(calculate_surge(19.0, DayOfWeek::Wednesday), 1.5);
}

#[test]
fn test_calculate_surge_should_return_1_8_when_given_exactly_19h_on_friday() {
    assert_eq!(calculate_surge(19.0, DayOfWeek::Friday), 1.8);
}
