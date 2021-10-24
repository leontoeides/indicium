//! Support for the **grouped** `Select2` output format. Renders categorized
//! data into HTML
//! [\<optgroup\>](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/optgroup)
//! sections.

mod results;

// -----------------------------------------------------------------------------

use crate::select2::{Pagination, Record};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;
use std::string::ToString;

// -----------------------------------------------------------------------------
//
/// For the `select2::grouped` module to work with your records, you must
/// implement this `Groupable` trait for it. The trait returns
/// `GroupableRecord` which contains the information needed to make it usable
/// and groupable for the `Select2` jQuery plug-in.
///
/// When options are to be generated in `<optgroup>` sections, options should be
/// nested under the `children` key of each group object. The label for the
/// group should be specified as the `text` property on the group's
/// corresponding data object.

pub trait Groupable {
    fn record(&self) -> GroupableRecord;
} // Groupable

// -----------------------------------------------------------------------------
//
/// When options are to be generated in `<optgroup>` sections, options should be
/// nested under the `children` key of each group object. The label for the
/// group should be specified as the `text` property on the group's
/// corresponding data object.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Group {
    /// The label for the group should be specified as the `text` property on
    /// the group's corresponding data object.
    pub text: String,
    /// Options should be nested under the `children` key of each group object.
    pub children: Vec<Record>,
} // Group

// -----------------------------------------------------------------------------
//
/// For the `select2::grouped` module to work with your records, you must
/// implement the `Groupable` trait it. This trait will return a
/// `GroupableRecord` struct. In other words, you must implement a trait that
/// converts your record into this struct.
///
/// Select2 can render programmatically supplied data from an array or remote
/// data source (AJAX) as dropdown options. In order to accomplish this, Select2
/// expects a very specific data format. This format consists of a JSON object
/// containing an array of objects keyed by the `results` key.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct GroupableRecord {
    /// Just like with the `id` property, Select2 requires that the text that
    /// should be displayed for an option is stored in the `text` property.
    pub text: String,
    /// The label for the group should this record belongs to.
    pub group: String,
    /// You can also supply the `selected` properties for the options in this
    /// data structure.
    pub selected: bool,
    /// You can also supply the `disabled` properties for the options in this
    /// data structure.
    pub disabled: bool,
} // GroupableRecord

// -----------------------------------------------------------------------------

impl GroupableRecord {
    /// Combines a `GroupableRecord` with a `K` key to produce a `Select2Record`
    /// that can be returned to the user's client for use in the `Select2`
    /// plug-in.
    pub fn to_record<K: ToString>(
        &self,
        key: &K,
    ) -> Record {
        Record {
            id: key.to_string(),
            text: self.text.clone(),
            selected: self.selected,
            disabled: self.disabled,
        } // Record
    } // fn
} // impl

// -----------------------------------------------------------------------------
//
/// This `struct` represents the results of the search query. This should be
/// converted to `JSON` using your web framework (or something like
/// `serde_json`), and then returned to the `Select2` jQuery plug-in.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct GroupedResults {
    /// This format consists of a JSON object containing an array of objects
    /// keyed by the `results` key.
    pub results: Vec<Group>,
    /// The response object may also contain pagination data, if you would like
    /// to use the "infinite scroll" feature. This should be specified under the
    /// `pagination` key.
    pub pagination: Pagination,
} // GroupedResults