#![allow(clippy::inline_always)]

use std::collections::BTreeSet;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> crate::simple::search_index::SearchIndex<K> {
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
    #[inline(always)]
    #[allow(clippy::option_if_let_else)] // `map_or_else` is illegible
    pub(crate) fn internal_keyword_search(
        &self,
        keyword: &str,
    ) -> impl Iterator<Item = &K> {
        // Attempt to get matching keys for the search keyword from BTreeMap:
        if let Some(keys) = self.b_tree_map.get(keyword) {
            //let search_results: BTreeSet<&K> =
            keys
                // Iterate over all matching keys and only return
                // `maximum_keys_per_keyword` number of keys:
                .iter()
                // Only return `maximum_keys_per_keyword` number of keys.
                .take(self.maximum_keys_per_keyword)
                // Insert a reference to each resulting key into hash set:
                // .collect();

            // For debug builds:
            /* #[cfg(debug_assertions)]
            if search_results.len() >= self.maximum_keys_per_keyword {
                tracing::warn!(
                    "Internal table limit of {} results has been exceeded on search. \
                    Data has been dropped. \
                    This will impact accuracy of results. \
                    For this data set, consider using a more comprehensive search solution like MeiliSearch.",
                    self.maximum_keys_per_keyword
                ); // warn!
            } // if

            search_results */
        } else {
            // BTreeSet::new()
            self.empty_b_tree_set
                // Iterate over all matching keys and only return
                // `maximum_keys_per_keyword` number of keys:
                .iter()
                // Only return `maximum_keys_per_keyword` number of keys.
                .take(self.maximum_keys_per_keyword)
        } // if
    } // fn
} // impl