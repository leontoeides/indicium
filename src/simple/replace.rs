// -----------------------------------------------------------------------------

use crate::simple::{indexable::Indexable, search_index::SearchIndex};
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + Eq + Hash + PartialEq> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Replaces (or updates) the value for a key-value pair in the search
    /// index.

    pub fn replace(
        &mut self,
        key: &K,
        before: &dyn Indexable,
        after: &dyn Indexable,
    ) {
        // Remove all references to the old record and its keywords:
        self.remove(key, before);
        // Index the updated record:
        self.insert(key, after);
    } // fn

} // impl