//! Hamming distance

use rapidfuzz::distance::hamming::{Args, BatchComparator};

/// This `struct` is used to access the Hamming algorithm, as implemented by the
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
pub struct Hamming(BatchComparator<char>);

/// This `BatchComparator` trait implementation is used to access the Hamming
/// algorithm, as implemented by the 
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
impl crate::simple::internal::rapidfuzz::BatchComparator for Hamming {
    /// Intantiates a new batch comparator.
    fn new(one: &str) -> Self {
        Self(BatchComparator::new(one.chars()))
    } // fn

    /// Normalized similarity calculated.
    fn normalized_similarity(
        &self,
        many: &str,
        score_cutoff: f64
    ) -> Option<f64> {
        self.0.normalized_similarity_with_args(
            many.chars(),
            &Args::default().pad(true).score_cutoff(score_cutoff)
        )
    } // fn
} // impl