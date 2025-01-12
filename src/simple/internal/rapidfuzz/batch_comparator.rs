/// The `BatchComparator` trait allows `indicium` to treat the various distance
/// and string similarity algorithms in the
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate generically.
///
/// This batch comparator trait provides a small, generic subset of the
/// `One × Many comparisons` provided by different algorithms in `rapidfuzz`.
pub trait BatchComparator {
    /// Intantiates a new batch comparator.
    fn new(one: &str) -> Self;

    /// Normalized similarity calculated.
    ///
    /// A `None` will be returned if the score is less than the specified
    /// cut-off.
    fn normalized_similarity(&self, many: &str, score_cutoff: f64) -> Option<f64>;
} // trait BatchComparator