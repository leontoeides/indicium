/// The `Metric` trait allows `indicium` to treat the various distance/string
/// similarity metrics in Ilia Schelokov's
/// [eddie](https://crates.io/crates/eddie) crate generically.
///
/// This trait provides a small, generic subset of the features provided in
/// `eddie`.
pub trait Metric {
    /// Creates a new instance of a metric `struct` with an internal state for
    /// the metric methods to reuse.
    #[must_use]
    fn new() -> Self;

    /// Similarity metric. Inversion of relative distance, ranging from 1.0
    /// (equality) to 0.0 (nothing in common).
    #[must_use]
    fn similarity(&self, str1: &str, str2: &str) -> f64;
} // trait Metric