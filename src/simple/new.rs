use crate::simple::{AutocompleteType, SearchIndex, SearchType, StrSimType};
use std::cmp::Ord;
use std::collections::BTreeMap;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Makes a new, empty `SearchIndex`. It might be more convenient to use
    /// `SearchIndex::default()` or `SearchIndexBuilder::default()` to create
    /// a new search index.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use indicium::simple::{AutocompleteType, SearchIndex, SearchType, StrSimType};
    /// #
    /// let mut search_index = SearchIndex::<usize>::new(
    ///     SearchType::Or,                 // Search type.
    ///     AutocompleteType::Context,      // Autocompletion type.
    ///     Some(StrSimType::Levenshtein),  // String similarity metric type.
    ///     3,                              // String similarity match length.
    ///     0.5,                            // String similarity minimum score.
    ///     Some(vec![' ', '\n', '\r', '\t', ',', '.']), // Split characters.
    ///     false,                          // Case sensitive?
    ///     1,                              // Minimum keyword length (in chars or codepoints.)
    ///     24,                             // Maximum keyword length (in chars or codepoints.)
    ///     Some(24),                       // Maximum text length (in chars or codepoints.)
    ///     Some(vec!["a".to_string(), "the".to_string()]), // Keyword exclusions.
    ///     5,                              // Maximum number of auto-complete options.
    ///     100,                            // Maximum number of search results.
    ///     40_960,                         // Maximum keys per keyword.
    ///     Some("\0".to_string()),         // Dump keyword.
    /// );
    /// ```

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        search_type: SearchType,
        autocomplete_type: AutocompleteType,
        strsim_type: Option<StrSimType>,
        strsim_length: usize,
        strsim_minimum_score: f64,
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
    ) -> SearchIndex<K> {
        SearchIndex {
            b_tree_map: BTreeMap::new(),
            search_type,
            autocomplete_type,
            strsim_type,
            strsim_length,
            strsim_minimum_score,
            split_pattern,
            case_sensitive,
            minimum_keyword_length,
            maximum_keyword_length,
            maximum_string_length,
            exclude_keywords,
            maximum_autocomplete_options,
            maximum_search_results,
            maximum_keys_per_keyword,
            dump_keyword,
        } // SearchIndex
    } // fn

} // impl