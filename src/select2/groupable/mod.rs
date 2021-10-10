//! Support for the `Select2` **grouped** data format, which renders categorizes
//! your data into HTML
//! [\<optgroup\>](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/optgroup)
//! sections.

pub mod group;
pub mod groupable_record;
pub mod grouped_results;
pub mod results;

// -----------------------------------------------------------------------------

pub use crate::select2::groupable::group::Group;
pub use crate::select2::groupable::groupable_record::GroupableRecord;
pub use crate::select2::groupable::grouped_results::GroupedResults;

// -----------------------------------------------------------------------------
//
/// To make a struct Select2-ready with support for grouped data, the programmer
/// must implement the `Groupable` trait for it. The trait returns a
/// `GroupableRecord` with all content needed to make it usable with the
/// `select2.org` Javascript plugin.
///
/// When options are to be generated in `<optgroup>` sections, options should be
/// nested under the `children` key of each group object. The label for the
/// group should be specified as the `text` property on the group's
/// corresponding data object.

pub trait Groupable {
    fn record(&self) -> GroupableRecord;
} // Groupable