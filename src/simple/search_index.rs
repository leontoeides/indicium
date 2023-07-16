use crate::simple::{AutocompleteType, SearchType, StrSimType};
use std::cmp::Ord;
use std::collections::{BTreeMap, BTreeSet};

// -----------------------------------------------------------------------------
//
/// **The search index**. This is the most important structure in Indicium
/// `simple` search. You may instantiate your search index with
/// `SearchIndex::default()` or use the `SearchIndexBuilder` builder pattern.
///
/// `K` generic represents the search index key type (i.e. `MyStruct`).
///
/// It's recommended to wrap your target collection (your `Vec`, `HashMap`,
/// etc.) and this `SearchIndex` together in a new `struct` type. Then,
/// implement the `insert`, `replace`, `remove`, etc. methods for this new
/// `struct` type that will update both the collection and search index. This
/// will ensure that both your collection and index are always synchronized.

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SearchIndex<K: Ord> {
    /// Search index data structure.
    pub(crate) b_tree_map: BTreeMap<String, BTreeSet<K>>,
    /// The `SearchType` for searches. This setting may be manually overridden
    /// by using the `search_type` method.
    pub(crate) search_type: SearchType,
    /// The `AutocompleteType` for autocompletions. This setting may be manually
    /// overridden by using the `autocompletion_type` method.
    pub(crate) autocomplete_type: AutocompleteType,
    /// The `StrSimType` for string similarity fuzzy matching.
    pub(crate) strsim_type: Option<StrSimType>,
    /// Search index keyword must match the first _n_ characters of the user's
    /// keyword in order to be evaluated for fuzzy matching.
    pub(crate) strsim_length: usize,
    /// Minimum score for the search index's keyword to be returned as an
    /// alternative to the user's keyword. Score is between 0.0 and 1.0
    /// (inclusive), where 1.0 means the strings are the same.
    pub(crate) strsim_minimum_score: f64,
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
    /// Keywords that should not be indexed.
    pub(crate) exclude_keywords: Option<Vec<String>>,
    /// Maximum number of auto-complete options to return.
    pub(crate) maximum_autocomplete_options: usize,
    /// Maximum number of search results to return.
    pub(crate) maximum_search_results: usize,
    /// Maximum number of keys per keyword. If there are too many records
    /// attached to a single keyword, performance can begin to degrade. This
    /// setting limits the number of keys that may be attached to a keyword. See
    /// also: the `exclude_keywords` list and the `profile` method.
    pub(crate) maximum_keys_per_keyword: usize,
    /// A special keyword that will return (or "dump") all keys (or records) in
    /// the search index. It should be made so that it's difficult or impossible
    /// for a user inadvertently trigger this behaviour.
    pub(crate) dump_keyword: Option<String>,
} // SearchIndex