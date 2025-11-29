use crate::simple::search_index::SearchIndex;
use kstring::KString;
use std::{collections::BTreeSet, hash::Hash};

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {
    /// This search function will return keys as the search results. Each
    /// resulting key can then be used to retrieve the full record from its
    /// collection. _This search method accepts multiple keywords in the search
    /// string._ Search keywords must be an exact match.
    ///
    /// Search only supports exact keyword matches and does not use fuzzy
    /// matching. Consider providing the `autocomplete` feature to your users as
    /// an ergonomic alternative to fuzzy matching.
    #[allow(clippy::option_if_let_else)] // `map_or_else` is illegible
    pub(crate) fn internal_and_search(
        &self,
        keywords: &[KString],
    ) -> BTreeSet<&K> {
        // This `BTreeSet` is used to contain the search results:
        let mut search_results = BTreeSet::<&K>::new();

        // Get each keyword from our `BTreeMap`, and intersect the resulting
        // keys with our current keys:
        for keyword in keywords {
            // Attempt to retrieve keyword from search index. If keyword found,
            // intersect keyword records with search results records. If keyword
            // not found, empty search results:
            if let Some(keyword_results) = self.b_tree_map.get(keyword) {
                // Check if `search_results` is already populated:
                search_results = if search_results.is_empty() {
                    self
                        .internal_keyword_search(keyword)
                        .collect()
                } else {
                    search_results
                        // Iterate over each search result record:
                        .into_iter()
                        // Intersect the search result record with the
                        // keyword results. If the search result record
                        // doesn't exist in this keyword's results, filter
                        // it out:
                        .filter(|key| keyword_results.contains(key))
                        // And collect each key into a `BTreeSet` that will
                        // become the new `search_results`:
                        .collect()
                }
            } else {
                search_results.clear();
                return search_results
            } // if
        } // for_each

        // For debug builds:
        #[cfg(debug_assertions)]
        if search_results.len() >= self.maximum_keys_per_keyword {
            tracing::warn!(
                "Internal table limit of {} results has been exceeded on internal `and` search. \
                Data has been dropped. \
                This will impact accuracy of results. \
                For this data set, \
                consider using a more comprehensive search solution like MeiliSearch.",
                self.maximum_keys_per_keyword
            ); // warn!
        } // if

        // Return search results:
        search_results
    } // fn
} // impl
