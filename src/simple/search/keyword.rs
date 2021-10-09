use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// This search function will return keys as the search results. Each
    /// resulting key can then be used to retrieve the full record from its
    /// collection. _This search method only accepts a single keyword as the
    /// search string._ Search keywords must be an exact match.
    ///
    /// The search string is expected to only contain a single keyword. This is
    /// the lightest and fastest type. It is good for compact interfaces, where
    /// records are very simple, or data-sets are quite small. Results are
    /// returned in lexographic order.
    ///
    /// Search only supports exact keyword matches and does not use fuzzy
    /// matching. Consider providing the `autocomplete` feature to your users as
    /// an ergonomic alternative to fuzzy matching.
    ///
    /// Basic usage:
    ///
    /// ```ignore
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
    /// let search_results = search_index.search_keyword(&20, "Wessex");
    ///
    /// assert_eq!(
    ///     // Convert `BTreeMap<&K>` to `Vec<&K>` for comparison:
    ///     search_results.iter().cloned().collect::<Vec<&usize>>(),
    ///     vec![&1]
    /// );
    /// ```
    //
    // Note: This function is a variation of the `internal_keyword_search`
    // function. If this function is modified, it is likely the
    // `internal_keyword_search` function should be updated also.
    //
    // The difference between these two functions is that `keyword_search`
    // observes `maximum_search_results`, while `internal_keyword_search` does
    // not.

    #[tracing::instrument(level = "trace", name = "Keyword Search", skip(self))]
    pub(crate) fn search_keyword(
        &self,
        maximum_search_results: &usize,
        keyword: &str,
    ) -> Vec<&K> {

        // If case sensitivity set, leave case intact. Otherwise, normalize
        // keyword to lower case:
        let keyword = match self.case_sensitive {
            true => keyword.to_string(),
            false => keyword.to_lowercase(),
        }; // match

        self.internal_keyword_search(&String::from(&keyword))
            // Iterate the results of the keyword search:
            .iter()
            // Only return `maximum_search_results` number of keys:
            .take(*maximum_search_results)
            // Take ownership of reference so we return `&K` and not `&&K`:
            .cloned()
            // Collect all resulting keys into a `Vec`:
            .collect()

    } // fn

} // impl