mod and;
mod keyword;
mod or;

// -----------------------------------------------------------------------------

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
    /// The `search` function will return keys as the search results. Each
    /// resulting key can then be used to retrieve the full record from its
    /// collection. Search keywords must be an exact match.
    ///
    /// Search only supports exact keyword matches and does not use fuzzy
    /// matching. Consider providing the `autocomplete` feature to your users as
    /// an ergonomic alternative to fuzzy matching.
    ///
    /// Search behaviour can be changed by setting the [`SearchType`] in the
    /// `SearchIndex`. See also: [`SearchIndexBuilder`] or
    /// [`SearchIndex::new()`].
    ///
    /// [`SearchType`]: enum.SearchType.html
    /// [`SearchIndexBuilder`]: struct.SearchIndexBuilder.html
    /// [`SearchIndex::new()`]: struct.SearchIndex.html#method.new

    pub fn search(&'a self, string: &'a str) -> Vec<&'a K> {

        match &self.search_type {
            SearchType::And => self.and_search(string),
            SearchType::Keyword => self.keyword_search(string).iter().cloned().collect(),
            SearchType::Or => self.or_search(string),
        } // match

    } // fn

    // -------------------------------------------------------------------------
    //
    /// The `search` function will return keys as the search results. Each
    /// resulting key can then be used to retrieve the full record from its
    /// collection. Search keywords must be an exact match.
    ///
    /// Search only supports exact keyword matches and does not use fuzzy
    /// matching. Consider providing the `autocomplete` feature to your users as
    /// an ergonomic alternative to fuzzy matching.
    ///
    /// Search behaviour can be changed by using various `SearchType` variants
    /// as the first parameter for the method call. See [`SearchType`] for more
    /// information on the different search types.
    ///
    /// [`SearchType`]: enum.SearchType.html

    pub fn search_type(&'a self, search_type: &SearchType, string: &'a str) -> Vec<&'a K> {

        match search_type {
            SearchType::And => self.and_search(string),
            SearchType::Keyword => self.keyword_search(string).iter().cloned().collect(),
            SearchType::Or => self.or_search(string),
        } // match

    } // fn

} // impl