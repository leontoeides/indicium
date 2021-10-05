use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;
use std::collections::{BTreeSet, HashSet};
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Ord + Hash> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Returns matching autocompleted keywords for the provided search string.
    /// _This search method accepts multiple keywords in the search string._
    /// The last partial search keyword must be an exact match.
    ///
    /// The search string may contain multiple keywords and the last (partial)
    /// keyword will be autocompleted. The last keyword in the search string
    /// will be autocompleted from all available keywords in the search index.
    /// If your data-set is very large or has repetitive keywords, this is the
    /// recommended autocomplete type. Results are returned in lexographic
    /// order.
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
    /// let autocomplete_options = search_index.autocomplete_global("1100 e");
    ///
    /// assert_eq!(
    ///     autocomplete_options,
    ///     vec![
    ///         "1100 edgar".to_string(),
    ///         "1100 edgar ætheling".to_string(),
    ///         "1100 england".to_string()
    ///     ]
    /// );
    /// ```

    #[tracing::instrument(level = "trace", name = "Global Autocomplete", skip(self))]
    pub fn live_global(&self, string: &str) -> BTreeSet<&K> {

        // Split search `String` into keywords according to the `SearchIndex`
        // settings. Force "use entire string as a keyword" option off:
        let mut keywords: Vec<String> = self.string_keywords(string, false);

        // Pop the last keyword off the list - the keyword that we'll be
        // autocompleting:
        if let Some(last_keyword) = keywords.pop() {

            // Perform `And` search for entire string without the last keyword:
            let search_results: HashSet<&K> =
                self.internal_search_and(keywords.as_slice());

            // Get all autocompletions for the last keyword.
            let autocompletions: HashSet<(&String, &BTreeSet<K>)> =
                self.internal_autocomplete_keyword(&last_keyword);

            // Intersect the autocompletions for the last keyword with the
            // search results for the preceding keywords. This way, only
            // relevant autocompletions are returned:
            autocompletions
                .iter()
                // Only keep this autocompletion if it contains a key that the
                // search results contain:
                .filter(|(_keyword, keys)|
                    search_results.is_empty() || keys.iter().any(|key| search_results.contains(key))
                ) // filter
                // `internal_autocomplete_keyword` returns a key-value pair.
                // We're autocompleting the key, so discard the value:
                .map(|(_keyword, keys)| keys)
                // Copy each keyword from the iterator or we'll get a
                // doubly-referenced `&&String` keyword:
                .cloned()
                // Flatten the `BTreeSet` of keys for the keyword into our list:
                .flatten()
                // Only return `maximum_autocomplete_results` number of keywords:
                .take(self.maximum_autocomplete_results)
                // Collect all keyword autocompletions into a `BTreeSet`:
                .collect()

        } else {

            // The search string did not have a last keyword to autocomplete.
            // Return an empty `BTreeSet`:
            BTreeSet::new()

        } // if

    } // fn

} // impl