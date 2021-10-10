//! Support for the ungrouped, **flat** `Select2` output format.

pub mod flat_record;
pub mod flat_results;
pub mod results;

// -----------------------------------------------------------------------------

pub use crate::select2::selectable::flat_record::FlatRecord;
pub use crate::select2::selectable::flat_results::FlatResults;

// -----------------------------------------------------------------------------
//
/// To make a struct Select2-ready, the programmer must implement the
/// `Selectable` trait for it. The trait returns a `FlatRecord` with the content
/// needed to make it usable with the Select2 Javascript plugin.

pub trait Selectable {
    fn record(&self) -> FlatRecord;
} // Selectable