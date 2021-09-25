use crate::simple::conjunction::Conjunction;
use crate::simple::search_index::SearchIndex;
use std::clone::Clone;
use std::cmp::{Eq, Ord, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::{Send, Sync};

// -----------------------------------------------------------------------------

impl<'a, K: 'a + Clone + Debug + Eq + Hash + Ord + PartialEq + Send + Sync> SearchIndex<K>
where
    &'a K: Send {

    // -------------------------------------------------------------------------
    //
    /// Returns the keys resulting from the search string. The search string may
    /// contain several keywords.

    pub fn search(&'a self, string: &'a str) -> Vec<&'a K> {

        match &self.conjunction {
            Conjunction::And => self.and_search(string),
            Conjunction::Or => self.or_search(string),
        } // match

    } // fn

} // impl