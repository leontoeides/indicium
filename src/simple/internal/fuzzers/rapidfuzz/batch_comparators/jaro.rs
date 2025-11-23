//! Jaro similarity

#![allow(clippy::inline_always)]

use rapidfuzz::distance::jaro::{Args, BatchComparator};

/// This `struct` is used to access the Jaro algorithm, as implemented by the
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
///
/// The Jaro similarity is a measure of similarity between two strings, often
/// used in the field of record linkage and string matching. It’s particularly
/// effective in comparing short strings, such as names. The algorithm considers
/// both the common characters and their order in the strings, as well as the
/// number of transpositions needed to make the strings equal.
pub struct Jaro(BatchComparator<char>);

/// This `BatchComparator` trait implementation is used to access the Jaro 
/// algorithm, as implemented by the 
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
impl crate::simple::internal::fuzzers::rapidfuzz::BatchComparator for Jaro {
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