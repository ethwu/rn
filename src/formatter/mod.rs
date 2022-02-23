mod segment;
mod unit;

use std::iter::FromIterator;

use num::rational::Ratio;
pub use segment::Segment;
pub use unit::TimeUnit;

/// A system of units for formatting time expressions.
#[derive(Debug, Clone)]
pub struct TimeFormatter<'f> {
    /// The proportion of units to milliseconds, in units/ms. Represented as a
    /// tuple of the numerator and the denominator. For example, the Misalian
    /// Seximal Units use the snap as their basic unit. There are 279,936 snaps
    /// per every 86,400,000 milliseconds (279,936 snaps per day), so the `base`
    /// would be `(279_936, 86_400_000)`.
    base: Ratio<u64>,
    /// The segments to render, in the order that they are displayed.
    segments: Vec<Segment<'f>>,
}

impl<'f> TimeFormatter<'f> {
    /// Construct a new `TimeFormatter` with the passed specification.
    pub fn new<R, I>(base: R, spec: I) -> Self
    where
        R: Into<Ratio<u64>>,
        I: IntoIterator<Item = Segment<'f>>,
    {
        Self {
            base: base.into(),
            segments: Vec::from_iter(spec),
        }
    }

    pub fn render(&self, ms: u32) -> String {
        // assume that usually the string will have something like two digits
        // and a separator per section (e.g. "02:08:33.4" has three segments
        // with three characters each and one segment with one character).
        let mut out = String::with_capacity(self.segments.len() * 3);
        // the amount of time to be formatted, adjusted to be in base units
        let total = self.base * ms as u64;
        for segment in &self.segments {
            out += &segment.render(total);
        }
        out
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use assert2::check;

    #[test]
    fn construct_hms_ms() {
        // h:m:s.ms
        let si_time_units = TimeFormatter::new(
            (1, 1), // 1ms = 1ms
            [
                Segment::Value((10, "hour", 3_600_000, 24).into()),
                Segment::Literal(":"),
                Segment::Value((10, "minute", 60_000, 60).into()),
                Segment::Literal(":"),
                Segment::Value((10, "second", 1_000, 60).into()),
                Segment::Literal("."),
                Segment::Value((10, "millisecond", 1, 1_000, 0).into()),
            ],
        );

        check!(si_time_units.render(0) == "00:00:00.0");
        check!(si_time_units.render(7_679_092) == "02:07:59.092");
        check!(si_time_units.render(49_029_000) == "13:37:09.0");
    }
}
