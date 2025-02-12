use crate::simple::internal::fuzzers::Fuzzy;
use kstring::KStringBase;

// -----------------------------------------------------------------------------

impl<K: std::hash::Hash + Ord> crate::simple::search_index::SearchIndex<K> {
    // -------------------------------------------------------------------------
    //
    /// Returns matching autocompleted keywords for the provided search string.
    /// _This search method only accepts a single keyword as the search string._
    /// The partial search keyword must be an exact match.
    ///
    /// The search string is expected to only contain a single keyword. This is
    /// the lightest and fastest autocompletion type. It is good for compact
    /// interfaces or where records are very simple. Results are returned in
    /// lexographic order.
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
    /// let autocomplete_options = search_index.autocomplete_keyword(&5, "E");
    ///
    /// assert_eq!(
    ///     // Convert `BTreeMap<&String>` to `Vec<&str>` for comparison:
    ///     autocomplete_options.into_iter().collect::<Vec<&str>>(),
    ///     vec![&"edgar".to_string(), &"edgar ætheling".to_string(), &"england".to_string()]
    /// );
    /// ```
    #[tracing::instrument(level = "trace", name = "keyword autocomplete", skip(self))]
    pub(crate) fn autocomplete_keyword(
        &self,
        maximum_autocomplete_options: &usize,
        keyword: &str,
    ) -> Vec<&str> {
        // If the search index is set to be case insensitive, normalize the
        // keyword to lower-case:
        let keyword = self.normalize(keyword);

        // For debug builds:
        #[cfg(debug_assertions)]
        tracing::debug!("autocompleting: {:?}", keyword);

        // Attempt to get matching keywords from `BTreeMap`:
        let mut autocomplete_options: Vec<&str> = self
            .b_tree_map
            // Get matching keywords starting with (partial) keyword string:
            .range(kstring::KString::from_ref(&keyword)..)
            // `range` returns a key-value pair. We're autocompleting the
            // key (keyword), so discard the value (record key):
            .map(|(key, _value)| key)
            // We did not specify an end bound for our `range` function (see
            // above.) `range` will return _every_ keyword greater than the
            // supplied keyword. The below `take_while` will effectively break
            // iteration when we reach a keyword that does not start with our
            // supplied (partial) keyword.
            .take_while(|autocompletion| autocompletion.starts_with(keyword.as_ref()))
            // If the index's keyword matches the user's keyword, don't return
            // it as a result. For example, if the user's keyword was "new" (as
            // in New York), do not return "new" as an auto-completed keyword:
            // .filter(|autocompletion| *autocompletion != &keyword)
            // Only return `maximum_autocomplete_options` number of keywords:
            .take(*maximum_autocomplete_options)
            // Convert `&KString` to `&str`:
            .map(KStringBase::as_str)
            // Collect all keyword autocompletions into a `Vec`:
            .collect();

        // If `rapidfuzz` fuzzy matching enabled, this will examine the
        // autocompletion results. If the autocompletion results are empty, it
        // will use fuzzy matching to find closest possibilities:
        #[cfg(feature = "rapidfuzz")]
        crate::simple::internal::fuzzers::Rapidfuzz::autocomplete_keyword(
            self,
            &mut autocomplete_options,
            &keyword,
        );

        // If `strsim` fuzzy matching enabled, this will examine the
        // autocompletion results. If the autocompletion results are empty, it
        // will use fuzzy matching to find closest possibilities:
        #[cfg(feature = "strsim")]
        crate::simple::internal::fuzzers::Strsim::autocomplete_keyword(
            self,
            &mut autocomplete_options,
            &keyword,
        );

        // If `eddie` fuzzy matching enabled, this will examine the
        // autocompletion results. If the autocompletion results are empty, it
        // will use fuzzy matching to find closest possibilities:
        #[cfg(feature = "eddie")]
        crate::simple::internal::fuzzers::Eddie::autocomplete_keyword(
            self,
            &mut autocomplete_options,
            &keyword,
        );

        // If fuzzy string searching disabled, return the resulting
        // auto-complete options without further processing:
        #[cfg(not(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim")))]
        autocomplete_options
            .into_iter()
            .map(KStringBase::as_str)
            .collect();


        autocomplete_options
    } // fn
} // impl
