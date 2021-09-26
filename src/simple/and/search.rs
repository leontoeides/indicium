use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;
use std::collections::BTreeSet;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

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
    /// ### _And_ Searches
    ///
    /// The default logical conjuction for multiple keywords is `And`. For
    /// example, a search of `this that` will only return records containing
    /// keywords both `this` and `that`. In other words, _all_ keywords must be
    /// present in a record for it to be returned as a result. This search is
    /// restrictive. For this search, the results are returned in lexographic
    /// order. This conjuction uses less CPU resources than `Or`.
    ///
    /// Example usage:
    ///
    /// ```rust
    /// use crate::simple::conjunction::Conjunction;
    /// use crate::simple::search_index::SearchIndex;
    ///
    /// let mut search_index: SearchIndex<String> =
    ///     SearchIndexBuilder<String>::default()
    ///         .conjuction(Conjunction::And)
    ///         .build();
    ///
    /// // ...Search index populated here...
    ///
    /// let resulting_keys: Vec<usize> =
    ///     search_index.search(&"helicopter".to_string());
    ///
    /// assert_eq!(resulting_keys, Some(vec![&1]));
    /// ```

    pub fn and_search(&self, string: &str) -> Vec<&K> {

        // Split search `String` into keywords (according to the `SearchIndex`
        // settings):
        let keywords: Vec<String> = self.string_keywords(string, false);

        // This `BTreeSet` is used to contain the search results:
        let mut search_results: Option<BTreeSet<&K>> = None;

        // Get each keyword from our `BTreeMap`, and intersect the resulting
        // keys with our current keys:
        keywords
            // Iterate over the keywords supplied in the search string:
            .iter()
            // For each keyword in the search string:
            .for_each(|keyword| {

                // Search for keyword in our `BTreeMap`:
                let keyword_results = self.internal_keyword_search(keyword);

                // Update `search_results` with product of `intersection`:
                search_results = Some(
                    // Check if `search_results` is already populated:
                    match &search_results {
                        // If `search_results` is is not empty...
                        Some(search_results) => {
                            // ...intersect the current keyword's search results
                            // with the master search results:
                            keyword_results
                                // Intersection will only keep the values that
                                // are both in `search_results` and
                                // `keyword_results`.
                                .intersection(search_results)
                                // The `intersection` function will return an
                                // `Intersection` type that we can iterate over:
                                .into_iter()
                                // Copy each key from the `Intersection`
                                // iterator or we'll get a doubly-referenced
                                // `&&K` key:
                                .cloned()
                                // And collect each key into a `BTreeSet` that
                                // will become the new `search_results`.
                                .collect()
                        }, // Some
                        // If `search_results` is empty, initialize it with the
                        // first keyword's full search results:
                        None => keyword_results,
                    } // match
                ); // Some

            }); // for_each

        // Return search results:
        match search_results {
            // If `search_results` is is not empty, convert the `BTreeSet` to a
            // `Vec` for caller while observing `maximum_search_results`:
            Some(search_results) => search_results
                .iter()
                .take(self.maximum_search_results)
                .cloned()
                .collect(),
            // If `search_results` is empty, return an empty `Vec`:
            None => Vec::new(),
        } // match

    } // fn

} // impl