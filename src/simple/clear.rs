use crate::simple::SearchIndex;
use std::cmp::Ord;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Clears the search index, removing all elements.

    pub fn clear(&mut self) {
        self.b_tree_map.clear()
    } // fn

} // impl