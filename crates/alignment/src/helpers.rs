use chrono::Duration;
use pyo3::prelude::*;

pub fn timedelta_to_duration(timedelta: &Bound<'_, PyAny>) -> PyResult<Duration> {
    let days: i64 = timedelta.getattr("days")?.extract()?;
    let seconds: i64 = timedelta.getattr("seconds")?.extract()?;
    let microseconds: i64 = timedelta.getattr("microseconds")?.extract()?;

    Ok(chrono::Duration::days(days)
        + chrono::Duration::seconds(seconds)
        + chrono::Duration::microseconds(microseconds))
}
