//! Prefix similarity

#![allow(clippy::inline_always)]

use rapidfuzz::distance::prefix::{Args, BatchComparator};

/// This `struct` is used to access the Prefix algorithm, as implemented by
/// the [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic 
/// manner.
///
/// The Prefix similarity measures the length of the common prefix between two
/// sequences.
pub struct Prefix(BatchComparator<char>);

/// This `BatchComparator` trait implementation is used to access the 
/// Prefix algorithm, as implemented by the
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
impl crate::simple::internal::fuzzers::rapidfuzz::BatchComparator for Prefix {
    /// Instantiates a new batch comparator.
    #[inline(always)]
    fn new(one: &str) -> Self {
        Self(BatchComparator::new(one.chars()))
    } // fn

    /// Calculates normalized similarity.
    #[inline(always)]
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