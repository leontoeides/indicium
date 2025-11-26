//! Levenshtein distance

#![allow(clippy::inline_always)]

use rapidfuzz::distance::levenshtein::{Args, BatchComparator};

/// This `struct` is used to access the Levenshtein algorithm, as implemented by
/// the [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic 
/// manner.
///
/// The Levenshtein distance measures the minimum number of operations required
/// to transform one string into another, considering three types of elementary
/// edits: `insertions`, `deletions` and `substitutions`. It does respect
/// triangle inequality, and is thus a metric distance.
///
/// It finds use in various applications such as text processing, DNA sequence
/// analysis, and data cleaning.
pub struct Levenshtein(BatchComparator<char>);

/// This `BatchComparator` trait implementation is used to access the 
/// Levenshtein algorithm, as implemented by the
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
impl crate::simple::internal::fuzzers::rapidfuzz::BatchComparator for Levenshtein {
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