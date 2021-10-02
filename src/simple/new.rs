use crate::simple::{AutocompleteType, SearchIndex, SearchType};
use std::cmp::Ord;
use std::collections::BTreeMap;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Makes a new, empty `SearchIndex`. It might be preferrable to use
    /// `SearchIndex::default()` or `SearchIndexBuilder::default()` to create
    /// a new search index.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use indicium::simple::{AutocompleteType, SearchIndex, SearchType};
    /// #
    /// let mut search_index = SearchIndex::<usize>::new(
    ///     SearchType::Or,                 // Search type.
    ///     AutocompleteType::Context,      // Autocompletion type.
    ///     Some(vec![' ', '\n', '\r', '\t', ',', '.']), // Split characters.
    ///     false,                          // Case sensitive?
    ///     1,                              // Minimum keyword length (in chars or codepoints.)
    ///     24,                             // Maximum keyword length (in chars or codepoints.)
    ///     Some(24),                       // Maximum text length (in chars or codepoints.)
    ///     5,                              // Maximum number of auto-complete options.
    ///     100,                            // Maximum number of search results.
    /// );
    /// ```

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        search_type: SearchType,
        autocomplete_type: AutocompleteType,
        split_pattern: Option<Vec<char>>,
        case_sensitive: bool,
        minimum_keyword_length: usize,
        maximum_keyword_length: usize,
        maximum_string_length: Option<usize>,
        exclude_keywords: Option<Vec<String>>,
        maximum_autocomplete_results: usize,
        maximum_search_results: usize,
    ) -> SearchIndex<K> {
        SearchIndex {
            search_type,
            autocomplete_type,
            b_tree_map: BTreeMap::new(),
            split_pattern,
            case_sensitive,
            minimum_keyword_length,
            maximum_keyword_length,
            maximum_string_length,
            exclude_keywords,
            maximum_autocomplete_results,
            maximum_search_results,
        } // SearchIndex
    } // fn

} // impl