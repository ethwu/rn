#![feature(fn_traits)]
#![feature(trait_alias)]

use std::{error::Error, time::Duration};

use chrono::{DateTime, Local, NaiveTime, ParseResult, Utc};
use clap::{App, Arg};

mod formatter;

use crate::formatter::{Segment, TimeFormatter};

/// Return a time formatter for Misalian–Kunimunean Seximal Units.
pub fn misalian_kunimunean_time_formatter() -> TimeFormatter<'static> {
    TimeFormatter::new(
        (36 * 36 * 36 * 6, 86_400_000),
        [
            Segment::Value((6, "lapse", 7776, 36).into()),
            Segment::Literal(":"),
            Segment::Value((6, "lull", 216, 36).into()),
            Segment::Literal(":"),
            Segment::Value((6, "moment", 6, 36).into()),
            Segment::Literal("."),
            Segment::Value((6, "snap", 1, 6, 0).into()),
        ],
    )
}

/// Return a time formatter for Misalian–Kunimunean snaps.
pub fn mk_snap_time_formatter() -> TimeFormatter<'static> {
    TimeFormatter::new(
        (36 * 36 * 36 * 6, 86_400_000),
        [Segment::Value((6, "span", 1296, 1296, 3).into())],
    )
}

/// Get the duration that has elapsed since midnight today.
fn time_since_local_midnight() -> Duration {
    let now: DateTime<Local> = Local::now();
    let midnight: DateTime<Local> = Local::today().and_hms(0, 0, 0);

    now.signed_duration_since(midnight).to_std().unwrap()
}

/// Get the duration that has elapsed since midnight today.
fn time_since_utc_midnight() -> Duration {
    let now: DateTime<Utc> = Utc::now();
    let midnight: DateTime<Utc> = Utc::today().and_hms(0, 0, 0);

    now.signed_duration_since(midnight).to_std().unwrap()
}

/// Parse a user-provided time. Attempts various formats before giving up and
/// erroring out.
fn attempt_parse_time_since_midnight(when: &str) -> ParseResult<NaiveTime> {
    // Formats to try before giving up.
    const FORMATS: [&str; 6] = [
        "%T",          // 00:34:60
        "%R",          // 00:34
        "%r",          // 12:34:60 AM
        "%I:%M %p",    // 12:34 AM
        "%Hh %Mm %Ss", // 12h 34m 60s
        "%Hh %Mm",     // 12h 34m
    ];

    let mut t = None;
    for fmt in FORMATS {
        t = Some(NaiveTime::parse_from_str(when, fmt));
        if let Some(t) = t {
            if t.is_ok() {
                break;
            }
        }
    }

    // because the length of the loop above is guaranteed to be greater than
    // zero, this is perfectly safe.
    t.unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let authors = env!("CARGO_PKG_AUTHORS").replace(':', ", ");
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(&*authors)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("when").help("A time to display instead of the current time."))
        .arg(
            Arg::with_name("snap")
                .short("s")
                .long("snap")
                .help("Instead of returning the full time, return the current snap."),
        )
        .arg(
            Arg::with_name("local")
                .short("l")
                .long("local")
                .help("Use system time zone instead of UTC."),
        );
    let matches = app.get_matches();

    let millis = if let Some(when) = matches.value_of("when") {
        attempt_parse_time_since_midnight(when)?
            .signed_duration_since(NaiveTime::from_hms(0, 0, 0))
            .to_std()
            .unwrap()
    } else if matches.is_present("local") {
        time_since_local_midnight()
    } else {
        time_since_utc_midnight()
    }
    .as_millis() as u32;

    let formatter = if matches.is_present("snap") {
        mk_snap_time_formatter()
    } else {
        misalian_kunimunean_time_formatter()
    };
    println!("{}", formatter.render(millis));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    /// Format the given time in senary.
    fn senary_time_a(millis: u128) -> String {
        // the total number of snaps in the day
        let mut remaining = millis * 36 * 36 * 36 * 6 / 86_400_000;
        let snaps = (remaining % 6) as u8;
        remaining /= 6;
        let moments = (remaining % 36) as u8;
        remaining /= 36;
        let lulls = (remaining % 36) as u8;
        let lapses = (remaining / 36) as u8;

        format!(
            "{}:{}:{}.{}",
            radix_fmt::radix(lapses, 6),
            radix_fmt::radix(lulls, 6),
            radix_fmt::radix(moments, 6),
            radix_fmt::radix(snaps, 6),
        )
    }

    /// An alternative approach to formatting the senary time.
    fn senary_time_b(millis: u128) -> String {
        let mut out: Vec<String> = Vec::with_capacity(4);
        let mut remaining: u64 = millis as u64 * 36 * 36 * 36 * 6 / 86_400_000;
        out.push(format!(".{}", radix_fmt::radix_6(remaining % 6)));
        remaining /= 6;
        out.push(format!("{}", radix_fmt::radix_6(remaining % 36)));
        remaining /= 36;
        out.push(format!("{}:", radix_fmt::radix_6(remaining % 36)));

        out.push(format!("{}:", radix_fmt::radix_6(remaining / 36)));
        out.into_iter().rev().collect::<String>()
    }

    #[test]
    fn senary_representations_sanity_check() {
        assert_eq!(senary_time_a(0), senary_time_b(0));
        assert_eq!(senary_time_a(47521888), senary_time_b(47521888));
        assert_eq!(senary_time_a(130967197), senary_time_b(130967197));

        assert_eq!(senary_time_a(0), "0:0:0.0");
        assert_eq!(senary_time_a(47521888), "31:44:45.4");
        assert_eq!(senary_time_a(130967197), "130:32:30.1");

        let millis = time_since_utc_midnight().as_millis();
        assert_eq!(senary_time_a(millis), senary_time_b(millis));
    }

    #[test]
    fn senary_formatter() {
        let mkt = misalian_kunimunean_time_formatter();

        assert_eq!(mkt.render(0), "00:00:00.0");
        assert_eq!(mkt.render(47521888), "31:44:45.4");
        assert_eq!(mkt.render(81218884), "53:50:14.1");
        assert_eq!(mkt.render(81246133), "53:50:40.0");
        assert_eq!(mkt.render(130967197), "130:32:30.1");
    }
}
