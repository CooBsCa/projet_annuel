use chrono::NaiveDateTime;

pub struct Period {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}
/// Check if the two periods overlap
pub fn is_period_overlap(period1: &Period, period2: &Period) -> bool {
    period1.start < period2.end && period1.end > period2.start
}

impl Period {
    pub fn new(start: NaiveDateTime, end: NaiveDateTime) -> Self {
        Self { start, end }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_period_overlap() {
        let period1 = Period::new(
            NaiveDateTime::parse_from_str("2021-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2021-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );
        let period2 = Period::new(
            NaiveDateTime::parse_from_str("2021-01-01 11:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2021-01-01 13:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );
        assert!(is_period_overlap(&period1, &period2));

        let period1 = Period::new(
            NaiveDateTime::parse_from_str("2021-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2021-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );
        let period2 = Period::new(
            NaiveDateTime::parse_from_str("2021-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2021-01-01 13:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );
        assert!(!is_period_overlap(&period1, &period2));

        let period1 = Period::new(
            NaiveDateTime::parse_from_str("2021-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2021-01-01 12:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );
        let period2 = Period::new(
            NaiveDateTime::parse_from_str("2021-01-01 09:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
            NaiveDateTime::parse_from_str("2021-01-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
        );
        assert!(!is_period_overlap(&period1, &period2));
    }
}
