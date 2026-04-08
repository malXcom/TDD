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

impl std::str::FromStr for DayOfWeek {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "monday" => Ok(DayOfWeek::Monday),
            "tuesday" => Ok(DayOfWeek::Tuesday),
            "wednesday" => Ok(DayOfWeek::Wednesday),
            "thursday" => Ok(DayOfWeek::Thursday),
            "friday" => Ok(DayOfWeek::Friday),
            "saturday" => Ok(DayOfWeek::Saturday),
            "sunday" => Ok(DayOfWeek::Sunday),
            _ => Err(()),
        }
    }
}

impl DayOfWeek {
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
    if !(10.0..22.0).contains(&hour) {
        return 0.0;
    }

    if day == DayOfWeek::Sunday {
        return 1.2;
    }

    if day.is_friday_saturday() && hour >= 19.0 {
        return 1.8;
    }

    if day.is_weekday() {
        if (12.0..13.5).contains(&hour) {
            return 1.3;
        }
        if (19.0..21.0).contains(&hour) {
            return 1.5;
        }
    }

    1.0
}
