use std::fmt;

#[derive(Debug, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut roll_overred_hours = minutes / 60;
        let mut minutes = minutes % 60;
        if minutes < 0 {
            minutes = 60 + minutes;
            roll_overred_hours = roll_overred_hours - 1;
        }
        let mut hours = (hours + roll_overred_hours) % 24;
        if hours < 0 {
            hours = 24 + hours;
        }

        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
