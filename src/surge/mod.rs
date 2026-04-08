#[derive(Debug, PartialEq, Clone, Copy, Eq)]
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
            "monday" => Ok(Self::Monday),
            "tuesday" => Ok(Self::Tuesday),
            "wednesday" => Ok(Self::Wednesday),
            "thursday" => Ok(Self::Thursday),
            "friday" => Ok(Self::Friday),
            "saturday" => Ok(Self::Saturday),
            "sunday" => Ok(Self::Sunday),
            _ => Err(()),
        }
    }
}

impl DayOfWeek {
    const fn is_weekday(self) -> bool {
        matches!(
            self,
            Self::Monday | Self::Tuesday | Self::Wednesday | Self::Thursday
        )
    }

    const fn is_friday_saturday(self) -> bool {
        matches!(self, Self::Friday | Self::Saturday)
    }
}

#[must_use]
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
