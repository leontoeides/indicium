// Conditionally select hash map type based on feature flags:
#[cfg(feature = "gxhash")]
type HashSet<T> = std::collections::HashSet<T, gxhash::GxBuildHasher>;
#[cfg(all(feature = "ahash", not(feature = "gxhash")))]
use ahash::HashSet;
#[cfg(all(not(feature = "ahash"), not(feature = "gxhash")))]
use std::collections::HashSet;

// Static dependencies:
use crate::simple::{indexable::Indexable, search_index::SearchIndex};
use kstring::KString;
use std::collections::BTreeSet;
use std::{clone::Clone, cmp::Ord};

// -----------------------------------------------------------------------------

impl<K: Clone + Ord> SearchIndex<K> {
    // -------------------------------------------------------------------------
    //
    /// Inserts a key-value pair into the search index.
    ///
    /// Note that for the search results to be accurate, it is important to
    /// update the search index as the collection is updated. If an element is
    /// inserted into your collection, it should also be inserted into the
    /// search index.
    ///
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
    /// # use indicium::simple::{Indexable, SearchIndex};
    /// #
    /// # struct MyStruct {
    /// #   title: String,
    /// #   year: u16,
    /// #   body: String,
    /// # }
    /// #
    /// # impl Indexable for MyStruct {
    /// #   fn strings(&self) -> Vec<String> {
    /// #       vec![
    /// #           self.title.clone(),
    /// #           self.year.to_string(),
    /// #           self.body.clone(),
    /// #       ]
    /// #   }
    /// # }
    /// #
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
    /// # use indicium::simple::{Indexable, SearchIndex};
    /// # use std::collections::HashMap;
    /// #
    /// # struct MyStruct {
    /// #   title: String,
    /// #   year: u16,
    /// #   body: String,
    /// # }
    /// #
    /// # impl Indexable for MyStruct {
    /// #   fn strings(&self) -> Vec<String> {
    /// #       vec![
    /// #           self.title.clone(),
    /// #           self.year.to_string(),
    /// #           self.body.clone(),
    /// #       ]
    /// #   }
    /// # }
    /// #
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
    ///
    /// #### Pro-Tip: Enum Keys
    ///
    /// You can make a single, universal search index for all of your
    /// collections. This can be done by making an `enum` key that represents
    /// both the collection and the key. For example:
    ///
    /// ```rust
    /// # use indicium::simple::SearchIndex;
    /// #
    /// #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
    /// enum CollectionKey {
    ///     MyVecCollection(usize),
    ///     MyHashMapCollection(String),
    /// }
    ///
    /// let search_index: SearchIndex<CollectionKey> = SearchIndex::default();
    /// ```
    ///
    /// You can use the enum's variants to represent your different collections.
    /// Each variant's associated data can hold the `key` for your record.
    ///
    /// Note that I couldn't implement the `FromIterator` trait which would
    /// allow a caller to `.collect();` into the `SearchIndex`. `FromIterator`
    /// works with an owned iterator (`IntoIterator`) and uses owned values
    /// only. If there were a similar trait that worked with borrowed values,
    /// it would be do-able.

    #[tracing::instrument(level = "trace", name = "search index insert", skip(self, key, value))]
    pub fn insert(&mut self, key: &K, value: &dyn Indexable) {
        // Get all keywords for the `Indexable` record:
        let mut keywords: HashSet<KString> = self.indexable_keywords(value);

        // If `dump_keyword` feature is turned on, ensure that all records are
        // attached to this special keyword:
        if let Some(dump_keyword) = &self.dump_keyword {
            keywords.insert(dump_keyword.as_ref().into());
        } // if

        // Iterate over the keywords:
        keywords
            .into_iter()
            // For each keyword, add this record's _key_ to the _keyword entry_:
            .for_each(|keyword|
                // Attempt to get mutuable reference to the _keyword entry_ in
                // the search index:
                if let Some(keys) = self.b_tree_map.get_mut(&keyword) {
                    // Check if the maximum number of keys per keyword
                    // (records per keyword) limit has been reached. Note
                    // that the `dump_keyword` does not observe this
                    // limit.
                    if keys.len() < self.maximum_keys_per_keyword
                        || self.dump_keyword == Some(keyword.as_ref().into()) {
                            // If it hasn't, insert the key (record) into the
                            // list:
                            keys.insert(key.clone());
                    } else {
                        // If the limit has been reached, do not insert.
                        // Display warning for debug builds.
                        #[cfg(debug_assertions)]
                        tracing::warn!(
                            "Internal table limit of {} keys per keyword has been reached on insert. \
                            Record was not attached to `{}` keyword. \
                            This will impact accuracy of results. \
                            For this data set, consider using a more comprehensive search solution like MeiliSearch.",
                            self.maximum_keys_per_keyword,
                            keyword,
                        ); // warn!
                    } // if
                } else {
                    let mut b_tree_set = BTreeSet::new();
                    b_tree_set.insert(key.clone());
                    self.b_tree_map.insert(keyword.as_ref().into(), b_tree_set);
                } // match
            ); // for_each
    } // fn
} // impl
