use crate::{duration::Duration, Result};
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Time {
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl Time {
    pub fn new(year: u32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> Result<Self> {
        if !matches!(month, 1..=12) {
            return Err("Invalid month".into());
        }
        let days_month = if Time::is_leap_year(year) {
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 31, 31]
        } else {
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 31, 31]
        };
        if day < 1 || day > days_month[month as usize - 1] {
            return Err("Invalid day".into());
        }
        if hour > 23 {
            return Err("Invalid hour".into());
        }
        if minute > 59 {
            return Err("Invalid minute".into());
        }
        if second > 59 {
            return Err("Invalid second".into());
        }
        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }

    fn is_leap_year(year: u32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    }

    pub fn to_iso(&self) -> u64 {
        let unix_start = Time::new(1970, 1, 1, 0, 0, 0).unwrap();
        let days_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        const SECONDS_PER_DAY: u64 = 24 * 60 * 60;
        let mut days = 0;
        for year in unix_start.year..self.year {
            days += if Time::is_leap_year(year) { 366 } else { 365 };
        }
        for month in 1..self.month {
            days += days_month[month as usize - 1] as u64;
        }
        if self.month > 2 && Time::is_leap_year(self.year) {
            days += 1;
        }
        days += self.day as u64 - 1;
        let mut seconds = days * SECONDS_PER_DAY;
        seconds += self.hour as u64 * 60 * 60;
        seconds += self.minute as u64 * 60;
        seconds += self.second as u64;
        seconds
    }

    pub fn from_iso(seconds: u64) -> Self {
        let unix_start = Time::new(1970, 1, 1, 0, 0, 0).unwrap();
        let mut seconds = seconds;
        let mut year = unix_start.year;
        let mut month = 1;
        let mut day = 1;
        let mut hour = 0;
        let mut minute = 0;
        const SECONDS_PER_DAY: u64 = 24 * 60 * 60;
        while seconds >= SECONDS_PER_DAY {
            let days = if Time::is_leap_year(year) { 366 } else { 365 };
            if seconds >= days * SECONDS_PER_DAY {
                seconds -= days * SECONDS_PER_DAY;
                year += 1;
            } else {
                break;
            }
        }
        let days_month = if Time::is_leap_year(year) {
            [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 31, 31]
        } else {
            [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 31, 31]
        };
        while seconds >= SECONDS_PER_DAY {
            if seconds >= days_month[month as usize - 1] as u64 * SECONDS_PER_DAY {
                seconds -= days_month[month as usize - 1] as u64 * SECONDS_PER_DAY;
                month += 1;
            } else {
                break;
            }
        }
        while seconds >= SECONDS_PER_DAY {
            if seconds >= SECONDS_PER_DAY {
                seconds -= SECONDS_PER_DAY;
                day += 1;
            } else {
                break;
            }
        }
        while seconds >= 60 * 60 {
            if seconds >= 60 * 60 {
                seconds -= 60 * 60;
                hour += 1;
            } else {
                break;
            }
        }
        while seconds >= 60 {
            if seconds >= 60 {
                seconds -= 60;
                minute += 1;
            } else {
                break;
            }
        }
        let second = seconds as u8;
        Self {
            year,
            month: month as u8,
            day: day as u8,
            hour: hour as u8,
            minute: minute as u8,
            second,
        }
    }

    pub fn now() -> Self {
        use std::time::SystemTime;
        let gmt_offset = Duration::from_seconds(5 * 60 * 60 + 30 * 60);
        let now = SystemTime::now();
        let unix_time = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
        let seconds = unix_time.as_secs() + gmt_offset.to_seconds();
        Time::from_iso(seconds)
    }
}

impl Add<Duration> for Time {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        let seconds = self.to_iso() + rhs.to_seconds();
        Time::from_iso(seconds)
    }
}

impl AddAssign<Duration> for Time {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub<Duration> for Time {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        let seconds = self.to_iso() - rhs.to_seconds();
        Time::from_iso(seconds)
    }
}

impl SubAssign<Duration> for Time {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let month = match self.month {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "Jun",
            7 => "Jul",
            8 => "Aug",
            9 => "Sept",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => "???",
        };
        write!(
            f,
            "{:04}-{}-{:02} {:02}:{:02}:{:02}",
            self.year, month, self.day, self.hour, self.minute, self.second
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaps() {
        assert_eq!(Time::is_leap_year(2020), true);
        assert_eq!(Time::is_leap_year(2021), false);
        assert_eq!(Time::is_leap_year(2022), false);
        assert_eq!(Time::is_leap_year(2023), false);
        assert_eq!(Time::is_leap_year(2024), true);
        assert_eq!(Time::is_leap_year(1804), true);
        assert_eq!(Time::is_leap_year(1800), false);
        assert_eq!(Time::is_leap_year(1932), true);
    }

    #[test]
    fn test_iso() {
        let time = Time::new(2023, 11, 27, 3, 18, 52).to_iso();
        let iso = 1701055132;
        assert_eq!(iso, time);
        let time = Time::from_iso(iso);
        assert_eq!(time, Time::new(2023, 11, 27, 3, 18, 52));
    }
}
