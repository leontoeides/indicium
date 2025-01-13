//! `Metric` implementations that allow `indicium` to treat the various
//! Danny Guo's [strsim](https://crates.io/crates/strsim) crate's string
//! similarity metrics generically.

pub mod jaro;
pub mod jaro_winkler;
pub mod damerau_levenshtein;
pub mod levenshtein;
pub mod sorensen_dice;