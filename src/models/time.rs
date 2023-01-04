#[derive(Debug)]
pub struct TimeDifferential {
    pub years: i64,
    pub days: i64,
}

impl TimeDifferential {
    /// Creates a new instance of a time differential struct
    pub fn new(s: i64) -> Self {
        let years = s / 1000 / ((60 * 60 * 24) * 365);
        let days = (s - (1000 * 60 * 60 * 24 * 365 * years)) / 1000 / (60 * 60 * 24);

        TimeDifferential { years, days }
    }
}

impl ToString for TimeDifferential {
    /// Converts a time differential struct to a string
    fn to_string(&self) -> String {
        if self.years > 0 {
            format!("{} years {} days", self.years, self.days)
        } else {
            format!("{} days", self.days)
        }
    }
}
