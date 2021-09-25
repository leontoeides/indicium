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
    /// Returns the keys resulting from the search string. The search string may
    /// contain several keywords.

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