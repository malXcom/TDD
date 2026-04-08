use average::Mean;
use capitalize::Capitalize;

use slugify::slugify;

#[must_use]
pub fn capitalize(str: &str) -> String {
    str.capitalize()
}

#[must_use]
pub fn average(numbers: &[f64]) -> f64 {
    let mean: Mean = numbers.iter().collect();
    (mean.mean() * 100.0).round() / 100.0
}

#[must_use]
pub fn slug(str: &str) -> String {
    slugify!(&str)
}

#[must_use]
pub fn clamp(value: i32, min: i32, max: i32) -> i32 {
    value.clamp(min, max)
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
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "name" => Some(Self::Name),
            "grade" => Some(Self::Grade),
            "age" => Some(Self::Age),
            _ => None,
        }
    }
}

enum Order {
    Ascendant,
    Descendant,
}

impl Order {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "asc" => Some(Self::Ascendant),
            "desc" => Some(Self::Descendant),
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
        Some([]) | None => return vec![],
        Some(s) => s.iter().collect(),
    };

    let Some(sorted) = sort_by.and_then(SortBy::from_str) else {
        return vec![];
    };
    let Some(ordered) = Order::from_str(order.unwrap_or("asc")) else {
        return vec![];
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
