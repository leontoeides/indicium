#![allow(unused_mut)]

use crate::simple::internal::string_keywords::SplitContext;
use kstring::KString;
use std::{collections::BTreeSet, hash::Hash};

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> crate::simple::SearchIndex<K> {
    /// This search function will return keys as the search results. Each
    /// resulting key can then be used to retrieve the full record from its
    /// collection. _This search method accepts multiple keywords in the search
    /// string._ Search keywords must be an exact match.
    ///
    /// `Live` search allows for "search as you type." It is a hybridization
    /// of `autocomplete` and `search`. This method will effectively search
    /// all of the autocompletion options and return the search results to the
    /// caller.
    ///
    /// With this search type, the logical conjuction for multiple keywords is
    /// `And`. For example, a search of `this that` will only return records
    /// containing keywords both `this` and `that`. In other words, _all_
    /// keywords must be present in a record for it to be returned as a result.
    ///
    /// Search only supports exact keyword matches. For `Live` searches, fuzzy
    /// matching is only applied to the last keyword. Also, consider providing
    /// the `autocomplete` feature to your users for a better experience.
    ///
    /// Basic usage:
    ///
    /// ```ignore
    /// # use indicium::simple::{
    /// #   AutocompleteType,
    /// #   Indexable,
    /// #   SearchIndex,
    /// #   SearchType
    /// # };
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
    /// let search_results = search_index
    ///     .search_live(&20, "Norman C")
    ///     .into_iter()
    ///     .collect::<Vec<&usize>>();
    ///
    /// assert_eq!(search_results, vec![&2]);
    /// ```
    #[tracing::instrument(level = "trace", name = "live search", skip(self))]
    pub(crate) fn search_live(
        &self,
        maximum_search_results: &usize,
        string: &str
    ) -> BTreeSet<&K> {
        // Split search `String` into keywords according to the `SearchIndex`
        // settings. Force "use entire string as a keyword" option off:
        let mut keywords: Vec<KString> =
            self.string_keywords(string, &SplitContext::Searching);

        // For debug builds:
        #[cfg(debug_assertions)]
        tracing::debug!("searching: {:?}", keywords);

        // Pop the last keyword off the list - the keyword that we'll be
        // autocompleting:
        keywords.pop().map_or_else(BTreeSet::new, |last_keyword| {
            if keywords.is_empty() {
                let mut search_results: BTreeSet<&K> = self
                    .b_tree_map
                    // Get matching keywords starting with (partial) keyword
                    // string:
                    .range(last_keyword.clone()..)
                    // We did not specify an end bound for our `range`
                    // function (see above.) `range` will return _every_
                    // keyword greater than the supplied keyword. The below
                    // `take_while` will effectively break iteration when we
                    // reach a keyword that does not start with our supplied
                    // (partial) keyword.
                    .take_while(|(keyword, _keys)| keyword.starts_with(&*last_keyword))
                    // Only return `maximum_search_results` number of keys:
                    .take(*maximum_search_results)
                    // We're not interested in the `keyword` since we're
                    // returning `&K` keys. Return only `&K` from the tuple.
                    // Flatten the `BTreeSet<K>` from each autocomplete
                    // keyword option into our collection:
                    .flat_map(|(_keyword, keys)| keys)
                    // Collect all keyword search results into a `BTreeSet`:
                    .collect();

                // If `rapidfuzz` fuzzy matching enabled, examine the search
                // results before returning them:
                #[cfg(feature = "rapidfuzz")]
                if search_results.is_empty() {
                    // No search results were found for the user's last
                    // (partial) keyword. Attempt to use fuzzy string
                    // search to find other options:
                    search_results = self
                        .rapidfuzz_autocomplete_context(&search_results, &last_keyword)
                        // `rapidfuzz_autocomplete` returns both the keyword
                        // and keys. We're searching for the last (partial)
                        // keyword, so discard the keywords. Flatten the
                        // `BTreeSet<K>` from each search result into our
                        // collection:
                        .flat_map(|(_keyword, keys)| keys)
                        // Only return `maximum_search_results` number of
                        // keys:
                        .take(*maximum_search_results)
                        // Collect all keyword autocompletions into a
                        // `BTreeSet`:
                        .collect();
                } // if

                // If `strsim` fuzzy matching enabled, examine the search
                // results before returning them:
                #[cfg(feature = "strsim")]
                if search_results.is_empty() {
                    // No search results were found for the user's last
                    // (partial) keyword. Attempt to use fuzzy string
                    // search to find other options:
                    search_results = self
                        .strsim_autocomplete_context(&search_results, &last_keyword)
                        .into_iter()
                        // `strsim_autocomplete` returns both the keyword
                        // and keys. We're searching for the last (partial)
                        // keyword, so discard the keywords. Flatten the
                        // `BTreeSet<K>` from each search result into our
                        // collection:
                        .flat_map(|(_keyword, keys)| keys)
                        // Only return `maximum_search_results` number of
                        // keys:
                        .take(*maximum_search_results)
                        // Collect all keyword autocompletions into a
                        // `BTreeSet`:
                        .collect();
                } // if

                // If `eddie` fuzzy matching enabled, examine the search
                // results before returning them:
                #[cfg(feature = "eddie")]
                if search_results.is_empty() {
                    // No search results were found for the user's last
                    // (partial) keyword. Attempt to use fuzzy string
                    // search to find other options:
                    search_results = self
                        .eddie_autocomplete_context(&search_results, &last_keyword)
                        .into_iter()
                        // `strsim_autocomplete` returns both the keyword
                        // and keys. We're searching for the last (partial)
                        // keyword, so discard the keywords. Flatten the
                        // `BTreeSet<K>` from each search result into our
                        // collection:
                        .flat_map(|(_keyword, keys)| keys)
                        // Only return `maximum_search_results` number of
                        // keys:
                        .take(*maximum_search_results)
                        // Collect all keyword autocompletions into a
                        // `BTreeSet`:
                        .collect();
                } // if

                // Return search results to caller:
                search_results
            } else {
                // Perform `And` search for entire string, excluding the
                // last (partial) keyword:
                let search_results: BTreeSet<&K> = self
                    .internal_and_search(keywords.as_slice());

                // Get keys for the last (partial) keyword:
                let mut last_results: BTreeSet<&K> = self.b_tree_map
                    // Get matching keywords starting with (partial) keyword
                    // string:
                    .range(last_keyword.clone()..)
                    // We did not specify an end bound for our `range`
                    // function (see above.) `range` will return _every_
                    // keyword greater than the supplied keyword. The below
                    // `take_while` will effectively break iteration when we
                    // reach a keyword that does not start with our supplied
                    // (partial) keyword.
                    .take_while(|(keyword, _keys)| keyword.starts_with(&*last_keyword))
                    // Only keep this autocompletion if hasn't already been
                    // used as a keyword:
                    .filter(|(keyword, _keys)| !keywords.contains(keyword))
                    // We're not interested in the `keyword` since we're
                    // returning `&K` keys. Return only `&K` from the tuple.
                    // Flatten the `BTreeSet<K>` from each autocomplete
                    // keyword option into individual `K` keys:
                    .flat_map(|(_key, value)| value)
                    // Intersect the key results from the autocomplete
                    // options (produced from this iterator) with the search
                    // results produced above:
                    .filter(|key| search_results.contains(key))
                    // Only return `maximum_search_results` number of keys:
                    .take(*maximum_search_results)
                    // Collect all keyword autocompletions into a
                    // `BTreetSet`:
                    .collect();

                // If fuzzy string searching enabled, examine the search
                // results before returning them:
                #[cfg(feature = "rapidfuzz")]
                if last_results.is_empty() {
                    // No search results were found for the user's last
                    // (partial) keyword. Attempt to use fuzzy string
                    // search to find other options:
                    last_results = self
                        .rapidfuzz_autocomplete_context(&search_results, &last_keyword)
                        // Only keep this result if hasn't already been used
                        // as a keyword:
                        .filter(|(keyword, _keys)| !keywords.contains(keyword))
                        // Intersect the key results from the autocomplete
                        // options (produced from this iterator) with the
                        // search results produced at the top:
                        .flat_map(|(_keyword, keys)|
                            keys.iter().filter(|key| search_results.contains(key))
                        ) // map
                        // Only return `maximum_search_results` number of keys:
                        .take(*maximum_search_results)
                        // Collect all keyword autocompletions into a
                        // `BTreeSet`:
                        .collect();
                } // if

                // If fuzzy string searching enabled, examine the search
                // results before returning them:
                #[cfg(feature = "strsim")]
                if last_results.is_empty() {
                    // No search results were found for the user's last
                    // (partial) keyword. Attempt to use fuzzy string
                    // search to find other options:
                    last_results = self
                        .strsim_autocomplete_context(&search_results, &last_keyword)
                        .into_iter()
                        // Only keep this result if hasn't already been used
                        // as a keyword:
                        .filter(|(keyword, _keys)| !keywords.contains(keyword))
                        // Intersect the key results from the autocomplete
                        // options (produced from this iterator) with the
                        // search results produced at the top:
                        .flat_map(|(_keyword, keys)|
                            keys.iter().filter(|key| search_results.contains(key))
                        ) // map
                        // Only return `maximum_search_results` number of keys:
                        .take(*maximum_search_results)
                        // Collect all keyword autocompletions into a
                        // `BTreeSet`:
                        .collect();
                } // if

                // If fuzzy string searching enabled, examine the search
                // results before returning them:
                #[cfg(feature = "eddie")]
                if last_results.is_empty() {
                    // No search results were found for the user's last
                    // (partial) keyword. Attempt to use fuzzy string
                    // search to find other options:
                    last_results = self
                        .eddie_autocomplete_context(&search_results, &last_keyword)
                        .into_iter()
                        // Only keep this result if hasn't already been used
                        // as a keyword:
                        .filter(|(keyword, _keys)| !keywords.contains(keyword))
                        // Intersect the key results from the autocomplete
                        // options (produced from this iterator) with the
                        // search results produced at the top:
                        .flat_map(|(_keyword, keys)|
                            keys.iter().filter(|key| search_results.contains(key))
                        ) // map
                        // Only return `maximum_search_results` number of keys:
                        .take(*maximum_search_results)
                        // Collect all keyword autocompletions into a
                        // `BTreeSet`:
                        .collect();
                } // if

                // Return search results to caller:
                last_results
            } // if
        }) // if
    } // fn
} // impl
