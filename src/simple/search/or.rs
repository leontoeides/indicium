use crate::simple::internal::string_keywords::SplitContext;
use crate::simple::search_index::SearchIndex;
use kstring::KString;
use std::{cmp::Ord, collections::BTreeMap, hash::Hash};

// -----------------------------------------------------------------------------

impl<'a, K: 'a + Hash + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// This search function will return keys as the search results. Each
    /// resulting key can then be used to retrieve the full record from its
    /// collection. _This search method accepts multiple keywords in the search
    /// string._ Search keywords must be an exact match.
    ///
    /// With this search type, the logical conjuction for multiple keywords is
    /// `Or`. For example, a search of `this that` will return records
    /// containing keywords `this` or `that`. In other words, _any_ keyword can
    /// be present in a record for it to be returned as a result.
    ///
    /// For this search, the results are returned in order of descending
    /// relevance. Records containing both keywords `this` and `that` will be
    /// the top results. This conjuction uses more CPU resources than `And`
    /// because the keyword hits must be tallied and sorted.
    ///
    /// If your collection contains less than 10,000 records, `Or` might be a
    /// good place to start. To me, `Or` effectively feels like "using these
    /// keywords, find a record I might want" which works well if there aren't
    /// too many records. It's also worth noting that this conjuction uses more
    /// CPU resources because the results must be tallied and sorted in order of
    /// relevance.
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
    /// let search_results = search_index.search_or(&20, "last England");
    /// assert_eq!(search_results, vec![&0, &1, &2]);
    /// ```

    #[tracing::instrument(level = "trace", name = "or search", skip(self))]
    pub(crate) fn search_or(
        &'a self,
        maximum_search_results: &usize,
        string: &'a str,
    ) -> Vec<&'a K> {

        // Split search `String` into keywords (according to the `SearchIndex`
        // settings). `string_keywords` will allow "use entire string as a
        // keyword" if enabled in user settings:
        let keywords: Vec<KString> = self.string_keywords(
            string,
            SplitContext::Searching,
        );

        // For debug builds:
        #[cfg(debug_assertions)]
        tracing::debug!("searching: {:?}", keywords);

        // This `BTreeMap` is used to count the number of hits for each
        // resulting key. This is so we can return search results in order of
        // relevance:
        let mut search_results: BTreeMap<&K, usize> = BTreeMap::new();

        // Get each keyword from our search index, record the resulting keys in
        // a our `BTreeMap`, and track the hit-count for each key:
        keywords
            // Iterate over the keywords supplied in the search string:
            .into_iter()
            // For each keyword in the search string:
            .for_each(|keyword| {
                // Search for keyword in our `BTreeMap`:
                self.internal_keyword_search(&keyword)
                    // Iterate over the resulting keys (if any):
                    .into_iter()
                    // For each resulting key from the keyword search:
                    .for_each(|key| match search_results.get_mut(key) {
                        // Add "hit" to counter for an already existing key:
                        Some(result_entry) => { *result_entry += 1 },
                        // No record for this key, initialize to one hit:
                        None => { search_results.insert(key, 1); },
                    }) // for_each
            }); // for_each

        // At this point, we have a list of resulting keys in a `BTreeMap`. The
        // hash map value holds the number of times each key has been returned
        // in the above keywords search.
        //
        // We want to sort these keys by descending hit-count. First, we must
        // convert it to a `Vec` so this can be done:

        let mut search_results: Vec<(&K, usize)> = search_results
            // Iterate over keys in the hash map:
            .into_iter()
            // Collect the tuple elements into a `Vec`:
            .collect();

        // Sort the tuple elements by hit-count descending:
        search_results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Return the search results to the user:
        search_results
            // Iterate over the tuple elements:
            .into_iter()
            // Only return `maximum_search_results` number of keys:
            .take(*maximum_search_results)
            // Remove the hit-count from the tuple, returning only the key:
            .map(|(key, _value)| key)
            // Collect the keys into a `Vec`:
            .collect()

    } // fn

} // impl