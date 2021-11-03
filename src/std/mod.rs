// DRY implementation of many standard traits.
// https://www.greyblake.com/blog/2021-10-11-phantom-types-in-rust/
struct Sketch<T>
where
    T: Layout, {}
struct LogOptimalType;
type Histogram = Sketch<LogOptimalType>;

impl GuessLayout for LogOptimalLayout {
    const LOG_MIN_VALUE: f64 = f64::MIN.ln();
}

/// Support `histogram_1 + histogram_2`
impl std::ops::Add for Sketch<T> {
    type Output = Histogram;
    fn add(self, another: Histogram) -> Self::Output {
        let value = self.value + another.value;
        value
    }
}

/// Support `histogram_1 - histogram_2`
/// Do these have to obey algrabic rules?
impl std::ops::Sub for LogOptimalLayout {
    type Output = Histogram;
    fn sub(self, another: Histogram) -> Self::Output {
        let value = self.value + another.value;
        Histogram { value }
    }
}

impl Default for LogOptimalLayout {}

/// Support `format!("{}",histogram)`
/// see `to_string`
impl std::fmt::Display for LogOptimalLayout {}

/// Support
///
/// - `let hist_static: Histogram<Dynamic> = hist_dynamic.into()`
/// - `let hist_quadratic: Histogram<Quadratic> = hist_optimal.into()`
/// - `let hist_linear: Histogram<Linear> = hist_optimal.into()`
/// - `let hist_serialized: Histogram<Serialized> = hist_dynamic.into()`
/// - `let hist_dynamic: Histogram<Deserialized> = hist_serialized.into()`
///
impl From for LogOptimalLayout {}
// Iterator (IntoIterator)

// `equals`
impl PartialEq for LogOptimalLayout {
    fn eq(&self, other: &Self) -> bool {
        self.precision == other.precision
    }
}

// `hash_code`
impl std::hash::Hash for LogOptimalLayout {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        self.precision.hash(hasher);
    }
}

impl PartialOrd for LogOptimalLayout {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.precision.partial_cmp(&other.precision)
    }
}

impl Ord for LogOptimalLayout {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.precision.cmp(&other.precision)
    }
}
