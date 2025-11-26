//! Damerau-Levenshtein distance

#![allow(clippy::inline_always)]

/// This `Metric` trait implementation is used to access the Damerau-Levenshtein
/// algorithm, as implemented by Ilia Schelokov's
/// [eddie](https://crates.io/crates/eddie) crate, in a generic manner.
impl crate::simple::internal::fuzzers::eddie::Metric for eddie::str::DamerauLevenshtein {
    #[inline(always)]
    /// Creates a new instance of a metric `struct` with an internal state for
    /// the metric methods to reuse.
    fn new() -> Self {
        Self::new()
    }

    /// Similarity metric. Inversion of relative distance, ranging from 1.0
    /// (equality) to 0.0 (nothing in common).
    #[inline(always)]
    fn similarity(&self, str1: &str, str2: &str) -> f64 {
        self.similarity(str1, str2)
    } // fn
} // impl