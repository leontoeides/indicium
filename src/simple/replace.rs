use crate::simple::{indexable::Indexable, search_index::SearchIndex};
use std::clone::Clone;
use std::cmp::Ord;

// -----------------------------------------------------------------------------

impl<K: Clone + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Replaces (or updates) the value for a key-value pair in the search
    /// index.
    ///
    /// Note that for the search results to be accurate, it is important to
    /// update the search index as the collection is updated.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use indicium::simple::{AutocompleteType, Indexable, SearchIndex, SearchType};
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
    /// search_index.replace(
    ///     &0,
    ///     &MyStruct {
    ///         title: "Harold Godwinson".to_string(),
    ///         year: 1066,
    ///         body: "Last crowned Anglo-Saxon king of England.".to_string(),
    ///     },
    ///     &MyStruct {
    ///         title: "Edward the Confessor".to_string(),
    ///         year: 1042,
    ///         body: "One of the last Anglo-Saxon kings of England.".to_string(),
    ///     },
    /// );
    ///
    /// let search_results = search_index.search("1042");
    /// assert_eq!(search_results, vec![&0]);
    /// ```

    #[tracing::instrument(level = "trace", name = "Search Index Replace", skip(self, key, before, after))]
    pub fn replace(
        &mut self,
        key: &K,
        before: &dyn Indexable,
        after: &dyn Indexable,
    ) {
        // Remove all references to the old record and its keywords:
        self.remove(key, before);
        // Index the updated record:
        self.insert(key, after);
    } // fn

} // impl