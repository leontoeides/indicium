//! Damerau-Levenshtein distance

use rapidfuzz::distance::damerau_levenshtein::{Args, BatchComparator};

/// This `struct` is used to access the Damerau-Levenshtein algorithm, as 
/// implemented by the [rapidfuzz](https://crates.io/crates/rapidfuzz) crate,
/// in a generic manner.
pub struct DamerauLevenshtein(BatchComparator<char>);

/// This `BatchComparator` trait implementation is used to access the 
/// Damerau-Levenshtein algorithm, as implemented by the 
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
impl crate::simple::internal::rapidfuzz::BatchComparator for DamerauLevenshtein {
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
            &Args::default().score_cutoff(score_cutoff)
        )
    } // fn
} // impl