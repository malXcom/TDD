#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl DayOfWeek {
    pub fn from_str(s: &str) -> Option<DayOfWeek> {
        match s.to_lowercase().as_str() {
            "monday" => Some(DayOfWeek::Monday),
            "tuesday" => Some(DayOfWeek::Tuesday),
            "wednesday" => Some(DayOfWeek::Wednesday),
            "thursday" => Some(DayOfWeek::Thursday),
            "friday" => Some(DayOfWeek::Friday),
            "saturday" => Some(DayOfWeek::Saturday),
            "sunday" => Some(DayOfWeek::Sunday),
            _ => None,
        }
    }

    fn is_weekday(&self) -> bool {
        matches!(
            self,
            DayOfWeek::Monday | DayOfWeek::Tuesday | DayOfWeek::Wednesday | DayOfWeek::Thursday
        )
    }

    fn is_friday_saturday(&self) -> bool {
        matches!(self, DayOfWeek::Friday | DayOfWeek::Saturday)
    }
}

pub fn calculate_surge(hour: f64, day: DayOfWeek) -> f64 {
    if hour < 10.0 || hour >= 22.0 {
        return 0.0;
    }

    if day == DayOfWeek::Sunday {
        return 1.2;
    }

    if day.is_friday_saturday() && hour >= 19.0 {
        return 1.8;
    }

    if day.is_weekday() {
        if hour >= 12.0 && hour < 13.5 {
            return 1.3;
        }
        if hour >= 19.0 && hour < 21.0 {
            return 1.5;
        }
    }

    1.0
}
