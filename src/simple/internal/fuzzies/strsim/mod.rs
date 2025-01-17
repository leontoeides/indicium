//! Integration with Danny Guo's [strsim](https://crates.io/crates/strsim)
//! string similarity crate.

// -----------------------------------------------------------------------------
//
// Generic `Fuzzy` trait interface to the `strsim` crate.

pub mod fuzzy;

// -----------------------------------------------------------------------------
//
// Interfaces for the `strsim` crate integration.

pub mod autocomplete_global;
pub mod autocomplete_global_metric;

pub mod autocomplete_context;
pub mod autocomplete_context_metric;

pub mod keyword_global;
pub mod keyword_global_metric;

// -----------------------------------------------------------------------------
//
// The `Metric` trait allows `indicium` to treat the various string similarity
// metrics in Danny Guo's [strsim](https://crates.io/crates/strsim) crate
// generically.

mod metric;

use crate::simple::internal::fuzzies::strsim::metric::Metric;

// -----------------------------------------------------------------------------
//
// The `Metric` implementations for the various string similarity metrics in the
// `strsim` crate.

mod metrics;

use crate::simple::internal::fuzzies::strsim::metrics::jaro::Jaro;
use crate::simple::internal::fuzzies::strsim::metrics::jaro_winkler::JaroWinkler;
use crate::simple::internal::fuzzies::strsim::metrics::damerau_levenshtein::DamerauLevenshtein;
use crate::simple::internal::fuzzies::strsim::metrics::levenshtein::Levenshtein;
use crate::simple::internal::fuzzies::strsim::metrics::sorensen_dice::SorensenDice;