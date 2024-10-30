use std::num::ParseIntError;
use std::str::FromStr;

use chrono::{Datelike, Local, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

pub use datepicker::*;

pub mod datepicker;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum TimeRange {
    #[default]
    CurrentDay,
    LastThreeDays,
    LastSevenDays,
    LastMilli(u64),
    Custom(u64, u64),
}

pub fn get_start_end_milli(utc_t: u64) -> [u64; 2] {
    let dt =
        Local.from_utc_datetime(&(Utc.timestamp_millis_opt(utc_t as i64).unwrap().naive_utc()));
    let start = Local
        .with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 0, 0, 0)
        .unwrap();
    let end = Local
        .with_ymd_and_hms(dt.year(), dt.month(), dt.day(), 23, 59, 59)
        .unwrap();
    [
        start.timestamp_millis() as u64,
        end.timestamp_millis() as u64 + 999,
    ]
}

impl TimeRange {
    pub fn get_range(&self) -> [u64; 2] {
        match self {
            TimeRange::CurrentDay => get_start_end_milli(get_timestamp()),
            TimeRange::LastThreeDays => {
                let [t1, t2] = get_start_end_milli(get_timestamp());
                [t1 - 2 * 24 * 3600 * 1000, t2]
            }
            TimeRange::LastSevenDays => {
                let [t1, t2] = get_start_end_milli(get_timestamp());
                [t1 - 6 * 24 * 3600 * 1000, t2]
            }
            TimeRange::LastMilli(t) => {
                let t1 = get_timestamp();
                [t1 - t, t1]
            }
            TimeRange::Custom(t1, t2) => [*t1, *t2],
        }
    }

    pub fn is_customer(&self) -> bool {
        matches!(self, TimeRange::Custom(_, _))
    }
}

impl FromStr for TimeRange {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<&str> = s.split('-').collect();
        let t_type = chars[0].parse::<usize>().unwrap();
        let t = match t_type {
            1 => TimeRange::CurrentDay,
            2 => TimeRange::LastThreeDays,
            3 => TimeRange::LastSevenDays,
            4 => {
                let t1 = chars[1].parse::<u64>()?;
                TimeRange::LastMilli(t1)
            }
            5 => {
                let t2 = chars[1].parse::<u64>()?;
                let t3 = chars[2].parse::<u64>()?;
                TimeRange::Custom(t2, t3)
            }
            _ => TimeRange::CurrentDay,
        };
        Ok(t)
    }
}

impl ToString for TimeRange {
    fn to_string(&self) -> String {
        match self {
            TimeRange::CurrentDay => "1-0-0".to_string(),
            TimeRange::LastThreeDays => "2-0-0".to_string(),
            TimeRange::LastSevenDays => "3-0-0".to_string(),
            TimeRange::LastMilli(t1) => format!("4-{}-0", t1.clone()),
            TimeRange::Custom(t2, t3) => format!("5-{}-{}", t2.clone(), t3.clone()),
        }
    }
}

#[inline]
pub fn get_timestamp() -> u64 {
    Utc::now().timestamp_millis() as u64
}

#[inline]
pub fn get_format_time(timestamp: &u64, format: &str) -> String {
    let dt = Local.from_utc_datetime(
        &(Utc
            .timestamp_millis_opt(*timestamp as i64)
            .unwrap()
            .naive_utc()),
    );
    dt.format(format).to_string()
}

#[inline]
pub fn get_utc_time(local_time: &str, format: &str) -> Option<u64> {
    let dt = NaiveDateTime::parse_from_str(local_time, format).ok()?.and_local_timezone(Local).single().unwrap();
    Some(dt.timestamp() as u64)
}

#[test]
fn test_get_time() {
    let t = get_utc_time("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S");
    assert_eq!(t.is_some(), true);
    let t = t.unwrap();
    let l = Local
        .with_ymd_and_hms(2015, 9, 05, 23, 56, 4)
        .unwrap();
    let t2 = l.timestamp() as u64;
    assert_eq!(t, t2);
}