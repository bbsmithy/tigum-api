use chrono::{NaiveDateTime, NaiveDate, NaiveTime};

pub fn create_fake_date() -> NaiveDateTime {
    let date = NaiveDate::from_ymd(2019, 12, 24);
    let time = NaiveTime::from_hms(12, 45, 12);
    NaiveDateTime::new(date, time)
}