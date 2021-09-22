use crate::simple::conjunction::Conjunction;
use crate::simple::search_index::SearchIndex;
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::Send;

// -----------------------------------------------------------------------------

impl<'a, K: 'a + Clone + Debug + Eq + Hash + PartialEq + Send> SearchIndex<K>
where
    &'a K: Send {

    // -------------------------------------------------------------------------
    //
    /// Returns the keys resulting from the search string. The search string may
    /// contain several keywords.

    pub fn search(&'a self, conjuction: &'a Conjunction, string: &'a str) -> Vec<&'a K> {

        match conjuction {
            Conjunction::And => self.search_and(string).iter().cloned().collect(),
            Conjunction::Or => self.search_or(string),
        } // match

    } // fn

} // impl