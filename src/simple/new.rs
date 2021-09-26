use crate::simple::{SearchIndex, SearchType};
use std::cmp::Ord;
use std::collections::BTreeMap;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Makes a new, empty `SearchIndex`. It might be preferrable to use
    /// `SearchIndex::default()` or `SearchIndexBuilder::default()` to create
    /// a new search index.

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        search_type: SearchType,
        autocomplete_type: SearchType,
        split_pattern: Option<Vec<char>>,
        case_sensitive: bool,
        minimum_keyword_length: usize,
        maximum_keyword_length: usize,
        maximum_string_length: Option<usize>,
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
            maximum_autocomplete_results,
            maximum_search_results,
        } // SearchIndex
    } // fn

} // impl