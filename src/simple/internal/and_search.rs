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
    /// collection. _This search method accepts multiple keywords in the search
    /// string._ Search keywords must be an exact match.
    ///
    /// Search only supports exact keyword matches and does not use fuzzy
    /// matching. Consider providing the `autocomplete` feature to your users as
    /// an ergonomic alternative to fuzzy matching.

    pub(crate) fn internal_search_and(&self, keywords: &[String]) -> HashSet<&K> {

        // This `HashSet` is used to contain the search results:
        let mut search_results: Option<HashSet<&K>> = None;

        // Get each keyword from our `BTreeMap`, and intersect the resulting
        // keys with our current keys:
        keywords
            // Iterate over the keywords supplied in the search string:
            .iter()
            // For each keyword in the search string:
            .for_each(|keyword| {

                // Search for the keyword in our search index `BTreeMap`:
                let keyword_results = self.internal_keyword_search(keyword);

                // Update `search_results` with product of `intersection`:
                search_results = Some(
                    // Check if `search_results` is already populated:
                    match &search_results {
                        // If master `search_results` is not empty...
                        Some(search_results) => {
                            // ...intersect the current keyword's search results
                            // with the master search results:
                            keyword_results
                                // Intersection will only keep the values that
                                // are both in `search_results` and
                                // `keyword_results`.
                                .intersection(search_results)
                                // The `intersection` method will return an
                                // `Intersection` lazy iterator that we must
                                // iterate over:
                                .into_iter()
                                // Copy each `&K` key from the `Intersection`
                                // iterator or we'll get produce a
                                // doubly-referenced `&&K` key:
                                .cloned()
                                // And collect each key into a `HashSet` that
                                // will become the new `search_results`.
                                .collect()
                        }, // Some
                        // If master `search_results` is empty, initialize it
                        // with the first keyword's full search results:
                        None => keyword_results,
                    } // match
                ); // Some

            }); // for_each

        // Return search results:
        match search_results {
            // If master `search_results` is not empty, return it:
            Some(search_results) => search_results,
            // If master `search_results` is empty, return an empty `HashSet`:
            None => HashSet::new(),
        } // match

    } // fn

} // impl