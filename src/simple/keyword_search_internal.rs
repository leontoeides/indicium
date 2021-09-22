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
    ///
    /// Note: This function is a variation of the `keyword_search` function. If
    /// this function is modified, it is likely the `keyword_search` function
    /// should be updated also.
    ///
    /// The difference between these two functions is that `keyword_search`
    /// observes `maximum_search_results`, while `keyword_search_internal` does
    /// not.

    pub(crate) fn keyword_search_internal(&self, keyword: &str) -> HashSet<&K> {

        // If case insensitivity set, convert the keyword to lower case:
        let keyword = match self.case_sensitive {
            true => keyword.to_string(),
            false => keyword.to_lowercase(),
        }; // match

        // This hash set will contain all of the keys found for the keyword and
        // be returned to the caller:
        let mut hash_set: HashSet<&K> = HashSet::new();

        // Attempt to get matching keys for the search keyword from BTreeMap:
        if let Some(keys) = self.b_tree_map.get(&keyword) {

            // Attempt to get matching keys for search keyword:
            keys
                // Iterate over all matching keys and only return
                // `maximum_search_results` number of keys:
                .iter()
                // Insert each resulting key into the hash set:
                .for_each(|key| { hash_set.insert(key); })

            // -> If fuzzy matching were to be implemented for
            // `indicium::simple` it would probably be put here. <-

        } // if

        hash_set

    } // fn

} // impl