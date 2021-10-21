use crate::simple::internal::string_keywords::SplitContext;
use crate::simple::SearchIndex;
use std::cmp::Ord;
use std::collections::BTreeSet;
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
    /// Search only supports exact keyword matches and does not use fuzzy
    /// matching. Consider providing the `autocomplete` feature to your users as
    /// an ergonomic alternative to fuzzy matching.
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
    ///     .iter()
    ///     .cloned()
    ///     .collect::<Vec<&usize>>();
    ///
    /// assert_eq!(search_results, vec![&2]);
    /// ```

    #[tracing::instrument(level = "trace", name = "Live Search", skip(self))]
    pub(crate) fn search_live(
        &self,
        maximum_search_results: &usize,
        string: &str,
    ) -> BTreeSet<&K> {

        // Split search `String` into keywords according to the `SearchIndex`
        // settings. Force "use entire string as a keyword" option off:
        let mut keywords: Vec<String> = self.string_keywords(
            string,
            SplitContext::Searching,
        );

        // For debug builds:
        #[cfg(debug_assertions)]
        tracing::trace!("Searching: {:?}", keywords);

        // Pop the last keyword off the list - the keyword that we'll be
        // autocompleting:
        if let Some(last_keyword) = keywords.pop() {

            // Perform `And` search for entire string, excluding the last
            // (partial) keyword:
            let search_results: BTreeSet<&K> =
                self.internal_search_and(keywords.as_slice())
                    // Iterate over each key:
                    .iter()
                    // Copy each `&K` key reference from the iterator or we'll
                    // get a doubly-referenced `&&K` key:
                    .cloned()
                    // Collect serach results into our `BTreeSet`:
                    .collect();

            // Get keys for the last (partial) keyword:
            let autocomplete_options: BTreeSet<&K> = self.b_tree_map
                // Get matching keywords starting with (partial) keyword string:
                .range(last_keyword.to_owned()..)
                // We did not specify an end bound for our `range` function (see
                // above.) `range` will return _every_ keyword greater than the
                // supplied keyword. The below `take_while` will effectively
                // break iteration when we reach a keyword that does not start
                // with our supplied (partial) keyword.
                .take_while(|(key, _value)| key.starts_with(&last_keyword))
                // Only return `maximum_keys_per_keyword` number of keywords:
                .take(self.maximum_keys_per_keyword)
                // We're not interested in the `keyword` since we're returning
                // `&K` keys. Return only `&K` from the tuple:
                .map(|(_keyword, keys)| keys)
                // Flatten the `BTreeSet<K>` from each autocomplete keyword
                // option into our collection:
                .flatten()
                // Collect all keyword autocompletions into a `Vec`:
                .collect();

            // For debug builds:
            #[cfg(debug_assertions)]
            if autocomplete_options.len() >= self.maximum_keys_per_keyword {
                tracing::warn!(
                    "Internal table limit of {} keywords has been exceeded for live search autocompletion. \
                    Data has been dropped. \
                    This will impact accuracy of results. \
                    For this data set, consider using a more comprehensive search solution like MeiliSearch.",
                    self.maximum_keys_per_keyword
                ); // warn!
            } // if

            // How we combine `search_results` and `autocomplete_options`
            // together depends on how many keywords there are in the search
            // string. Strings that have only a single keyword, and strings
            // that have multiple keywords must be handled differently:

            match keywords.len() {

                // Consider this example search string: `t`.
                //
                // Depending on the data-set, autocomplete options `trouble` and
                // `tribble` may be given.
                //
                // There are no previous keywords to intersect with, just the
                // letter `t`. If we attempt to intersect with an empty
                // `search_results`, no keys will ever be returned. So we must
                // handle this scenario differently. We will return the keys for
                // these autocomplete options without further processing:

                0 => autocomplete_options
                    // Iterate over all possible keys for the last (partial)
                    // keyword:
                    .iter()
                    // Only return `maximum_search_results` number of keys:
                    .take(self.maximum_search_results)
                    // Copy each key from the `Intersection` iterator or we'll
                    // get a doubly-referenced `&&K` key:
                    .cloned()
                    // And collect each key into a `BTreeSet` that will become
                    // the new `search_results`.
                    .collect(),

                // Consider this example search string: `Shatner t`.
                //
                // Depending on the data-set, autocomplete options for `t` might
                // be `trouble` and `tribble`. However, in this example there is
                // a previous keyword: `Shatner`.
                //
                // This match arm will intersect the results from each
                // autocomplete option with `Shatner`. For both `trouble` and
                // `tribble` autocomplete options, only keys that also exist for
                // `Shatner` will be returned. All resulting keys for both
                // autocomplete options will be flattened together:

                _ => search_results
                    // Intersection will only keep the values that are both in
                    // `search_results` and `autocomplete_options`.
                    .intersection(&autocomplete_options)
                    // The `intersection` function will return an `Intersection`
                    // type that we can iterate over:
                    .into_iter()
                    // Only return `maximum_search_results` number of keys:
                    .take(self.maximum_search_results)
                    // Copy each key from the `Intersection` iterator or we'll
                    // get a doubly-referenced `&&K` key:
                    .cloned()
                    // And collect each key into a `BTreeSet` that will become
                    // the new `search_results`.
                    .collect(),

            } // match

        } else {

            // The search string did not have a last keyword to autocomplete (or
            // any keywords to search for.) Return an empty `BTreeSet`:
            BTreeSet::new()

        } // if

    } // fn

} // impl