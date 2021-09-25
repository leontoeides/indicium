use crate::simple::search_index::SearchIndex;
use std::clone::Clone;
use std::cmp::{Eq, Ord, PartialEq};
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + Eq + Hash + Ord + PartialEq> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Returns the keys resulting from the single keyword search.
    ///
    /// The provided string is expected to be only a single keyword. For
    /// multi-keyword support see the `search` method.
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

        self.internal_keyword_search(&keyword)
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