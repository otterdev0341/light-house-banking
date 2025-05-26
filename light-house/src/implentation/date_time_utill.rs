use chrono::{DateTime, Utc};

pub fn parse_to_datetime_utc(date_str: &str) -> Result<DateTime<Utc>, String> {
    match date_str.parse::<DateTime<Utc>>() {
        Ok(parsed_date) => Ok(parsed_date),
        Err(err) => Err(format!("Failed to parse date: {}", err)),
    }
}