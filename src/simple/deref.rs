use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Deref;

// -----------------------------------------------------------------------------
//
/// Dereferencing a `SearchIndex<K>` will give access the underlying `BTreeMap`.

impl<K: Ord> Deref for SearchIndex<K> {
    type Target = BTreeMap<String, BTreeSet<K>>;
    fn deref(&self) -> &Self::Target {
        &self.b_tree_map
    } // fn
} // impl