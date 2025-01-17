#![allow(clippy::inline_always)]

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
    #[inline]
    #[allow(clippy::map_unwrap_or)] // `map_or_else` is illegible
    pub(crate) fn internal_keyword_search(
        &self,
        keyword: &str,
    ) -> impl Iterator<Item = &K> {
        // Attempt to get matching keys for the search keyword from BTreeMap:
        self.b_tree_map
            .get(keyword)
            // Return an iterator over the keys for the keyword:
            .map(|keys| keys
                // Iterate over all matching keys and only return
                // `maximum_keys_per_keyword` number of keys:
                .iter()
                // Only return `maximum_keys_per_keyword` number of keys.
                .take(self.maximum_keys_per_keyword)
            )
            // If there are no keys for the keyword, then return an empty
            // iterator. The `empty_key_set` allows us to trick the compiler
            // into returning an empty iterator.
            .unwrap_or_else(|| self.empty_key_set
                .iter()
                .take(self.maximum_keys_per_keyword))
    } // fn
} // impl