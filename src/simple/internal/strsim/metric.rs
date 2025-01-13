/// The `Metric` trait allows `indicium` to treat the various Danny Guo's
/// [strsim](https://crates.io/crates/strsim) string similarity metrics
/// generically.
///
/// This trait provides a small, generic subset of the features provided in
/// `strsim`.
pub trait Metric {
    /// Calculates a normalized score between 0.0 and 1.0 (inclusive), where 1.0
    /// means the strings are the same.
    fn similarity(a: &str, b: &str) -> f64;
} // trait Metric