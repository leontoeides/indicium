use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;
use std::collections::HashSet;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// This search function will return keys as the search results. Each
    /// resulting key can then be used to retrieve the full record from its
    /// collection. _This search method only accepts a single keyword as the
    /// search string._ Search keywords must be an exact match.
    ///
    /// Search only supports exact keyword matches and does not use fuzzy
    /// matching. Consider providing the `autocomplete` feature to your users as
    /// an ergonomic alternative to fuzzy matching.
    ///
    /// Note: This function is lower-level and for internal use only. It does
    /// not observe any settings such as _case-sensitivity_ or _maximum
    /// results_. These constraints should be observed at higher levels.

    pub(crate) fn internal_keyword_search(&self, keyword: &str) -> HashSet<&K> {

        // Check if the search index contains the user's keyword:
        let keyword: &str = if self.b_tree_map.contains_key(keyword) {
            // The search index contains the user's keyword, so we will use it
            // to return keys:
            keyword
        // If user's keyword not in search index, find the most similar keyword:
        } else if let Some(fuzzy_keyword) = self.strsim_keyword(keyword) {
            fuzzy_keyword
        // If search keyword not found and no similar keywords, return an empty
        // `HashSet`:
        } else {
            return HashSet::new()
        }; // if

        // Attempt to get matching keys for the search keyword from BTreeMap:
        let search_results: HashSet<&K> = if let Some(keys) = self.b_tree_map.get(keyword) {

            // Attempt to get matching keys for search keyword:
            keys
                // Iterate over all matching keys and only return
                // `maximum_search_results` number of keys:
                .iter()
                // Only return `maximum_search_results` number of keys:
                .take(self.maximum_keys_per_keyword)
                // Insert a reference to each resulting key into the hash set:
                .collect()

        } else {

            // The search keyword did not result in any matches. Return an
            // empty `HashSet`:
            HashSet::new()

        }; // if

        // For debug builds:
        #[cfg(debug_assertions)]
        if search_results.len() >= self.maximum_keys_per_keyword {
            tracing::warn!(
                "Internal table limit of {} results has been exceeded on search. \
                Data has been dropped. \
                This will impact accuracy of results. \
                For this data set, consider using a more comprehensive search solution like MeiliSearch.",
                self.maximum_keys_per_keyword
            ); // warn!
        } // if

        search_results

    } // fn

} // impl