//! Hamming distance

#![allow(clippy::inline_always)]

use rapidfuzz::distance::hamming::{Args, BatchComparator};

/// This `struct` is used to access the Hamming algorithm, as implemented by the
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
///
/// The Hamming distance measures the similarity of two sequences of equal
/// length. Specifically, it counts the minimum number of substitutions required
/// to transform one string into the other.
///
/// While regularly the Hamming distance only works with texts of equal length,
/// this implementation provides an addition argument `pad` to decide whether
/// texts of unequal length should be padded or return an error.
pub struct Hamming(BatchComparator<char>);

/// This `BatchComparator` trait implementation is used to access the Hamming
/// algorithm, as implemented by the 
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
impl crate::simple::internal::fuzzers::rapidfuzz::BatchComparator for Hamming {
    /// Instantiates a new batch comparator.
    #[must_use]
    #[inline(always)]
    fn new(one: &str) -> Self {
        Self(BatchComparator::new(one.chars()))
    } // fn

    /// Calculates normalized similarity.
    #[must_use]
    #[inline(always)]
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