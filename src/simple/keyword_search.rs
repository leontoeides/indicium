use crate::simple::search_index::SearchIndex;
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + Eq + Hash + PartialEq> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Returns the keys resulting from the single keyword search.
    ///
    /// The provided string is expected to be only a single keyword. For
    /// multi-keyword support see the `search` method.
    //
    /// Note: This function is a variation of the `keyword_search_internal`
    /// function. If this function is modified, it is likely the
    /// `keyword_search_internal` function should be updated also.
    ///
    /// The difference between these two functions is that `keyword_search`
    /// observes `maximum_search_results`, while `keyword_search_internal` does
    /// not.

    pub fn keyword_search(&self, keyword: &str) -> HashSet<&K> {

        // If case sensitivity set, leave case intact. Otherwise, convert
        // keyword to lower case:
        let keyword = match self.case_sensitive {
            true => keyword.to_string(),
            false => keyword.to_lowercase(),
        }; // match

        // Attempt to get matching keys for the search keyword from BTreeMap:
        if let Some(keys) = self.b_tree_map.get(&keyword) {

            // Attempt to get matching keys for search keyword:
            keys
                // Iterate over all matching keys and only return
                // `maximum_search_results` number of keys:
                .iter()
                // Only return `maximum_search_results` number of keys:
                .take(self.maximum_search_results)
                // Collect all resulting keys into a `HashSet`:
                .collect()

            // -> If fuzzy matching were to be implemented for
            // `indicium::simple` it would probably be put here. <-

        } else {

            // The search keyword did not result in any matches. Return an
            // empty `HashSet`:
            HashSet::new()

        } // if

    } // fn

} // impl