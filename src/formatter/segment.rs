use std::fmt;

use super::TimeUnit;

/// A segment to render.
#[derive(Debug, Clone)]
pub enum Segment<'s> {
    /// A literal string.
    Literal(&'s str),
    /// A dynamic segment formatted using a TimeUnit.
    Value(TimeUnit<'s>),
}

impl Segment<'_> {
    /// Render this segment with the given number of ms since the start of the
    /// day.
    pub fn render(&self, total: u64) -> String {
        match self {
            Self::Literal(s) => s.to_string(),
            Self::Value(u) => u.render(total / u.value % u.limit),
        }
    }

    /// Render this segment with the given number of ms since the start of the
    /// day.
    pub fn render_fmt(&self, f: &mut fmt::Formatter, total: u64) -> fmt::Result {
        match self {
            Self::Literal(s) => write!(f, "{}", s),
            Self::Value(u) => u.render_fmt(f, total / u.value % u.limit),
        }
    }
}

impl<'s> From<&'s str> for Segment<'s> {
    fn from(s: &'s str) -> Self {
        Self::Literal(s)
    }
}

impl<'s> From<TimeUnit<'s>> for Segment<'s> {
    fn from(u: TimeUnit<'s>) -> Self {
        Self::Value(u)
    }
}
