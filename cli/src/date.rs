use anyhow::{Result, bail};
use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Utc};

/// Puzzles unlock at midnight US Eastern. Advent runs entirely within EST, so
/// a fixed offset is exact and DST never applies.
const AOC_UTC_OFFSET_HOURS: i32 = -5;

const FIRST_YEAR: u16 = 2015;
const LAST_DAY: u8 = 25;

fn aoc_timezone() -> FixedOffset {
    FixedOffset::east_opt(AOC_UTC_OFFSET_HOURS * 3600).expect("offset is in range")
}

/// The current time on AoC's clock. Defaults are based on this rather than
/// local time so that they roll over the moment a new puzzle unlocks.
pub fn now() -> DateTime<FixedOffset> {
    Utc::now().with_timezone(&aoc_timezone())
}

/// Fills in a missing year or day from `now`, then validates the result.
pub fn resolve(
    year: Option<u16>,
    day: Option<u8>,
    now: DateTime<FixedOffset>,
) -> Result<(u16, u8)> {
    let year = year.unwrap_or(now.year() as u16);
    let day = day.unwrap_or(now.day() as u8);

    if year < FIRST_YEAR {
        bail!("year {year} is before the first Advent of Code ({FIRST_YEAR})");
    }
    if !(1..=LAST_DAY).contains(&day) {
        bail!("day {day} is outside 1-{LAST_DAY}; pass one with -d");
    }

    Ok((year, day))
}

pub fn is_unlocked(year: u16, day: u8, now: DateTime<FixedOffset>) -> bool {
    let unlock = aoc_timezone()
        .with_ymd_and_hms(year.into(), 12, day.into(), 0, 0, 0)
        .single()
        .expect("every day of December exists");

    now >= unlock
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(timestamp: &str) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(timestamp)
            .unwrap()
            .with_timezone(&aoc_timezone())
    }

    #[test]
    fn defaults_follow_the_aoc_clock_not_local_time() {
        // 22:30 on Nov 30 in UTC-7 is already Dec 1 for AoC.
        let now = at("2026-11-30T22:30:00-07:00");
        assert_eq!(resolve(None, None, now).unwrap(), (2026, 1));
    }

    #[test]
    fn explicit_values_override_defaults() {
        let now = at("2026-12-05T12:00:00-05:00");
        assert_eq!(resolve(Some(2024), Some(9), now).unwrap(), (2024, 9));
        assert_eq!(resolve(None, Some(9), now).unwrap(), (2026, 9));
        assert_eq!(resolve(Some(2024), None, now).unwrap(), (2024, 5));
    }

    #[test]
    fn rejects_days_and_years_aoc_never_had() {
        let now = at("2026-12-05T12:00:00-05:00");
        assert!(resolve(None, Some(0), now).is_err());
        assert!(resolve(None, Some(26), now).is_err());
        assert!(resolve(Some(2014), None, now).is_err());
        // A default can land out of range too, e.g. on the 31st.
        assert!(resolve(None, None, at("2026-12-31T12:00:00-05:00")).is_err());
    }

    #[test]
    fn unlocks_exactly_at_midnight_eastern() {
        assert!(!is_unlocked(2026, 1, at("2026-11-30T23:59:59-05:00")));
        assert!(is_unlocked(2026, 1, at("2026-12-01T00:00:00-05:00")));
        assert!(is_unlocked(2026, 1, at("2026-11-30T22:00:00-07:00")));
    }

    #[test]
    fn past_years_stay_unlocked() {
        assert!(is_unlocked(2015, 25, at("2026-01-01T00:00:00-05:00")));
    }
}
