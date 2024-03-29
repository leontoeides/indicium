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

// -----------------------------------------------------------------------------

impl<K: Clone + Ord> SearchIndex<K> {
    // -------------------------------------------------------------------------
    //
    /// Removes a key-value pair from the search index.
    ///
    /// Note that for the search results to be accurate, it is important to
    /// update the search index as the collection is updated. If an element is
    /// removed from your collection, it should also be removed from the search
    /// index.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use indicium::simple::{AutocompleteType, Indexable, SearchIndex, SearchType};
    /// # use pretty_assertions::assert_eq;
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
    /// # let my_vec = vec![
    /// #   MyStruct {
    /// #       title: "Harold Godwinson".to_string(),
    /// #       year: 1066,
    /// #       body: "Last crowned Anglo-Saxon king of England.".to_string(),
    /// #   },
    /// #   MyStruct {
    /// #       title: "Edgar Ætheling".to_string(),
    /// #       year: 1066,
    /// #       body: "Last male member of the royal house of Cerdic of Wessex.".to_string(),
    /// #   },
    /// #   MyStruct {
    /// #       title: "William the Conqueror".to_string(),
    /// #       year: 1066,
    /// #       body: "First Norman monarch of England.".to_string(),
    /// #   },
    /// #   MyStruct {
    /// #       title: "William Rufus".to_string(),
    /// #       year: 1087,
    /// #       body: "Third son of William the Conqueror.".to_string(),
    /// #   },
    /// #   MyStruct {
    /// #       title: "Henry Beauclerc".to_string(),
    /// #       year: 1100,
    /// #       body: "Fourth son of William the Conqueror.".to_string(),
    /// #   },
    /// # ];
    /// #
    /// # let mut search_index: SearchIndex<usize> = SearchIndex::default();
    /// #
    /// # my_vec
    /// #   .iter()
    /// #   .enumerate()
    /// #   .for_each(|(index, element)|
    /// #       search_index.insert(&index, element)
    /// #   );
    /// #
    /// let search_results = search_index.search("last");
    /// assert_eq!(search_results, vec![&0, &1]);
    ///
    /// search_index.remove(
    ///     &0,
    ///     &MyStruct {
    ///         title: "Harold Godwinson".to_string(),
    ///         year: 1066,
    ///         body: "Last crowned Anglo-Saxon king of England.".to_string(),
    ///     },
    /// );
    ///
    /// let search_results = search_index.search("last");
    /// assert_eq!(search_results, vec![&1]);
    /// ```

    #[tracing::instrument(level = "trace", name = "search index remove", skip(self, key, value))]
    pub fn remove(&mut self, key: &K, value: &dyn Indexable) {
        // Get all keywords for the `Indexable` record:
        let mut keywords: HashSet<KString> = self.indexable_keywords(value);

        // If `dump_keyword` feature is turned on, ensure that all records are
        // detached from this special keyword:
        if let Some(dump_keyword) = &self.dump_keyword {
            keywords.insert(dump_keyword.as_ref().into());
        } // if

        // Iterate over the keywords:
        for keyword in keywords {
            // Attempt to get mutuable reference to the _keyword entry_ in
            // the search index:
            let is_empty = self.b_tree_map.get_mut(&keyword).map_or(false, |keys| {
                // If keyword found in search index, remove the _key
                // reference_ for this record from _keyword entry_:
                keys.remove(key);
                // Return whether the _keyword entry_ is now empty or not:
                keys.is_empty()
            }); // map_or
            if is_empty {
                self.b_tree_map.remove(&keyword);
            }
        } // for_each
    } // fn
} // impl
