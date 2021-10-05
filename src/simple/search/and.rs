use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;
use std::collections::HashSet;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
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
    /// let search_results = search_index.search_and("Conqueror third");
    /// assert_eq!(search_results, vec![&3]);
    /// ```

    #[tracing::instrument(level = "trace", name = "And Search", skip(self))]
    pub(crate) fn search_and(&self, string: &str) -> Vec<&K> {

        // Split search `String` into keywords (according to the `SearchIndex`
        // settings). `string_keywords` will **not** allow "use entire string as
        // a keyword," even if enabled in user settings:
        let keywords: Vec<String> = self.string_keywords(string, false);

        // This `BTreeSet` is used to contain the search results:
        let mut search_results: Option<HashSet<&K>> = None;

        // Get each keyword from our `BTreeMap`, and intersect the resulting
        // keys with our current keys:
        keywords
            // Iterate over the keywords supplied in the search string:
            .iter()
            // For each keyword in the search string:
            .for_each(|keyword| {

                // Search for keyword in our `BTreeMap`:
                let keyword_results = self.internal_keyword_search(keyword);

                // Update `search_results` with product of `intersection`:
                search_results = Some(
                    // Check if `search_results` is already populated:
                    match &search_results {
                        // If `search_results` is is not empty...
                        Some(search_results) => {
                            // ...intersect the current keyword's search results
                            // with the master search results:
                            keyword_results
                                // Intersection will only keep the values that
                                // are both in `search_results` and
                                // `keyword_results`.
                                .intersection(search_results)
                                // The `intersection` function will return an
                                // `Intersection` type that we can iterate over:
                                .into_iter()
                                // Copy each key from the `Intersection`
                                // iterator or we'll get a doubly-referenced
                                // `&&K` key:
                                .cloned()
                                // And collect each key into a `HashSet` that
                                // will become the new `search_results`.
                                .collect()
                        }, // Some
                        // If `search_results` is empty, initialize it with the
                        // first keyword's full search results:
                        None => keyword_results,
                    } // match
                ); // Some

            }); // for_each

        // Return search results:
        match search_results {
            // If `search_results` is is not empty, convert the `HashSet` to a
            // `Vec` for caller while observing `maximum_search_results`:
            Some(search_results) => search_results
                .iter()
                .take(self.maximum_search_results)
                .cloned()
                .collect(),
            // If `search_results` is empty, return an empty `Vec`:
            None => Vec::new(),
        } // match

    } // fn

} // impl