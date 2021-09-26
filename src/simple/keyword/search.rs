use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;
use std::collections::BTreeSet;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// The keyword_search function will return keys for records that match the
    /// keyword provided by the caller. Each resulting key can then be used to
    /// retrieve the full record from its collection. The search keyword must be
    /// an exact match. The results are returned in undefined order.
    ///
    /// Example usage:
    ///
    /// ```rust
    ///
    /// let resulting_keys: Vec<usize> =
    ///     search_index.keyword_search(&"helicopter".to_string());
    ///
    /// assert_eq!(resulting_keys, Some(vec![&1]));
    /// ```
    ///
    /// Search only supports exact keyword matches and does not use fuzzy
    /// matching. Consider providing the autocomplete feature to your users as
    /// an ergonomic alternative to fuzzy matching.
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