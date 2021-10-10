use crate::select2::Record;
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;
use std::string::ToString;

// -----------------------------------------------------------------------------
//
/// For the `select2` module to work, you must implement the `Selectable` trait
/// for your record. This trait will return this `GroupableRecord` struct. In 
/// other words, you must convert your record into this `struct`.
///
/// Select2 can render programmatically supplied data from an array or remote
/// data source (AJAX) as dropdown options. In order to accomplish this, Select2
/// expects a very specific data format. This format consists of a JSON object
/// containing an array of objects keyed by the `results` key.
///
/// Select2 requires that each object contain an `id` and a `text` property.
/// Additional parameters passed in with data objects will be included on the
/// data objects that Select2 exposes.

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