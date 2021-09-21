// -----------------------------------------------------------------------------

use crate::simple::search_index::SearchIndex;
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + Eq + Hash + PartialEq> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Returns the keys resulting from the search string. The search string may
    /// contain several keywords.

    pub fn search(&self, string: &str) -> Vec<K> {

        // Split search `String` into keywords according to the `SearchIndex`
        // settings:
        let keywords = self.string_keywords(string);

        // This `HashMap` is used to count the number of hits for each resulting
        // key. This is so we can return search results in order of relevance:
        let mut search_results: HashMap<K, usize> = HashMap::new();

        // Get each keyword from our `BTreeMap`, record the resulting keys in
        // a our `HashMap`, and track the hit-count for each key:
        keywords
            // Iterate over the keywords supplied in the search string:
            .iter()
            // For each keyword in the search string:
            .for_each(|keyword|
                // Search for keyword in our `BTreeMap`:
                self.keyword_search(keyword)
                    // Iterate over the resulting keys (if any):
                    .iter()
                    // For each resulting key from the keyword search:
                    .for_each(|key| match search_results.get_mut(key) {
                        // Add "hit" to counter for an already existing key:
                        Some(result_entry) => { *result_entry += 1 },
                        // No record for this key, initialize to one hit:
                        None => { search_results.insert((*key).clone(), 1); },
                    }) // for_each
            ); // for_each

        // At this point, we have a list of keys in a `HashMap`. The hash map
        // value holds the number of times each key has been returned in the
        // above keywords search.
        //
        // We want to sort these keys by descending hit-count. We'll convert it
        // to a `Vec` so this can be done:

        let mut search_results: Vec<(K, usize)> = search_results
            // Iterate over keys in the hash map:
            .iter()
            // Convert the key-value pair into a tuple element:
            .map(|(key, value)| (key.clone(), *value))
            // Collect the tuple elements into a `Vec`:
            .collect();

        // Sort the tuple elements by hit-count descending:
        search_results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Return the search results to the user:
        search_results
            // Iterate over the tuple elements:
            .iter()
            // Only return `maximum_search_results` number of keys:
            .take(self.maximum_search_results)
            // Remove the hit-count from the tuple, returning only the key:
            .map(|(key, _value)| key.clone())
            // Collect the keys into a `Vec`:
            .collect()

    } // fn

} // impl