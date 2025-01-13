//! Jaro-Winkler similarity

use rapidfuzz::distance::jaro_winkler::{Args, BatchComparator};

/// This `struct` is used to access the Jaro-Winkler algorithm, as implemented
/// by the [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic
/// manner.
pub struct JaroWinkler(BatchComparator<char>);

/// This `BatchComparator` trait implementation is used to access the
/// Jaro-Winkler algorithm, as implemented by the
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
impl crate::simple::internal::rapidfuzz::BatchComparator for JaroWinkler {
    /// Instantiates a new batch comparator.
    fn new(one: &str) -> Self {
        Self(BatchComparator::new(one.chars()))
    } // fn

    /// Calculates normalized similarity.
    fn normalized_similarity(
        &self,
        many: &str,
        score_cutoff: f64
    ) -> Option<f64> {
        self.0.normalized_similarity_with_args(
            many.chars(),
            &Args::default().score_cutoff(score_cutoff)
        )
    } // fn
} // impl