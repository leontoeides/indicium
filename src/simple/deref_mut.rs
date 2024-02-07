use crate::simple::search_index::SearchIndex;
use std::{cmp::Ord, ops::DerefMut};

// -----------------------------------------------------------------------------
//
/// Dereferencing a `SearchIndex<K>` will give access the underlying `BTreeMap`.

impl<K: Ord> DerefMut for SearchIndex<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.b_tree_map
    } // fn
} // impl
