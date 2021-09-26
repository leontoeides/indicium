use crate::simple::{indexable::Indexable, search_index::SearchIndex};
use std::clone::Clone;
use std::cmp::Ord;
use std::collections::BTreeSet;

// -----------------------------------------------------------------------------

impl<K: Clone + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// ### Indexing a Collection
    ///
    /// To index an existing collection, we can iterate over the collection. For
    /// each record, we will insert it into the search index. Once the index has
    /// been populated, you can use the `autocomplete` and `search` functions.
    ///
    /// This should look something like these two examples:
    ///
    /// #### Vec
    ///
    /// ```rust
    /// use indicium::simple::SearchIndex;
    ///
    /// let my_vec: Vec<MyStruct> = Vec::new();
    ///
    /// // In the case of a `Vec` collection, we use the index as our key.  A
    /// // `Vec` index is a `usize` type. Therefore we will instantiate
    /// // `SearchIndex` as `SearchIndex<usize>`.
    ///
    /// let mut search_index: SearchIndex<usize> = SearchIndex::default();
    ///
    /// my_vec
    ///     .iter()
    ///     .enumerate()
    ///     .for_each(|(index, element)|
    ///         search_index.insert(&index, element)
    ///     );
    /// ```
    ///
    /// #### HashMap
    ///
    /// ```rust
    /// use std::collections::HashMap;
    /// use indicium::simple::SearchIndex;
    ///
    /// let my_hash_map: HashMap<String, MyStruct> = HashMap::new();
    ///
    /// // In the case of a `HashMap` collection, we use the hash map's key as
    /// // the `SearchIndex` key. In our hypothetical example, we will use
    /// // MyStruct's `title` as a the key which is a `String` type. Therefore
    /// // we will instantiate `HashMap<K, V>` as HashMap<String, MyStruct> and
    /// // `SearchIndex<K>` as `SearchIndex<String>`.
    ///
    /// let mut search_index: SearchIndex<String> = SearchIndex::default();
    ///
    /// my_hash_map
    ///     .iter()
    ///     .for_each(|(key, value)|
    ///         search_index.insert(key, value)
    ///     );
    /// ```
    ///
    /// As long as the `Indexable` trait was implemented for your value type,
    /// the above examples will index a previously populated `Vec` or `HashMap`.
    /// However, the preferred method for large collections is to `insert` into
    /// the `SearchIndex` as you insert into your collection (Vec, HashMap,
    /// etc.)

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
                    Some(keys) => { keys.insert(key.clone()); },
                    // If keyword was not found in search index, initialize
                    // _keyword entry_ with the _key reference_ for this record:
                    None => {
                        let mut b_tree_set = BTreeSet::new();
                        b_tree_set.insert(key.clone());
                        self.b_tree_map.insert(keyword.clone(), b_tree_set);
                    }, // None
                } // match
            ) // for_each

    } // fn

} // impl