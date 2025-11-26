//! Indel distance

#![allow(clippy::inline_always)]

use rapidfuzz::distance::indel::{Args, BatchComparator};

/// This `struct` is used to access the Indel algorithm, as implemented by the
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
///
/// The Indel distance is a specialized version of the `Levenshtein` distance
/// with only insertions and deletions. It can be calculated from the `Longest
/// Common Subsequence`.
///
/// Similar to LCS it’s commonly used in Bioinformatics applications like DNA
/// sequence analysis, where insertions and deletions play a crucial role in
/// understanding evolutionary relationships and genetic variations.
pub struct Indel(BatchComparator<char>);

/// This `BatchComparator` trait implementation is used to access the Indel 
/// algorithm, as implemented by the 
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
impl crate::simple::internal::fuzzers::rapidfuzz::BatchComparator for Indel {
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