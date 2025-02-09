//! Damerau-Levenshtein distance

#![allow(clippy::inline_always)]

use rapidfuzz::distance::damerau_levenshtein::{Args, BatchComparator};

/// This `struct` is used to access the Damerau-Levenshtein algorithm, as 
/// implemented by the [rapidfuzz](https://crates.io/crates/rapidfuzz) crate,
/// in a generic manner.
///
/// The Damerau-Levenshtein distance measures the minimum number of operations
/// required to transform one string into another, considering four types of
/// elementary edits: `insertions`, `deletions`, `substitutions`, and
/// `transpositions of adjacent characters`. A transposition involves swapping
/// two adjacent characters. It does respect triangle inequality, and is thus a
/// metric distance.
///
/// It’s often used in applications where transpositions are common. An example
/// for this would be typing errors involving adjacent characters.
pub struct DamerauLevenshtein(BatchComparator<char>);

/// This `BatchComparator` trait implementation is used to access the 
/// Damerau-Levenshtein algorithm, as implemented by the 
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
impl crate::simple::internal::fuzzers::rapidfuzz::BatchComparator for DamerauLevenshtein {
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