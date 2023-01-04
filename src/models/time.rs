#[derive(Debug, PartialEq)]
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
        if self.years > 0 && self.days == 0 {
            format!("{} years", self.years)
        } else if self.years > 0 {
            format!("{} years {} days", self.years, self.days)
        } else {
            format!("{} days", self.days)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TimeDifferential;

    const ONE_YEAR: i64 = 1000 * 60 * 60 * 24 * 365;
    const ONE_DAY: i64 = 1000 * 60 * 60 * 24;

    // The constructor contains some very simple logic to convert ms to years + days
    // This is a bit of an antipattern, but I don't want to overcomplicate this
    #[test]
    fn constructor_test() {
        assert_eq!(
            TimeDifferential::new(ONE_DAY),
            TimeDifferential { days: 1, years: 0 }
        );

        assert_eq!(
            TimeDifferential::new(ONE_YEAR),
            TimeDifferential { days: 0, years: 1 }
        );

        assert_eq!(
            TimeDifferential::new(ONE_DAY + ONE_YEAR),
            TimeDifferential { days: 1, years: 1 }
        );
    }

    #[test]
    fn to_string_test() {
        assert_eq!(TimeDifferential::new(ONE_DAY).to_string(), "1 days");
        assert_eq!(TimeDifferential::new(ONE_YEAR * 2).to_string(), "2 years");
        assert_eq!(
            TimeDifferential::new((ONE_YEAR * 5) + (ONE_DAY * 10)).to_string(),
            "5 years 10 days"
        );
    }

    #[test]
    fn to_string_test_omits_years_when_zero() {
        let td = TimeDifferential::new(ONE_DAY);

        assert!(td.to_string().contains("years") == false)
    }

    #[test]
    fn to_string_test_contains_years_when_non_zero() {
        let td = TimeDifferential::new(ONE_YEAR);

        assert!(td.to_string().contains("years"))
    }
}
