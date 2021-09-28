use crate::simple::{AutocompleteType, SearchIndex, SearchType};
use std::cmp::Ord;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

// -----------------------------------------------------------------------------
//
/// The [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html)
/// can be used to customize your search index. If you're in a hurry, you can
/// instantiate your search index with `SearchIndex::default()` instead.

pub struct SearchIndexBuilder<K> {
    b_tree_map: BTreeMap<String, BTreeSet<K>>,
    search_type: SearchType,
    autocomplete_type: AutocompleteType,
    split_pattern: Option<Vec<char>>,
    case_sensitive: bool,
    minimum_keyword_length: usize,
    maximum_keyword_length: usize,
    maximum_string_length: Option<usize>,
    maximum_autocomplete_results: usize,
    maximum_search_results: usize,
} // SearchIndexBuilder

// -----------------------------------------------------------------------------

impl<K: Debug + Ord> From<SearchIndex<K>> for SearchIndexBuilder<K> {
    /// Convert to `SearchIndexBuilder<K>` struct from `SearchIndex<K>` struct.
    fn from(search_index: SearchIndex<K>) -> Self {
        SearchIndexBuilder {
            b_tree_map: search_index.b_tree_map,
            search_type: search_index.search_type,
            autocomplete_type: search_index.autocomplete_type,
            split_pattern: search_index.split_pattern,
            case_sensitive: search_index.case_sensitive,
            minimum_keyword_length: search_index.minimum_keyword_length,
            maximum_keyword_length: search_index.maximum_keyword_length,
            maximum_string_length: search_index.maximum_string_length,
            maximum_autocomplete_results: search_index.maximum_autocomplete_results,
            maximum_search_results: search_index.maximum_search_results,
        } // SearchIndexBuilder
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl<K: Debug + Ord> From<SearchIndexBuilder<K>> for SearchIndex<K> {
    /// Convert to `SearchIndex<K>` struct from `SearchIndexBuilder<K>` struct.
    fn from(search_index: SearchIndexBuilder<K>) -> Self {
        SearchIndex {
            b_tree_map: search_index.b_tree_map,
            search_type: search_index.search_type,
            autocomplete_type: search_index.autocomplete_type,
            split_pattern: search_index.split_pattern,
            case_sensitive: search_index.case_sensitive,
            minimum_keyword_length: search_index.minimum_keyword_length,
            maximum_keyword_length: search_index.maximum_keyword_length,
            maximum_string_length: search_index.maximum_string_length,
            maximum_autocomplete_results: search_index.maximum_autocomplete_results,
            maximum_search_results: search_index.maximum_search_results,
        } // SearchIndexBuilder
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl<K: Debug + Ord> SearchIndexBuilder<K> {

    /// Initialize `SearchIndexBuilder` with default settings.
    pub fn default() -> SearchIndexBuilder<K> {
        SearchIndexBuilder::from(SearchIndex::default())
    } // fn

    /// Logical conjuction for connecting search results for each keyword.
    pub fn search_type(&mut self, search_type: &SearchType) -> &mut SearchIndexBuilder<K> {
        self.search_type = search_type.to_owned();
        self
    } // fn

    /// Logical conjuction for connecting autcompletion results for each
    /// keyword.
    pub fn autocomplete_type(&mut self, autocomplete_type: &AutocompleteType) -> &mut SearchIndexBuilder<K> {
        self.autocomplete_type = autocomplete_type.to_owned();
        self
    } // fn

    /// Characters used to split strings into keywords.
    pub fn split_pattern(&mut self, split_pattern: &Option<Vec<char>>) -> &mut SearchIndexBuilder<K> {
        self.split_pattern = split_pattern.to_owned();
        self
    } // fn

    /// Indicates whether the search index is case sensitive or not. If set to
    /// false (case insensitive), all keywords will be normalized to lower case.
    pub fn case_sensitive(&mut self, case_sensitive: &bool) -> &mut SearchIndexBuilder<K> {
        self.case_sensitive = *case_sensitive;
        self
    } // fn

    /// Minimum keyword length (in chars or codepoints) to be indexed.
    pub fn min_keyword_len(&mut self, minimum_keyword_length: &usize) -> &mut SearchIndexBuilder<K> {
        self.minimum_keyword_length = *minimum_keyword_length;
        self
    } // fn

    /// Maximum keyword length (in chars or codepoints) to be indexed.
    pub fn max_keyword_len(&mut self, maximum_keyword_length: &usize) -> &mut SearchIndexBuilder<K> {
        self.maximum_keyword_length = *maximum_keyword_length;
        self
    } // fn

    /// Maximum string length (in chars or codepoints) to be indexed. If set,
    /// Indicium will index the record's full field text / whole strings as a
    /// single keyword for autocompletion purposes.
    pub fn max_string_len(&mut self, maximum_string_length: &Option<usize>) -> &mut SearchIndexBuilder<K> {
        self.maximum_string_length = *maximum_string_length;
        self
    } // fn

    /// Maximum number of auto-complete options to return.
    pub fn max_autocomplete_results(&mut self, maximum_autocomplete_results: &usize) -> &mut SearchIndexBuilder<K> {
        self.maximum_autocomplete_results = *maximum_autocomplete_results;
        self
    } // fn

    /// Maximum number of search results to return.
    pub fn max_search_results(&mut self, maximum_search_results: &usize) -> &mut SearchIndexBuilder<K> {
        self.maximum_search_results = *maximum_search_results;
        self
    } // fn

    /// Build `SearchIndex` from the settings given to the `SearchIndexBuilder`.
    pub fn build(self) -> SearchIndex<K> {
        SearchIndex::from(self)
    } // fn

} // impl