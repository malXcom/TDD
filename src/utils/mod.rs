use average::Mean;
use capitalize::Capitalize;

use slugify::slugify;

pub fn capitalize(str: &str) -> String {
    return str.capitalize();
}

pub fn average(numbers: &[f64]) -> f64 {
    let mean: Mean = numbers.iter().collect();
    return (mean.mean() * 100.0).round() / 100.0;
}

pub fn slug(str: &str) -> String {
    return slugify!(&str);
}

pub fn clamp(value: i32, min: i32, max: i32) -> i32 {
    return value.clamp(min, max);
}

pub struct Students {
    pub name: String,
    pub grade: f64,
    pub age: i32,
}

enum SortBy {
    Name,
    Grade,
    Age,
}

impl SortBy {
    fn from_str(s: &str) -> Option<SortBy> {
        match s {
            "name" => Some(SortBy::Name),
            "grade" => Some(SortBy::Grade),
            "age" => Some(SortBy::Age),
            _ => None,
        }
    }
}

enum Order {
    Ascendant,
    Descendant,
}

impl Order {
    fn from_str(s: &str) -> Option<Order> {
        match s {
            "asc" => Some(Order::Ascendant),
            "desc" => Some(Order::Descendant),
            _ => None,
        }
    }
}

pub fn sort_students<'std>(
    students: Option<&'std [Students]>,
    sort_by: Option<&str>,
    order: Option<&str>,
) -> Vec<&'std Students> {
    let mut result: Vec<&'std Students> = match students {
        Some(s) if s.is_empty() => return vec![],
        Some(s) => s.iter().collect(),
        None => return vec![],
    };

    let sorted = match sort_by.and_then(SortBy::from_str) {
        Some(s) => s,
        None => return vec![],
    };

    let _ordered = order.unwrap_or("asc");

    let ordered = match Order::from_str(_ordered) {
        Some(o) => o,
        None => return vec![],
    };

    result.sort_by(|a, b| {
        let cmp = match sorted {
            SortBy::Name => a.name.cmp(&b.name),
            SortBy::Grade => a
                .grade
                .partial_cmp(&b.grade)
                .unwrap_or(std::cmp::Ordering::Equal),
            SortBy::Age => a.age.cmp(&b.age),
        };
        match ordered {
            Order::Ascendant => cmp,
            Order::Descendant => cmp.reverse(),
        }
    });

    result
}
