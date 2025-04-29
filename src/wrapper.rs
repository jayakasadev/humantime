use core::fmt;
use core::ops::Deref;
use core::str::FromStr;
use core::time::{Duration as CoreDuration};
use chrono::{DateTime, TimeZone, Utc};
use crate::date::{self, format_rfc3339, parse_rfc3339_weak};
use crate::duration::{self, format_duration, parse_duration};

/// A wrapper for duration that has `FromStr` implementation
///
/// This is useful if you want to use it somewhere where `FromStr` is
/// expected.
///
/// See `parse_duration` for the description of the format.
///
/// # Example
///
/// ```
/// use std::time::Duration;
/// let x: Duration;
/// x = "12h 5min 2ns".parse::<humantime::Duration>().unwrap().into();
/// assert_eq!(x, Duration::new(12*3600 + 5*60, 2))
/// ```
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Duration(CoreDuration);

/// A wrapper for SystemTime that has `FromStr` implementation
///
/// This is useful if you want to use it somewhere where `FromStr` is
/// expected.
///
/// See `parse_rfc3339_weak` for the description of the format. The "weak"
/// format is used as it's more pemissive for human input as this is the
/// expected use of the type (e.g. command-line parsing).
///
/// # Example
///
/// ```
/// use chrono::{DateTime, Utc};
/// let x: DateTime<Utc>;
/// x = "2018-02-16T00:31:37Z".parse::<DateTime<Utc>>().unwrap().into();
/// assert_eq!(humantime::format_rfc3339(x).to_string(), "2018-02-16T00:31:37Z");
/// ```
///
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp<Tz: TimeZone>(DateTime<Tz>);

impl AsRef<CoreDuration> for Duration {
    fn as_ref(&self) -> &CoreDuration {
        &self.0
    }
}

impl Deref for Duration {
    type Target = CoreDuration;
    fn deref(&self) -> &CoreDuration {
        &self.0
    }
}

impl From<Duration> for CoreDuration {
    fn from(val: Duration) -> Self {
        val.0
    }
}

impl From<CoreDuration> for Duration {
    fn from(dur: CoreDuration) -> Duration {
        Duration(dur)
    }
}

impl FromStr for Duration {
    type Err = duration::Error;
    fn from_str(s: &str) -> Result<Duration, Self::Err> {
        parse_duration(s).map(Duration)
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_duration(self.0).fmt(f)
    }
}

impl <Tz: TimeZone> AsRef<DateTime<Tz>> for Timestamp<Tz> {
    fn as_ref(&self) -> &DateTime<Tz> {
        &self.0
    }
}

impl <Tz: TimeZone> Deref for Timestamp<Tz> {
    type Target = DateTime<Tz>;
    fn deref(&self) -> &DateTime<Tz> {
        &self.0
    }
}

impl <Tz: TimeZone> From<Timestamp<Tz>> for DateTime<Tz> {
    fn from(val: Timestamp<Tz>) -> Self {
        val.0
    }
}

impl <Tz: TimeZone> From<DateTime<Tz>> for Timestamp<Tz> {
    fn from(dur: DateTime<Tz>) -> Timestamp<Tz> {
        Timestamp(dur)
    }
}

impl FromStr for Timestamp<Utc> {
    type Err = date::Error;
    fn from_str(s: &str) -> Result<Timestamp<Utc>, Self::Err> {
        parse_rfc3339_weak(s).map(Timestamp)
    }
}

impl <Tz: TimeZone> fmt::Display for Timestamp<Tz> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_rfc3339(self.0.clone()).fmt(f)
    }
}
