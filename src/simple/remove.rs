use crate::simple::{indexable::Indexable, search_index::SearchIndex};
use std::clone::Clone;
use std::cmp::Ord;

// -----------------------------------------------------------------------------

impl<K: Clone + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Removes a key-value pair from the search index.

    pub fn remove(&mut self, key: &K, value: &dyn Indexable) {

        // Get all keywords for the `Indexable` record:
        let keywords = self.indexable_keywords(value);

        // Iterate over the keywords:
        keywords
            .iter()
            // For each keyword, remove this record's _key_ from the _keyword
            // entry_:
            .for_each(|keyword| {
                // Attempt to get mutuable reference to the _keyword entry_ in
                // the search index:
                let is_empty = if let Some(keys) = self.b_tree_map.get_mut(keyword) {
                    // If keyword found in search index, remove the _key
                    // reference_ for this record from _keyword entry_:
                    keys.retain(|value| value != key);
                    // Return whether the _keyword entry_ is now empty or not:
                    keys.is_empty()
                } else {
                    // If keyword not found in search index, signal that we
                    // should **not** remove the _keyword entry_ because that
                    // would result in an error:
                    false
                }; // if
                // If the _keyword entry_ no longer contains any _key
                // references_, it is empty and we should remove the keyword
                // from the search index:
                if is_empty { self.b_tree_map.remove(keyword); }
            }) // for_each

    } // fn

} // impl