use std::collections::BTreeMap;
use std::fmt::Debug;

// -----------------------------------------------------------------------------
//
/// Structure that represents a search index.

#[derive(Debug)]
pub struct SearchIndex<K: Debug> {
    /// The search index data structure.
    pub(crate) b_tree_map: BTreeMap<String, Vec<K>>,
    /// The characters that splits strings into keywords.
    pub(crate) split_pattern: Option<Vec<char>>,
    /// Indicates whether the search index is case sensitive or not. If set to
    /// false (case insensitive), all keywords will be converted to lower case.
    pub(crate) case_sensitive: bool,
    /// Minimum keyword length (in chars or codepoints) to be indexed.
    pub(crate) minimum_keyword_length: usize,
    /// Maximum keyword length (in chars or codepoints) to be indexed.
    pub(crate) maximum_keyword_length: usize,
    /// Maximum string length (in chars or codepoints) to be indexed. If set,
    /// Indicium will also index the record's full field text / whole strings
    /// as a single keyword for autocompletion purposes.
    pub(crate) maximum_string_length: Option<usize>,
    /// Maximum number of auto-complete options to return.
    pub(crate) maximum_autocomplete_results: usize,
    /// Maximum number of search results to return.
    pub(crate) maximum_search_results: usize,
} // SearchIndex