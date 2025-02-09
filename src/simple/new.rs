impl<K: Ord> crate::simple::SearchIndex<K> {
    /// Makes a new, empty `SearchIndex`. It might be more convenient to use
    /// `SearchIndex::default()` or `SearchIndexBuilder::default()` to create
    /// a new search index.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use indicium::simple::{AutocompleteType, EddieMetric, RapidfuzzMetric, SearchIndex, SearchType, StrsimMetric};
    /// #
    /// let mut search_index = SearchIndex::<usize>::new(
    ///     SearchType::Or,                     // Search type.
    ///     AutocompleteType::Context,          // Autocompletion type.
    ///     Some(EddieMetric::Levenshtein),     // String similarity metric type.
    ///     Some(RapidfuzzMetric::Levenshtein), // String similarity metric type.
    ///     Some(StrsimMetric::Levenshtein),    // String similarity metric type.
    ///     3,                                  // String similarity match length.
    ///     0.5,                                // String similarity minimum score.
    ///     Some(vec![' ', '\n', '\r', '\t', ',', '.']), // Split characters.
    ///     false,                              // Case sensitive?
    ///     1,                                  // Minimum keyword length (in chars or codepoints.)
    ///     24,                                 // Maximum keyword length (in chars or codepoints.)
    ///     Some(24),                           // Maximum text length (in chars or codepoints.)
    ///     Some(vec!["a".to_string(), "the".to_string()]), // Keyword exclusions.
    ///     5,                                  // Maximum number of auto-complete options.
    ///     100,                                // Maximum number of search results.
    ///     40_960,                             // Maximum keys per keyword.
    ///     Some("\0".to_string()),             // Dump keyword.
    /// );
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        search_type: crate::simple::SearchType,
        autocomplete_type: crate::simple::AutocompleteType,
        eddie_metric: Option<crate::simple::EddieMetric>,
        rapidfuzz_metric: Option<crate::simple::RapidfuzzMetric>,
        strsim_metric: Option<crate::simple::StrsimMetric>,
        fuzzy_length: usize,
        fuzzy_minimum_score: f64,
        split_pattern: Option<Vec<char>>,
        case_sensitive: bool,
        minimum_keyword_length: usize,
        maximum_keyword_length: usize,
        maximum_string_length: Option<usize>,
        exclude_keywords: Option<Vec<String>>,
        maximum_autocomplete_options: usize,
        maximum_search_results: usize,
        maximum_keys_per_keyword: usize,
        dump_keyword: Option<String>,
    ) -> Self {
        Self {
            b_tree_map: std::collections::BTreeMap::new(),
            search_type,
            autocomplete_type,
            eddie_metric,
            rapidfuzz_metric,
            strsim_metric,
            fuzzy_length,
            fuzzy_minimum_score,
            split_pattern,
            case_sensitive,
            minimum_keyword_length,
            maximum_keyword_length,
            maximum_string_length,
            exclude_keywords: exclude_keywords
                .map(|vec| vec.into_iter().map(std::convert::Into::into).collect()),
            maximum_autocomplete_options,
            maximum_search_results,
            maximum_keys_per_keyword,
            dump_keyword: dump_keyword.map(std::convert::Into::into),
            empty_b_tree_set: std::collections::BTreeSet::new(),
        } // SearchIndex
    } // fn
} // impl
