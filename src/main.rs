#![feature(fn_traits)]
#![feature(trait_alias)]

use std::{error::Error, time::Duration};

use chrono::{DateTime, Local, NaiveTime, ParseResult, Utc};
use clap::Parser;

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

/// Return a time formatter for Misalian–Kunimunean spans.
pub fn mk_span_time_formatter() -> TimeFormatter<'static> {
    TimeFormatter::new(
        (36 * 36 * 36 * 6, 86_400_000),
        [Segment::Value((6, "span", 1296, 1296, 3).into())],
    )
}

/// Return a time formatter for Misalian–Kunimunean snaps.
pub fn mk_snap_time_formatter() -> TimeFormatter<'static> {
    TimeFormatter::new(
        (36 * 36 * 36 * 6, 86_400_000),
        [Segment::Value((6, "snap", 1, 36 * 36 * 36 * 6, 7).into())],
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
    const FORMATS: [&str; 12] = [
        "%T",          // 00:34:60
        "%R",          // 00:35
        "%r",          // 12:34:60 AM
        "%I:%M %p",    // 12:35 AM
        "%Hh %Mm %Ss", // 12h 34m 60s
        "%Hh %Mm",     // 12h 35m
        "%Hh",         // 12h
        "%I%M %p",     // 1235am
        "%I %p",       // 12am
        "%H%M",        // 1235
        "%+",          // 2001-07-08T00:34:60.026490+09:30
        "%c",          // Sun Jul 8 00:34:60 2001
    ];

    let mut t = None;
    for fmt in FORMATS {
        match NaiveTime::parse_from_str(when, fmt) {
            Ok(t) => return Ok(t),
            Err(err) => t = Some(err),
        }
    }

    // because the length of the loop above is guaranteed to be greater than
    // zero, this is perfectly safe.
    Err(t.unwrap())
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// What time to display. Defaults to the current time.
    ///
    /// Several input formats are supported, including ISO-8601 extended date/time
    /// format and `ctime` format. In these formats, the date is ignored. AM and
    /// PM may be upper- or lowercased. Examples of supported times include `00:34:60`, `12:34:60 AM`, `4pm`, `6h 45m`, and `8h24m36s`.
    when: Option<String>,
    /// Display the current snap.
    ///
    /// Outputs the number of spans that have elapsed since midnight. Because of
    /// the way that lapses, lulls, moments, and snaps are specified, this is
    /// the same as the default extended form, but without delimiters; e.g.,
    /// extended form `20:34:05.0` is equivalent to basic form `2034050`. Zero
    /// padded to fill seven digits. Ranges from `0000000` to `5555555`.
    #[clap(short, long)]
    basic: bool,
    /// Use system time zone instead of UTC.
    #[clap(short, long)]
    local: bool,
    /// Alias of `--basic`.
    #[clap(long)]
    snap: bool,
    /// Display the current span.
    ///
    /// Outputs the number of spans that have elapsed since midnight.
    /// Zero-padded to fill three digits. Ranges from `000` to `555`.
    #[clap(short, long)]
    span: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let millis = if let Some(when) = args.when {
        attempt_parse_time_since_midnight(&when)?
            .signed_duration_since(NaiveTime::from_hms(0, 0, 0))
            .to_std()
            .unwrap()
    } else if args.local {
        time_since_local_midnight()
    } else {
        time_since_utc_midnight()
    }
    .as_millis() as u32;

    let formatter = if args.span {
        mk_span_time_formatter()
    } else if args.basic || args.snap {
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

    use assert2::check;

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
        check!(senary_time_a(0) == senary_time_b(0));
        check!(senary_time_a(47521888) == senary_time_b(47521888));
        check!(senary_time_a(130967197) == senary_time_b(130967197));

        check!(senary_time_a(0) == "0:0:0.0");
        check!(senary_time_a(47521888) == "31:44:45.4");
        check!(senary_time_a(130967197) == "130:32:30.1");

        let millis = time_since_utc_midnight().as_millis();
        check!(senary_time_a(millis) == senary_time_b(millis));
    }

    #[test]
    fn senary_formatter() {
        let mkt = misalian_kunimunean_time_formatter();

        check!(mkt.render(0) == "00:00:00.0");
        check!(mkt.render(47521888) == "31:44:45.4");
        check!(mkt.render(81218884) == "53:50:14.1");
        check!(mkt.render(81246133) == "53:50:40.0");
        check!(mkt.render(130967197) == "130:32:30.1");
    }

    #[test]
    fn basic_formatter() {
        let basic = mk_snap_time_formatter();

        check!(basic.render(0) == "0000000");
        check!(basic.render(47521888) == "3144454");
        check!(basic.render(81218884) == "5350141");
        check!(basic.render(81246133) == "5350400");
        check!(basic.render(130967197) == "13032301");
    }
}
