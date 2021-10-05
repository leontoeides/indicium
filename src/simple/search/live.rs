use crate::simple::SearchIndex;
use std::cmp::Ord;
use std::collections::BTreeSet;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //

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
    /// let search_results = search_index
    ///     .search_live("Norman C")
    ///     .iter()
    ///     .cloned()
    ///     .collect::<Vec<&usize>>();
    ///
    /// assert_eq!(search_results, vec![&2]);
    /// ```

    #[tracing::instrument(level = "trace", name = "Live Search", skip(self))]
    pub(crate) fn search_live(&self, string: &str) -> BTreeSet<&K> {

        // Split search `String` into keywords according to the `SearchIndex`
        // settings. Force "use entire string as a keyword" option off:
        let mut keywords: Vec<String> = self.string_keywords(string, false);

        // Pop the last keyword off the list - the keyword that we'll be
        // autocompleting:
        if let Some(last_keyword) = keywords.pop() {

            // Perform `And` search for entire string without the last keyword:
            let search_results: BTreeSet<&K> =
                self.internal_search_and(keywords.as_slice())
                    .iter()
                    .cloned()
                    .collect();

            // Get all autocompletions for the last keyword and their keys:
            let autocompletions: BTreeSet<&BTreeSet<K>> =
                self.internal_autocomplete_keyword(&last_keyword)
                    .iter()
                    .map(|(_keyword, keys)| *keys)
                    .collect();

            // How we combine `search_results` and `autocompletions` together
            // depends on how many keywords there are in the search string:
            match keywords.len() {

                0 => autocompletions
                    .iter()
                    // Only return `maximum_search_results` number of keys:
                    .take(self.maximum_search_results)
                    .cloned()
                    .flatten()
                    .collect(),

                _ => autocompletions
                    .iter()
                    .map(|autocompletion_keys|
                        autocompletion_keys
                            .iter()
                            .filter(|autocompletion_key| search_results.contains(autocompletion_key))
                            .collect::<BTreeSet<&K>>()
                    )
                    .flatten()
                    // Only return `maximum_search_results` number of keys:
                    .take(self.maximum_search_results)
                    .collect(),
            }

        } else {

            // The search string did not have a last keyword to autocomplete (or
            // any keywords to search for.) Return an empty `BTreeSet`:
            BTreeSet::new()

        } // if

    } // fn

} // impl