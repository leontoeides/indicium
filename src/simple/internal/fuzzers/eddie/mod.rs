//! Integration with Ilia Schelokov's [eddie](https://crates.io/crates/eddie)
//! string similarity crate.

// -----------------------------------------------------------------------------
//
// Generic `Fuzzy` trait interface to the `eddie` crate.

pub mod fuzzy;

// -----------------------------------------------------------------------------
//
// Interfaces for the `eddie` crate integration.

pub mod keyword;
pub mod keyword_metric;

pub mod global;
pub mod global_metric;

pub mod context;
pub mod context_metric;

// pub mod substitute;
// pub mod substitute_metric;

// -----------------------------------------------------------------------------
//
// The `Metric` trait allows `indicium` to treat the various distance/string
// similarity metrics in Ilia Schelokov's
// [eddie](https://crates.io/crates/eddie) crate generically.

mod metric;

use crate::simple::internal::fuzzers::eddie::metric::Metric;

// -----------------------------------------------------------------------------
//
// The `Metric` implementations for the various distance/string similarity
// metrics in the `eddie` crate.

mod metrics;