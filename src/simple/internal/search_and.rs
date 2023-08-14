use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;
use std::collections::BTreeSet;
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

    pub(crate) fn internal_search_and(&self, keywords: &[String]) -> BTreeSet<&K> {

        // This `BTreeSet` is used to contain the search results:
        let mut search_results: Option<BTreeSet<&K>> = None;

        // Get each keyword from our `BTreeMap`, and intersect the resulting
        // keys with our current keys:
        keywords
            // Iterate over the keywords supplied in the search string:
            .iter()
            // For each keyword in the search string:
            .for_each(|keyword| {

                // Attempt to retrieve keyword from search index. If keyword
                // found, intersect keyword records with search results records.
                // If keyword not found, empty search results:
                match self.b_tree_map.get(keyword) {

                    // Keyword found. Update `search_results` with product of an
                    // intersection with this keyword's records:
                    Some(keyword_results) => search_results = Some(

                        // Check if `search_results` is already populated:
                        match &search_results {

                            // If `search_results` is is not empty, intersect
                            // the current keyword's results with the master
                            // search results:
                            Some(search_results) => search_results
                                // Iterate over each search result record:
                                .iter()
                                // Intersect the search result record with the
                                // keyword results. If the search result record
                                // doesn't exist in this keyword's results,
                                // filter it out:
                                .filter(|key|
                                    keyword_results.contains(key)
                                )
                                // Copy each key from the `Intersection`
                                // iterator or we'll get a doubly-referenced
                                // `&&K` key:
                                .cloned()
                                // And collect each key into a `BTreeSet` that
                                // will become the new `search_results`:
                                .collect(),

                            // If `search_results` is empty, initialize it with
                            // the first keyword's full search results:
                            None => self.internal_keyword_search(keyword),

                        } // match

                    ), // Some

                    // Any keyword that returns no results will short-circuit
                    // the search results into an empty set:
                    None => search_results = Some(BTreeSet::new()),

                } // match

            }); // for_each

        // For debug builds:
        #[cfg(debug_assertions)]
        if let Some(search_results) = &search_results {
            if search_results.len() >= self.maximum_keys_per_keyword {
                tracing::warn!(
                    "Internal table limit of {} results has been exceeded on internal `and` search. \
                    Data has been dropped. \
                    This will impact accuracy of results. \
                    For this data set, consider using a more comprehensive search solution like MeiliSearch.",
                    self.maximum_keys_per_keyword
                ); // warn!
            } // if
        } // if

        // Return search results:
        match search_results {
            // If master `search_results` is not empty, return it:
            Some(search_results) => search_results,
            // If master `search_results` is empty, return an empty `BTreeSet`:
            None => BTreeSet::new(),
        } // match

    } // fn

} // impl