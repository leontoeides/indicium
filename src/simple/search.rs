use crate::simple::{SearchIndex, SearchType};
use std::cmp::Ord;
use std::hash::Hash;
use std::marker::Send;

// -----------------------------------------------------------------------------

impl<'a, K: 'a + Hash + Ord + Send> SearchIndex<K>
where
    &'a K: Send {

    // -------------------------------------------------------------------------
    //
    /// Returns the keys resulting from the search string. The search string may
    /// contain several keywords.

    pub fn search(&'a self, string: &'a str) -> Vec<&'a K> {

        match &self.search_type {
            SearchType::Keyword => self.keyword_search(string).iter().cloned().collect(),
            SearchType::And => self.and_search(string),
            SearchType::Or => self.or_search(string),
        } // match

    } // fn

    // -------------------------------------------------------------------------
    //
    /// Returns the keys resulting from the search string. The search string may
    /// contain several keywords.

    pub fn search_type(&'a self, search_type: &SearchType, string: &'a str) -> Vec<&'a K> {

        match search_type {
            SearchType::Keyword => self.keyword_search(string).iter().cloned().collect(),
            SearchType::And => self.and_search(string),
            SearchType::Or => self.or_search(string),
        } // match

    } // fn

} // impl