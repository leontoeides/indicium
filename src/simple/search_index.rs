use crate::simple::Conjunction;
use std::collections::BTreeMap;
use std::fmt::Debug;

// -----------------------------------------------------------------------------
//
/// Structure that represents a search index.

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SearchIndex<K: Debug> {
    /// Search index data structure.
    // Note: `Vec<K>` was chosen over `HashSet<K>` and `BTreeSet<K>` because it
    // supports multiple elements of `K`. This way, if a record contains the
    // same keyword multiple times, the record can be returned as more
    // relevant.
    pub(crate) b_tree_map: BTreeMap<String, Vec<K>>,
    /// Logical conjuction for connecting search results for each keyword.
    pub(crate) conjunction: Conjunction,
    /// Characters used to split strings into keywords.
    pub(crate) split_pattern: Option<Vec<char>>,
    /// Indicates whether the search index is case sensitive or not. If set to
    /// false (case insensitive), all keywords will be normalized to lower case.
    pub(crate) case_sensitive: bool,
    /// Minimum keyword length (in chars or codepoints) to be indexed.
    pub(crate) minimum_keyword_length: usize,
    /// Maximum keyword length (in chars or codepoints) to be indexed.
    pub(crate) maximum_keyword_length: usize,
    /// Maximum string length (in chars or codepoints) to be indexed. If set,
    /// Indicium will index the record's full field text / whole strings as a
    /// single keyword for autocompletion purposes.
    pub(crate) maximum_string_length: Option<usize>,
    /// Maximum number of auto-complete options to return.
    pub(crate) maximum_autocomplete_results: usize,
    /// Maximum number of search results to return.
    pub(crate) maximum_search_results: usize,
} // SearchIndex