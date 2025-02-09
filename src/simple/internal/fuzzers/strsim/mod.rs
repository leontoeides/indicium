//! Integration with Danny Guo's [strsim](https://crates.io/crates/strsim)
//! string similarity crate.

// -----------------------------------------------------------------------------
//
// Generic `Fuzzy` trait interface to the `strsim` crate.

pub mod fuzzy;

// -----------------------------------------------------------------------------
//
// Interfaces for the `strsim` crate integration.

pub mod keyword;
pub mod keyword_metric;

pub mod global;
pub mod global_metric;

pub mod context;
pub mod context_metric;

pub mod substitute;
pub mod substitute_metric;

// -----------------------------------------------------------------------------
//
// The `Metric` trait allows `indicium` to treat the various string similarity
// metrics in Danny Guo's [strsim](https://crates.io/crates/strsim) crate
// generically.

mod metric;

use crate::simple::internal::fuzzers::strsim::metric::Metric;

// -----------------------------------------------------------------------------
//
// The `Metric` implementations for the various string similarity metrics in the
// `strsim` crate.

mod metrics;

use crate::simple::internal::fuzzers::strsim::metrics::jaro::Jaro;
use crate::simple::internal::fuzzers::strsim::metrics::jaro_winkler::JaroWinkler;
use crate::simple::internal::fuzzers::strsim::metrics::damerau_levenshtein::DamerauLevenshtein;
use crate::simple::internal::fuzzers::strsim::metrics::levenshtein::Levenshtein;
use crate::simple::internal::fuzzers::strsim::metrics::sorensen_dice::SorensenDice;