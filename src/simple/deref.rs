// -----------------------------------------------------------------------------

use crate::simple::search_index::SearchIndex;
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + Eq + Hash + PartialEq> Deref for SearchIndex<K> {
    type Target = BTreeMap<String, Vec<K>>;
    fn deref(&self) -> &Self::Target {
        &self.b_tree_map
    } // fn
} // impl