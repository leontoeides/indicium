use crate::select2::Record;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Group {
    /// The label for the group should be specified as the `text` property on
    /// the group's corresponding data object.
    pub text: String,
    /// When options are to be generated in `<optgroup>` sections, options
    /// should be nested under the `children` key of each group object.
    pub children: Vec<Record>,
} // Group