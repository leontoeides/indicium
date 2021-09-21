use crate::simple::{indexable::Indexable, search_index::SearchIndex};
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + Eq + Hash + PartialEq> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Inserts a key-value pair into the search index.

    pub fn insert(&mut self, key: &K, value: &dyn Indexable) {

        // Get all keywords for the `Indexable` record:
        let keywords = self.indexable_keywords(value);

        // Iterate over the keywords:
        keywords
            .iter()
            // For each keyword, add this record's _key_ to the _keyword entry_:
            .for_each(|keyword|
                // Attempt to get mutuable reference to the _keyword entry_ in
                // the search index:
                match self.b_tree_map.get_mut(keyword) {
                    // If keyword was found in search index, add _key reference_
                    // for this record to _keyword entry_:
                    Some(keys) => keys.push(key.clone()),
                    // If keyword was not found in search index, initialize
                    // _keyword entry_ with the _key reference_ for this record:
                    None => {
                        self.b_tree_map.insert(
                            keyword.clone(),
                            vec![key.clone()]
                        ); // insert
                    }, // None
                } // match
            ) // for_each

    } // fn

} // impl