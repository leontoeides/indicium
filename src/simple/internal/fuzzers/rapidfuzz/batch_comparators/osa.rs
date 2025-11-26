//! Optimal String Alignment distance

#![allow(clippy::inline_always)]

use rapidfuzz::distance::osa::{Args, BatchComparator};

/// This `struct` is used to access the Optimal String Alignment algorithm, as 
/// implemented by the [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in 
/// a generic manner.
///
/// The Optimal String Alignment distance (OSA) measures the minimum number of
/// operations required to transform one string into another, considering four
/// types of elementary edits: `insertions`, `deletions`, `substitutions`, and
/// `transpositions`.
///
/// While both the `Damerau-Levenshtein` and OSA distance include
/// transpositions, they differ in the treatment of transpositions. OSA treats
/// any transposition as a single operation, regardless of whether the
/// transposed characters are adjacent or not. In contrast, the
/// Damerau-Levenshtein distance specifically allows transpositions of adjacent
/// characters.
///
/// The handling of transpositions in the OSA distance is simpler, which makes
/// it computationally less intensive.
pub struct Osa(BatchComparator<char>);

/// This `BatchComparator` trait implementation is used to access the 
/// Optimal String Alignment algorithm, as implemented by the
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
impl crate::simple::internal::fuzzers::rapidfuzz::BatchComparator for Osa {
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