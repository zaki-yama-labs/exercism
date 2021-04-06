use std::ops::Add;

use chrono::{DateTime, Duration, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let base: i64 = 10;
    start.add(Duration::seconds(base.pow(9)))
}
