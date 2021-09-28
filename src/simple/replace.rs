use crate::simple::{indexable::Indexable, search_index::SearchIndex};
use std::clone::Clone;
use std::cmp::Ord;

// -----------------------------------------------------------------------------

impl<K: Clone + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Replaces (or updates) the value for a key-value pair in the search
    /// index.
    ///
    /// Note that for the search results to be accurate, it is important to
    /// update the search index as the collection is updated.

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