// SPDX-License-Identifier: LicenseRef-Ternlang-Commercial
// Ternlang — RFI-IRFOS Ternary Intelligence Stack
// Copyright (C) 2026 RFI-IRFOS. All rights reserved.

//! Temporal cognition: the unix-epoch anchor and human-legible time for memory.
//!
//! Episodic memory is meaningless without a real clock. Every episode is anchored
//! to a unix-epoch millisecond timestamp and rendered in ISO 8601, matching the
//! RFI-IRFOS `iso` tool's canonical stamp: `Weekday/ISO8601Z/epoch_seconds`
//! (e.g. `Thu/2026-06-25T05:43:11Z/1782366191`). Pure Rust — no external process,
//! no `chrono` — so it works anywhere the crate does.

use std::time::{SystemTime, UNIX_EPOCH};

const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

/// Real wall-clock now, unix milliseconds. The pull the MCP layer uses to stamp
/// memories; the store core stays deterministic by taking time as a parameter.
pub fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// Howard Hinnant's civil-from-days: days since 1970-01-01 → (year, month, day).
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32; // [1, 12]
    (y + if m <= 2 { 1 } else { 0 }, m, d)
}

/// Inverse: (year, month, day) → days since 1970-01-01.
fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = y - if m <= 2 { 1 } else { 0 };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if m > 2 { m - 3 } else { m + 9 } as i64;
    let doy = (153 * mp + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

fn split_ms(ms: i64) -> (i64, u32, u32, u32, i64) {
    let secs = ms.div_euclid(1000);
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    (days, (rem / 3600) as u32, (rem % 3600 / 60) as u32, (rem % 60) as u32, secs)
}

/// Weekday abbreviation for a timestamp (e.g. `"Thu"`).
pub fn weekday(ms: i64) -> &'static str {
    let days = ms.div_euclid(1000).div_euclid(86_400);
    WEEKDAYS[(days.rem_euclid(7) as usize + 4) % 7]
}

/// ISO 8601 UTC, second precision: `2026-06-25T05:43:11Z`.
pub fn unix_ms_to_iso(ms: i64) -> String {
    let (days, hh, mm, ss, _) = split_ms(ms);
    let (y, mo, d) = civil_from_days(days);
    format!("{y:04}-{mo:02}-{d:02}T{hh:02}:{mm:02}:{ss:02}Z")
}

/// Canonical RFI-IRFOS stamp: `Weekday/ISO8601Z/epoch_seconds`
/// — identical to the `iso` tool, so memory timestamps match the rest of the stack.
pub fn unix_ms_to_stamp(ms: i64) -> String {
    let (_, _, _, _, secs) = split_ms(ms);
    format!("{}/{}/{}", weekday(ms), unix_ms_to_iso(ms), secs)
}

/// Parse a timestamp back to unix milliseconds. Accepts, liberally:
/// - a full stamp `Thu/2026-06-25T05:43:11Z/1782366191` (epoch field wins),
/// - ISO datetime `2026-06-25T05:43:11Z` / `2026-06-25 05:43:11`,
/// - a bare date `2026-06-25` (→ midnight UTC),
/// - bare digits (epoch seconds, or milliseconds if 13+ digits).
pub fn iso_to_unix_ms(s: &str) -> Option<i64> {
    let s = s.trim();
    // Full stamp: trust the trailing epoch-seconds field.
    if let Some((_, rest)) = s.split_once('/') {
        if let Some((_, epoch)) = rest.rsplit_once('/') {
            if let Ok(secs) = epoch.trim().parse::<i64>() {
                return Some(secs * 1000);
            }
        }
    }
    // Bare digits → epoch.
    if s.chars().all(|c| c.is_ascii_digit() || c == '-') && s.contains(|c: char| c.is_ascii_digit()) && !s.contains('T') && s.matches('-').count() == 0 {
        if let Ok(n) = s.parse::<i64>() {
            return Some(if s.len() >= 13 { n } else { n * 1000 });
        }
    }
    // ISO date / datetime.
    let (date, time) = match s.split_once(['T', ' ']) {
        Some((d, t)) => (d, t.trim_end_matches('Z')),
        None => (s, ""),
    };
    let mut dp = date.split('-');
    let y: i64 = dp.next()?.parse().ok()?;
    let mo: u32 = dp.next()?.parse().ok()?;
    let d: u32 = dp.next()?.parse().ok()?;
    let (mut hh, mut mm, mut ss) = (0i64, 0i64, 0i64);
    if !time.is_empty() {
        let mut tp = time.split(':');
        hh = tp.next().and_then(|x| x.parse().ok()).unwrap_or(0);
        mm = tp.next().and_then(|x| x.parse().ok()).unwrap_or(0);
        ss = tp.next().and_then(|x| x.split('.').next()?.parse().ok()).unwrap_or(0);
    }
    let days = days_from_civil(y, mo, d);
    Some((days * 86_400 + hh * 3600 + mm * 60 + ss) * 1000)
}

/// Human-legible elapsed time, e.g. `"3d 4h ago"`, `"12m ago"`, `"just now"`,
/// `"in 2h"`. Gives an agent relative temporal cognition over a recall.
pub fn humanize_age_ms(delta_ms: i64) -> String {
    let future = delta_ms < 0;
    let mut s = delta_ms.unsigned_abs() / 1000;
    if s < 45 {
        return "just now".into();
    }
    let days = s / 86_400;
    s %= 86_400;
    let hours = s / 3600;
    s %= 3600;
    let mins = s / 60;
    let body = if days > 0 {
        format!("{days}d {hours}h")
    } else if hours > 0 {
        format!("{hours}h {mins}m")
    } else {
        format!("{mins}m")
    };
    if future {
        format!("in {body}")
    } else {
        format!("{body} ago")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_iso_tool_reference() {
        // Reference from the `iso` binary: Thu/2026-06-25T05:43:11Z/1782366191
        let ms = 1_782_366_191_000;
        assert_eq!(unix_ms_to_iso(ms), "2026-06-25T05:43:11Z");
        assert_eq!(weekday(ms), "Thu");
        assert_eq!(unix_ms_to_stamp(ms), "Thu/2026-06-25T05:43:11Z/1782366191");
    }

    #[test]
    fn roundtrip_iso() {
        for &ms in &[0i64, 1_782_366_191_000, 1_750_000_000_000, 4_102_444_800_000] {
            let iso = unix_ms_to_iso(ms);
            let back = iso_to_unix_ms(&iso).unwrap();
            assert_eq!(back, (ms / 1000) * 1000, "roundtrip {iso}");
        }
    }

    #[test]
    fn parse_forms() {
        assert_eq!(iso_to_unix_ms("1970-01-01"), Some(0));
        assert_eq!(iso_to_unix_ms("2026-06-25T05:43:11Z"), Some(1_782_366_191_000));
        assert_eq!(iso_to_unix_ms("Thu/2026-06-25T05:43:11Z/1782366191"), Some(1_782_366_191_000));
        assert_eq!(iso_to_unix_ms("1782366191"), Some(1_782_366_191_000));
    }

    #[test]
    fn humanize() {
        assert_eq!(humanize_age_ms(10_000), "just now");
        assert_eq!(humanize_age_ms(3 * 86_400_000 + 4 * 3_600_000), "3d 4h ago");
        assert_eq!(humanize_age_ms(-2 * 3_600_000), "in 2h 0m");
    }
}
