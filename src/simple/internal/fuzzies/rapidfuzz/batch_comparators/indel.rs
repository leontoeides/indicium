//! Indel distance

#![allow(clippy::inline_always)]

use rapidfuzz::distance::indel::{Args, BatchComparator};

/// This `struct` is used to access the Indel algorithm, as implemented by the
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
pub struct Indel(BatchComparator<char>);

/// This `BatchComparator` trait implementation is used to access the Indel 
/// algorithm, as implemented by the 
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
impl crate::simple::internal::fuzzies::rapidfuzz::BatchComparator for Indel {
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
            &Args::default().score_cutoff(score_cutoff)
        )
    } // fn
} // impl