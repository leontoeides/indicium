use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;
use std::collections::BTreeSet;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// The `search` function will return keys as the search results. Each
    /// resulting key can then be used to retrieve the full record from its
    /// collection. _This search method only accepts a single keyword as the
    /// search string._ Search keywords must be an exact match.
    ///
    /// The search string is expected to only contain a single keyword. This is
    /// the lightest and fastest search type. It is good for compact interfaces,
    /// where records are very simple, or data-sets are quite small.
    ///
    /// Search only supports exact keyword matches and does not use fuzzy
    /// matching. Consider providing the `autocomplete` feature to your users as
    /// an ergonomic alternative to fuzzy matching.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use indicium::simple::SearchIndex;
    /// # use std::collections::BTreeSet;
    /// #
    /// # let mut search_index: SearchIndex<usize> = SearchIndex::default();
    /// #
    /// let resulting_keys: BTreeSet<&usize> =
    ///     search_index.keyword_search(&"helicopter".to_string());
    ///
    /// assert_eq!(
    ///     // Convert `BTreeSet<&usize>` to `Vec<&usize>`:
    ///     resulting_keys.iter().cloned().collect::<Vec<&usize>>(),
    ///     vec![&1]
    /// );
    /// ```
    //
    // Note: This function is a variation of the `internal_keyword_search`
    // function. If this function is modified, it is likely the
    // `internal_keyword_search` function should be updated also.
    //
    // The difference between these two functions is that `keyword_search`
    // observes `maximum_search_results`, while `internal_keyword_search` does
    // not.

    pub fn keyword_search(&self, keyword: &str) -> BTreeSet<&K> {

        // If case sensitivity set, leave case intact. Otherwise, convert
        // keyword to lower case:
        let keyword = match self.case_sensitive {
            true => keyword.to_string(),
            false => keyword.to_lowercase(),
        }; // match

        self.internal_keyword_search(&String::from(&keyword))
            // Iterate the results of the keyword search:
            .iter()
            // Only return `maximum_search_results` number of keys:
            .take(self.maximum_search_results)
            // Take ownership of reference so we return `&K` and not `&&K`:
            .cloned()
            // Collect all resulting keys into a `BTreeSet`:
            .collect()

    } // fn

} // impl