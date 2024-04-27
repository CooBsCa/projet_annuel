use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use super::time_format;

#[derive(Serialize, Deserialize, Debug)]
pub struct StructWithCustomDate {
    // DateTime supports Serde out of the box, but uses RFC3339 format. Provide
    // some custom logic to make it use our desired format.
    #[serde(with = "time_format")]
    pub timestamp: NaiveTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_format() {
        let json_str = r#"
          {
            "timestamp": "08:57"
          }
        "#;

        let data: StructWithCustomDate = serde_json::from_str(json_str).unwrap();
        assert_eq!(data.timestamp, NaiveTime::from_hms_opt(8, 57, 0).unwrap());

        let serialized = serde_json::to_string_pretty(&data).unwrap();
        println!("{}", serialized);
    }
}
