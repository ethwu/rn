use std::fmt;

/// Default padding width.
const DEFAULT_WIDTH: usize = 2;

/// A time unit to display. It only makes sense when taken in conjunction with
/// a reference unit, such as the attribute `prototype` on [`TimeFormatter`].
#[derive(Debug, Default, Clone)]
pub struct TimeUnit<'u> {
    /// The preferred radix of this unit's unit system.
    radix: u8,
    /// The name of this time unit.
    _name: &'u str,
    /// The value of this time unit as a multiple of the reference unit.
    pub(super) value: u64,
    /// The masximum number of these time units permitted.
    pub(super) limit: u64,
    /// How wide to pad this unit.
    width: usize,
}

impl<'u> TimeUnit<'u> {
    pub fn new(name: &'u str, value: u64, limit: u64, width: usize) -> Self {
        Self::with_radix(10, name, value, limit, width)
    }

    pub fn with_radix(radix: u8, name: &'u str, value: u64, limit: u64, width: usize) -> Self {
        Self {
            radix,
            _name: name,
            value,
            limit,
            width,
        }
    }

    /// Render the passed value to a string using this unit.
    pub fn render(&self, value: u64) -> String {
        // TODO: make padding width and character configurable
        format!(
            "{:0width$}",
            ValueDisplay(self.radix, value),
            width = self.width
        )
    }

    /// Render the passed value to a formatter using this unit.
    pub fn render_fmt(&self, f: &mut fmt::Formatter, value: u64) -> fmt::Result {
        // TODO: make padding width and character configurable
        write!(
            f,
            "{:0width$}",
            ValueDisplay(self.radix, value),
            width = self.width
        )
    }
}

/// A hack for padding the radix-converted number correctly.
/// ```rust
/// let radix = 8;
/// let value = 39;
/// assert_eq!(format!("{0:3}", ValueDisplay(radix, value)), "047");
/// ```
struct ValueDisplay(u8, u64);

impl fmt::Display for ValueDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = radix_fmt::radix(self.1, self.0).to_string();
        f.pad_integral(true, "", &r)
    }
}

impl<'u> From<(&'u str, u64, u64)> for TimeUnit<'u> {
    fn from((name, value, limit): (&'u str, u64, u64)) -> Self {
        Self::new(name, value, limit, DEFAULT_WIDTH)
    }
}

impl<'u> From<(u8, &'u str, u64, u64)> for TimeUnit<'u> {
    fn from((radix, name, value, limit): (u8, &'u str, u64, u64)) -> Self {
        Self::with_radix(radix, name, value, limit, DEFAULT_WIDTH)
    }
}

impl<'u> From<(&'u str, u64, u64, usize)> for TimeUnit<'u> {
    fn from((name, value, limit, width): (&'u str, u64, u64, usize)) -> Self {
        Self::new(name, value, limit, width)
    }
}

impl<'u> From<(u8, &'u str, u64, u64, usize)> for TimeUnit<'u> {
    fn from((radix, name, value, limit, width): (u8, &'u str, u64, u64, usize)) -> Self {
        Self::with_radix(radix, name, value, limit, width)
    }
}
