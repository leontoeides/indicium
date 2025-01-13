//! Integration with Ilia Schelokov's [eddie](https://crates.io/crates/eddie)
//! string similarity crate.

// -----------------------------------------------------------------------------
//
// Generic `Fuzzy` trait interface to the `eddie` crate.

// pub mod fuzzy;

// -----------------------------------------------------------------------------
//
// Interfaces for the `eddie` crate integration.

pub mod autocomplete_global;
pub mod autocomplete_global_metric;

pub mod autocomplete_context;
pub mod autocomplete_context_metric;

pub mod keyword_global;
pub mod keyword_global_metric;

// -----------------------------------------------------------------------------
//
// The `Metric` trait allows `indicium` to treat the various distance/string
// similarity metrics in Ilia Schelokov's
// [eddie](https://crates.io/crates/eddie) crate generically.

mod metric;

use crate::simple::internal::fuzzies::eddie::metric::Metric;

// -----------------------------------------------------------------------------
//
// The `Metric` implementations for the various distance/string similarity
// metrics in the `eddie` crate.

mod metrics;