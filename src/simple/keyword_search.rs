use crate::simple::search_index::SearchIndex;
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
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

    pub fn keyword_search(&self, keyword: &str) -> Vec<&K> {

        // If case insensitivity set, convert the keyword to lower case:
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
                // Only return `maximum_search_results` number of keys.
                // Note: `search` requires a complete list of keys from this
                // function to work correctly, so this `take` has been commented
                // out for now. Perhaps we could add a `observe_maximum_results`
                // `bool` parameter for this function?
                // .take(self.maximum_search_results)
                // Collect all resulting keys into a `Vec`:
                .collect()

            // -> If fuzzy matching were to be implemented for
            // `indicium::simple` it would be put here. <-

        } else {

            // The search keyword did not result in any matches. Return an
            // empty `Vec`:

            vec![]

        } // if

    } // fn

} // impl