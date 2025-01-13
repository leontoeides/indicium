//! `Metric` implementations that allow `indicium` to treat the various
//! distance/string similarity metrics in Ilia Schelokov's
//! [eddie](https://crates.io/crates/eddie) crate generically.

pub mod levenshtein;
pub mod damerau_levenshtein;
pub mod hamming;
pub mod jaro;
pub mod jaro_winkler;