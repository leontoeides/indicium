//! Jaro-Winkler similarity

#![allow(clippy::inline_always)]

/// This `struct` is used to access the Jaro-Winkler similarity, as implemented
/// by Danny Guo's [strsim](https://crates.io/crates/strsim) crate, in a generic
/// manner.
pub struct JaroWinkler;

/// This `Metric` trait implementation is used to access the Jaro-Winkler
/// similarity, as implemented by Danny Guo's
/// [strsim](https://crates.io/crates/strsim) crate, in a generic manner.
impl crate::simple::internal::fuzzers::strsim::Metric for JaroWinkler {
    /// Similarity metric. Inversion of relative distance, ranging from 1.0
    /// (equality) to 0.0 (nothing in common).
    #[must_use]
    #[inline(always)]
    fn similarity(str1: &str, str2: &str) -> f64 {
        strsim::jaro_winkler(str1, str2)
    } // fn
} // impl