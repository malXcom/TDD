use capitalize::Capitalize;
use average::Mean;

use slugify::slugify;

pub fn capitalize(str: &str) -> String {
    return str.capitalize();
}

pub fn average(numbers: &[f64]) -> f64 {
    let mean: Mean = numbers.iter().collect();
    return mean.mean();
}

pub fn slug(str: &str) -> String {
    return slugify!(&str);
}

pub fn clamp(value: i32, min: i32, max: i32) -> i32 {
    return value.clamp(min, max);
}