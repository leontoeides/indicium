use crate::simple::search_index::SearchIndex;
use std::clone::Clone;
use std::cmp::{Eq, Ord, PartialEq};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::Send;
#[cfg(feature = "rayon")]
use rayon::slice::ParallelSliceMut;

// -----------------------------------------------------------------------------

impl<'a, K: 'a + Clone + Debug + Eq + Hash + Ord + PartialEq + Send> SearchIndex<K>
where
    &'a K: Send {

    // -------------------------------------------------------------------------
    //
    /// Returns the keys resulting from the search string. The search string may
    /// contain several keywords.

    pub fn or_search(&'a self, string: &'a str) -> Vec<&'a K> {

        // Split search `String` into keywords (according to the `SearchIndex`
        // settings):
        let keywords: Vec<String> = self.string_keywords(&String::from(string), true);

        // This `HashMap` is used to count the number of hits for each resulting
        // key. This is so we can return search results in order of relevance:
        let mut search_results: HashMap<&K, usize> = HashMap::new();

        // Get each keyword from our `BTreeMap`, record the resulting keys in
        // a our `HashMap`, and track the hit-count for each key:
        keywords
            // Iterate over the keywords supplied in the search string:
            .iter()
            // For each keyword in the search string:
            .for_each(|keyword| {
                // Search for keyword in our `BTreeMap`:
                self.internal_keyword_search(keyword)
                    // Iterate over the resulting keys (if any):
                    .iter()
                    // For each resulting key from the keyword search:
                    .for_each(|key| match search_results.get_mut(key) {
                        // Add "hit" to counter for an already existing key:
                        Some(result_entry) => { *result_entry += 1 },
                        // No record for this key, initialize to one hit:
                        None => { search_results.insert(key, 1); },
                    }) // for_each
            }); // for_each

        // At this point, we have a list of resulting keys in a `HashMap`. The
        // hash map value holds the number of times each key has been returned
        // in the above keywords search.
        //
        // We want to sort these keys by descending hit-count. First, we must
        // convert it to a `Vec` so this can be done:

        let mut search_results: Vec<(&K, usize)> = search_results
            // Iterate over keys in the hash map:
            .iter()
            // Convert the key-value pair into a tuple element:
            .map(|(key, value)| (*key, *value))
            // Collect the tuple elements into a `Vec`:
            .collect();

        // Sort the tuple elements by hit-count descending:
        #[cfg(feature = "rayon")]
        search_results.par_sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        #[cfg(not(feature = "rayon"))]
        search_results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Return the search results to the user:
        search_results
            // Iterate over the tuple elements:
            .iter()
            // Only return `maximum_search_results` number of keys:
            .take(self.maximum_search_results)
            // Remove the hit-count from the tuple, returning only the key:
            .map(|(key, _value)| *key)
            // Collect the keys into a `Vec`:
            .collect()

    } // fn

} // impl