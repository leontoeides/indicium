#![allow(unused_mut)]

use crate::simple::internal::string_keywords::SplitContext;
use crate::simple::search_index::SearchIndex;
use std::{cmp::Ord, hash::Hash};

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Returns matching autocompleted keywords for the provided search string.
    /// _This search method accepts multiple keywords in the search string._
    /// The last partial search keyword must be an exact match.
    ///
    /// The search string may contain multiple keywords and the last (partial)
    /// keyword will be autocompleted. The last keyword in the search string
    /// will be autocompleted from all available keywords in the search index.
    /// If your data-set is very large or has repetitive keywords (see also: the
    /// [`profile`] utility method), this is the recommended autocomplete type.
    /// Results are returned in lexographic order.
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
    /// let autocomplete_options = search_index.autocomplete_global(
    ///     &5,
    ///     "1100 e"
    /// );
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

    #[tracing::instrument(level = "trace", name = "global autocomplete", skip(self))]
    pub(crate) fn autocomplete_global(
        &self,
        maximum_autocomplete_options: &usize,
        string: &str,
    ) -> Vec<String> {

        // Split search `String` into keywords according to the `SearchIndex`
        // settings. Force "use entire string as a keyword" option off:
        let mut keywords: Vec<String> = self.string_keywords(
            string,
            SplitContext::Searching,
        );

        // For debug builds:
        #[cfg(debug_assertions)]
        tracing::debug!("autocompleting: {:?}", keywords);

        // Pop the last keyword off the list. It's the keyword that we'll be
        // autocompleting:
        if let Some(last_keyword) = keywords.pop() {

            // Autocomplete the last keyword:
            let mut autocompletions: Vec<&String> = self.b_tree_map
                // Get matching keywords starting with (partial) keyword string:
                .range(last_keyword.to_string()..)
                // `range` returns a key-value pair. We're autocompleting the
                // key (keyword), so discard the value (record key):
                .map(|(key, _value)| key)
                // We did not specify an end bound for our `range` function (see
                // above.) `range` will return _every_ keyword greater than the
                // supplied keyword. The below `take_while` will effectively
                // break iteration when we reach a keyword that does not start
                // with our supplied (partial) keyword.
                .take_while(|autocompletion| autocompletion.starts_with(&last_keyword))
                // If the index's keyword matches the user's keyword, don't
                // return it as a result. For example, if the user's keyword was
                // "new" (as in New York), do not return "new" as an
                // auto-completed keyword:
                // .filter(|autocompletion| *autocompletion != &last_keyword)
                // Only keep this autocompletion if hasn't already been used as
                // a keyword:
                .filter(|autocompletion| !keywords.contains(autocompletion))
                // If the index's keyword matches the user's keyword, don't
                // return it as a result. For example, if the user's keyword was
                // "new" (as in New York), do not return "new" as an
                // auto-completed keyword:
                // .filter(|autocompletion| *autocompletion != &keyword)
                // Only return `maximum_autocomplete_options` number of
                // keywords:
                .take(*maximum_autocomplete_options)
                // Collect all keyword autocompletions into a `Vec`:
                .collect();

            // If fuzzy string searching enabled, examine the resulting
            // auto-complete options before using them:
            #[cfg(feature = "fuzzy")]
            if autocompletions.is_empty() {
                // No autocomplete options were found for the user's last
                // (partial) keyword. Attempt to use fuzzy string search to find
                // other autocomplete options:
                autocompletions = self.strsim_global_autocomplete(&last_keyword)
                    .into_iter()
                    // Only keep this autocompletion if hasn't already been used
                    // as a keyword:
                    .filter(|(keyword, _keys)| !keywords.contains(keyword))
                    // Only return `maximum_autocomplete_options` number of
                    // keywords:
                    .take(*maximum_autocomplete_options)
                    // `strsim_autocomplete` returns both the keyword and keys.
                    // We're autocompleting the last (partial) keyword, so
                    // discard the keys:
                    .map(|(keyword, _keys)| keyword)
                    // Collect all keyword autocompletions into a `Vec`:
                    .collect()
            } // if

            // Push a blank placeholder onto the end of the keyword list. We
            // will be putting our autocompletions for the last keyword into
            // this spot:
            keywords.push("".to_string());

            // Build autocompleted search strings from the autocompletions
            // derived from the last keyword:
            autocompletions
                // Iterate over each autocompleted last keyword:
                .iter()
                // Use the prepended `keywords` and autocompleted last keyword
                // to build an autocompleted search string:
                .map(|autocompletion| {
                    // Remove previous autocompleted last keyword from list:
                    keywords.pop();
                    // Add current autocompleted last keyword to end of list:
                    keywords.push(autocompletion.to_string());
                    // Join all keywords together into a single `String` using a
                    // space delimiter:
                    keywords.join(" ").trim_end().to_owned()
                })
                // Collect all string autocompletions into a `Vec`:
                .collect()

        } else {

            // The search string did not have a last keyword to autocomplete.
            // Return an empty `Vec`:
            Vec::new()

        } // if

    } // fn

} // impl