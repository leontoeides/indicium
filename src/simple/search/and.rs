use crate::simple::internal::string_keywords::SplitContext;
use std::{collections::BTreeSet, hash::Hash};

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> crate::simple::search_index::SearchIndex<K> {
    /// This search function will return keys as the search results. Each
    /// resulting key can then be used to retrieve the full record from its
    /// collection. _This search method accepts multiple keywords in the search
    /// string._ Search keywords must be an exact match.
    ///
    /// With this search type, the logical conjuction for multiple keywords is
    /// `And`. For example, a search of `this that` will only return records
    /// containing keywords both `this` and `that`. In other words, _all_
    /// keywords must be present in a record for it to be returned as a result.
    ///
    /// For this search, the results are returned in lexographic order. This
    /// conjuction uses less CPU resources than `Or`.
    ///
    /// The `And` search feels more like "use my keywords to filter out the
    /// records I don't want." It's also a better choice for large collections
    /// because it uses less CPU resouces than `Or`.
    ///
    /// Search only supports exact keyword matches and does not use fuzzy
    /// matching. Consider providing the `autocomplete` feature to your users as
    /// an ergonomic alternative to fuzzy matching.
    ///
    /// Basic usage:
    ///
    /// ```ignore
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
    /// let search_results = search_index.and_search(&20, "Conqueror third");
    /// assert_eq!(search_results, vec![&3]);
    /// ```
    #[tracing::instrument(level = "trace", name = "and search", skip(self))]
    pub(crate) fn and_search(
        &self,
        maximum_search_results: &usize,
        string: &str
    ) -> Vec<&K> {
        // Split search `String` into keywords (according to the `SearchIndex`
        // settings). `string_keywords` will **not** allow "use entire string as
        // a keyword," even if enabled in user settings:
        let keywords: Vec<kstring::KString> =
            self.string_keywords(string, &SplitContext::Searching);

        // For debug builds:
        #[cfg(debug_assertions)]
        tracing::debug!("searching: {:?}", keywords);

        // This `BTreeSet` is used to contain the search results:
        let mut search_results = BTreeSet::<&K>::new();

        // Get each keyword from our `BTreeMap`, and intersect the resulting
        // keys with our current keys:
        for keyword in keywords {
            // Attempt to retrieve keyword from search index. If keyword
            // found, intersect keyword records with search results records.
            // If keyword not found, empty search results:
            match self.b_tree_map.get(&keyword) {
                // Keyword found. Update `search_results` with product of an
                // intersection with this keyword's records:
                Some(keyword_results) => search_results =
                    // Check if `search_results` is already populated:
                    if search_results.is_empty() {
                        self
                            .internal_keyword_search(&keyword)
                            .collect()
                    } else {
                        search_results
                            // Iterate over each search result record:
                            .into_iter()
                            // Intersect the search result record with the
                            // keyword results. If the search result record
                            // doesn't exist in this keyword's results, filter
                            // it out:
                            .filter(|key| keyword_results.contains(key))
                            // And collect each key into a `BTreeSet` that will
                            // become the new `search_results`:
                            .collect()
                    }, // Some

                // Any keyword that returns no results will short-circuit the
                // search results into an empty set.
                //
                // Note: the previous setup involved returning an
                // `BTreeSet::new`. This setup looks strange, but involves no
                // allocations.
                None => {
                    search_results.clear();
                    return search_results
                        .into_iter()
                        .take(*maximum_search_results)
                        .collect()
                }, // None
            } // match
        } // for_each

        // Return search results:
        search_results
            .into_iter()
            .take(*maximum_search_results)
            .collect()
    } // fn
} // impl
