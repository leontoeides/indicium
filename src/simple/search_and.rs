use crate::simple::search_index::SearchIndex;
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + Eq + Hash + PartialEq> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Returns the keys resulting from the search string. The search string may
    /// contain several keywords.

    pub fn search_and(&self, string: &str) -> HashSet<&K> {

        // Split search `String` into keywords (according to the `SearchIndex`
        // settings):
        let keywords: Vec<String> = self.string_keywords(string, false);

        // This `HashSet` is used to contain the search results:
        let mut search_results: Option<HashSet<&K>> = None;

        // Get each keyword from our `BTreeMap`, record the resulting keys in
        // a our `HashMap`, and track the hit-count for each key:
        keywords
            // Iterate over the keywords supplied in the search string:
            .iter()
            // For each keyword in the search string:
            .for_each(|keyword| {

                let keyword_results = self.keyword_search_internal(keyword);

                println!("Keyword: {} = {} results.", keyword, keyword_results.len());

                search_results = Some(match &search_results {
                    Some(search_results) => {
                        keyword_results
                            .intersection(search_results)
                            .into_iter()
                            .cloned()
                            .collect()
                    }, // Some
                    None => keyword_results,
                }); // match

                println!("Intersection: {} results.", search_results.as_ref().unwrap().len());

            }); // for_each

        match search_results {
            Some(search_results) => search_results,
            None => HashSet::new(),
        } // match

    } // fn

} // impl