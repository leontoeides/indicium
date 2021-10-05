use crate::simple::SearchIndex;
use std::cmp::Ord;
use std::collections::BTreeSet;
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
    /// will be autocompleted by using the preceding keywords as a filter. This
    /// effectively provides contextual autocompletion. It is the heaviest and
    /// slowest autocompletion type but likely provides the best user
    /// experience. Results are returned in lexographic order.
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
    /// let autocomplete_options = search_index.autocomplete_context("E");
    ///
    /// assert_eq!(
    ///     autocomplete_options,
    ///     vec!["edgar".to_string(), "edgar ætheling".to_string(), "england".to_string()]
    /// );
    /// ```

    #[tracing::instrument(level = "trace", name = "Context Autocomplete", skip(self))]
    pub fn search_live(&self, string: &str) -> BTreeSet<&K> {

        // Split search `String` into keywords according to the `SearchIndex`
        // settings. Force "use entire string as a keyword" option off:
        let mut keywords: Vec<String> = self.string_keywords(string, false);

        // Pop the last keyword off the list - the keyword that we'll be
        // autocompleting:
        if let Some(last_keyword) = keywords.pop() {


            //println!("Search AND: {:?}", keywords);

            // Perform `And` search for entire string without the last keyword:
            let search_results: BTreeSet<&K> =
                self.internal_search_and(keywords.as_slice())
                    .iter()
                    .cloned()
                    .collect();

            //println!("Autocompletion: {:?}", last_keyword);


            // Get all autocompletions for the last keyword.
            let autocompletions: BTreeSet<&BTreeSet<K>> =
                self.internal_autocomplete_keyword(&last_keyword)
                    .iter()
                    .map(|(_keyword, keys)| *keys)
                    .collect();

            //println!("Search results {}, autocompletions {}", search_results.len(), autocompletions.len());


            match search_results.is_empty() {

                true => autocompletions
                    .iter()
                    .cloned()
                    .flatten()
                    .collect(),

                false => autocompletions
                    .iter()
                    .map(|autocompletion_keys|
                        autocompletion_keys
                            .iter()
                            .filter(|autocompletion_key| search_results.contains(autocompletion_key))
                            .collect::<BTreeSet<&K>>()
                    )
                    .flatten()
                    .collect(),
            }

        } else {

            // The search string did not have a last keyword to autocomplete.
            // Return an empty `BTreeSet`:
            BTreeSet::new()

        } // if

    } // fn

} // impl