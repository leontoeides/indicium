mod and;
mod keyword;
mod live;
mod or;

// -----------------------------------------------------------------------------

use crate::simple::{SearchIndex, SearchType};
use std::cmp::Ord;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<'a, K: 'a + Hash + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// The `search` function will return keys as the search results. Each
    /// resulting key can then be used to retrieve the full record from its
    /// collection. Search keywords must be an exact match.
    ///
    /// Search only supports exact keyword matches and does not use fuzzy
    /// matching. Consider providing the `autocomplete` feature to your users as
    /// an ergonomic alternative to fuzzy matching.
    ///
    /// Search behaviour can be changed by setting the [`SearchType`] in the
    /// `SearchIndex`. See also: [`SearchIndexBuilder`] and
    /// [`SearchIndex::new()`].
    ///
    /// [`SearchType`]: enum.SearchType.html
    /// [`SearchIndexBuilder`]: struct.SearchIndexBuilder.html
    /// [`SearchIndex::new()`]: struct.SearchIndex.html#method.new
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
    /// let search_results = search_index.search("last Wessex");
    /// assert_eq!(search_results, vec![&1]);
    /// ```

    #[tracing::instrument(level = "trace", name = "search", skip(self))]
    pub fn search(&'a self, string: &'a str) -> Vec<&'a K> {

        let search_results: Vec<&'a K> = match self.search_type {
            SearchType::And =>
                self.search_and(&self.maximum_search_results, string),
            SearchType::Keyword =>
                self.search_keyword(&self.maximum_search_results, string),
            SearchType::Live =>
                self.search_live(&self.maximum_search_results, string)
                    .into_iter()
                    .collect(),
            SearchType::Or =>
                self.search_or(&self.maximum_search_results, string),
        }; // match

        // For debug builds:
        #[cfg(debug_assertions)]
        tracing::debug!(
            "{} search results for \"{}\".",
            search_results.len(),
            string,
        ); // debug!

        search_results

    } // fn

    // -------------------------------------------------------------------------
    //
    /// This search method allows the caller to define a `SearchType`
    /// parameter, effectively overriding the index settings. See [`SearchType`]
    /// for more information on the different search types.
    ///
    /// The `search` function will return keys as the search results. Each
    /// resulting key can then be used to retrieve the full record from its
    /// collection. Search keywords must be an exact match.
    ///
    /// Search only supports exact keyword matches and does not use fuzzy
    /// matching. Consider providing the `autocomplete` feature to your users as
    /// an ergonomic alternative to fuzzy matching.
    ///
    /// [`SearchType`]: enum.SearchType.html
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
    /// let search_results = search_index.search_type(&SearchType::And, "Conqueror third");
    /// assert_eq!(search_results, vec![&3]);
    /// ```

    #[tracing::instrument(level = "trace", name = "search", skip(self))]
    pub fn search_type(
        &'a self,
        search_type: &SearchType,
        string: &'a str,
    ) -> Vec<&'a K> {

        let search_results: Vec<&'a K> = match search_type {
            SearchType::And =>
                self.search_and(&self.maximum_search_results, string),
            SearchType::Keyword =>
                self.search_keyword(&self.maximum_search_results, string),
            SearchType::Live =>
                self.search_live(&self.maximum_search_results, string)
                    .into_iter()
                    .collect(),
            SearchType::Or =>
                self.search_or(&self.maximum_search_results, string),
        }; // match

        // For debug builds:
        #[cfg(debug_assertions)]
        tracing::debug!(
            "{} search results for \"{}\".",
            search_results.len(),
            string,
        ); // debug!

        search_results

    } // fn

    // -------------------------------------------------------------------------
    //
    /// This search method allows the caller to define a `SearchType` and the
    /// maximum number of search results to return. These parameters override
    /// the index settings. See [`SearchType`] for more information on the
    /// different search types.
    ///
    /// The `search` function will return keys as the search results. Each
    /// resulting key can then be used to retrieve the full record from its
    /// collection. Search keywords must be an exact match.
    ///
    /// Search only supports exact keyword matches and does not use fuzzy
    /// matching. Consider providing the `autocomplete` feature to your users as
    /// an ergonomic alternative to fuzzy matching.
    ///
    /// [`SearchType`]: enum.SearchType.html
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
    /// let search_results = search_index.search_with(
    ///     &SearchType::And,
    ///     &20,
    ///     "Conqueror third"
    /// );
    ///
    /// assert_eq!(search_results, vec![&3]);
    /// ```

    #[tracing::instrument(level = "trace", name = "search", skip(self))]
    pub fn search_with(
        &'a self,
        search_type: &SearchType,
        maximum_search_results: &usize,
        string: &'a str,
    ) -> Vec<&'a K> {

        let search_results: Vec<&'a K> = match search_type {
            SearchType::And =>
                self.search_and(maximum_search_results, string),
            SearchType::Keyword =>
                self.search_keyword(maximum_search_results, string),
            SearchType::Live =>
                self.search_live(maximum_search_results, string)
                    .into_iter()
                    .collect(),
            SearchType::Or =>
                self.search_or(maximum_search_results, string),
        }; // match

        // For debug builds:
        #[cfg(debug_assertions)]
        tracing::debug!(
            "{} search results for \"{}\".",
            search_results.len(),
            string,
        ); // debug!

        search_results

    } // fn

} // impl