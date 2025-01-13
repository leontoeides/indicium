//! `BatchComparator` implementations that allow `indicium` to treat the various
//! distance and string similarity algorithms in the
//! [rapidfuzz](https://crates.io/crates/rapidfuzz) crate generically.

pub mod damerau_levenshtein;
pub mod hamming;
pub mod indel;
pub mod jaro;
pub mod jaro_winkler;
pub mod lcs_seq;
pub mod levenshtein;
pub mod osa;
pub mod postfix;
pub mod prefix;