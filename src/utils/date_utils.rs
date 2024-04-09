use chrono::{NaiveDate, NaiveDateTime};

/// get days between two dates
pub fn get_days_between_dates(start: NaiveDate, end: NaiveDate) -> Vec<NaiveDate> {
    let mut days = vec![];
    let mut current = start;
    while current <= end {
        days.push(current);
        if let Some(next) = current.succ_opt() {
            current = next
        }
    }
    days
}

pub fn split_day_into_slots(
    start: NaiveDateTime,
    end: NaiveDateTime,
    duration: i32,
) -> Vec<(NaiveDateTime, NaiveDateTime)> {
    let mut slots = vec![];
    let mut current_start = start;
    let mut current_end = start + chrono::Duration::minutes(duration as i64);
    while current_end <= end {
        slots.push((current_start, current_end));
        current_start += chrono::Duration::minutes(duration as i64);
        current_end += chrono::Duration::minutes(duration as i64);
    }
    slots
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_days_between_date() {
        let start = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2021, 1, 3).unwrap();
        let days = get_days_between_dates(start, end);
        assert_eq!(days.len(), 3);
        assert_eq!(days[0], start);
        assert_eq!(days[1], NaiveDate::from_ymd_opt(2021, 1, 2).unwrap());
        assert_eq!(days[2], end);
    }

    #[test]
    fn test_split_day_into_slot() {
        let start =
            NaiveDateTime::parse_from_str("2021-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2021-01-01 01:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let slots = split_day_into_slots(start, end, 30);
        let s1 = slots[0];
        assert_eq!(s1.0, start);
        assert_eq!(
            s1.1,
            NaiveDateTime::parse_from_str("2021-01-01 00:30:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
        let s2 = slots[1];
        assert_eq!(s2.0, s1.1);
        assert_eq!(
            s2.1,
            NaiveDateTime::parse_from_str("2021-01-01 01:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
        );
    }

    #[test]
    fn test_split_day_into_slot_odd() {
        // 45 minutes
        // We have 15 minutes left, so we should have only one slot
        let start =
            NaiveDateTime::parse_from_str("2021-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let end =
            NaiveDateTime::parse_from_str("2021-01-01 01:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let slots = split_day_into_slots(start, end, 45);
        assert_eq!(slots.len(), 1);
    }
}
