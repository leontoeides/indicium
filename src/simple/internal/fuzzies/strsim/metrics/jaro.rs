//! Jaro similarity

/// This `struct` is used to access the Jaro similarity, as implemented by Danny
/// Guo's [strsim](https://crates.io/crates/strsim) crate, in a generic manner.
pub struct Jaro;

/// This `Metric` trait implementation is used to access the Jaro similarity, as
/// implemented by Danny Guo's [strsim](https://crates.io/crates/strsim) crate,
/// in a generic manner.
impl crate::simple::internal::fuzzies::strsim::Metric for Jaro {
    /// Similarity metric. Inversion of relative distance, ranging from 1.0
    /// (equality) to 0.0 (nothing in common).
    fn similarity(str1: &str, str2: &str) -> f64 {
        strsim::jaro(str1, str2)
    } // fn
} // impl