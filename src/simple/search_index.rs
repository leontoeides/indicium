use crate::simple::{AutocompleteType, EddieMetric, RapidfuzzMetric, SearchType, StrsimMetric};
use kstring::KString;
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
#[derive(Debug)]
pub struct SearchIndex<K: Ord> {
    /// Search index data structure.
    pub(crate) b_tree_map: BTreeMap<KString, BTreeSet<K>>,

    /// The `SearchType` for searches. This setting may be manually overridden
    /// by using the `search_type` method.
    pub(crate) search_type: SearchType,

    /// The `AutocompleteType` for autocompletions. This setting may be manually
    /// overridden by using the `autocompletion_type` method.
    pub(crate) autocomplete_type: AutocompleteType,

    /// Used for the `eddie` optional feature. The `EddieMetric` is used to
    /// select the string similarity metric (or algorithm) for fuzzy matching.
    pub(crate) eddie_metric: Option<EddieMetric>,

    /// Used for the `rapidfuzz` optional feature. The `RapidfuzzMetric` is used
    /// to select the string similarity metric (or algorithm) for fuzzy
    /// matching.
    pub(crate) rapidfuzz_metric: Option<RapidfuzzMetric>,

    /// Used for the `strsim` optional feature. The `StrsimMetric` is used to
    /// select the string similarity metric (or algorithm) for fuzzy matching.
    pub(crate) strsim_metric: Option<StrsimMetric>,

    /// Used for the `eddie`, `rapidfuzz`, and `strsim` optional features.
    /// Search index keyword must match the first _n_ characters of the user's
    /// keyword in order to be evaluated for fuzzy matching.
    pub(crate) fuzzy_length: usize,

    /// Used for both the `strsim` and `eddie` optional features. Minimum score
    /// for the search index's keyword to be returned as an alternative to the
    /// user's keyword. Score is between `0.0` and `1.0` (inclusive), where
    /// `1.0` means the strings are the same.
    pub(crate) fuzzy_minimum_score: f64,

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
    pub(crate) exclude_keywords: Option<Vec<KString>>,

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
    pub(crate) dump_keyword: Option<KString>,

    /// The `empty_b_tree_set` allows us to trick the compiler into returning an
    /// empty `impl Iterator` with no memory allocations when there are no keys
    /// associated with a keyword.
    ///
    /// Without this, we would get a "distinct uses of `impl Trait` result in
    /// different opaque types" error.
    pub(crate) empty_b_tree_set: BTreeSet<K>,

    /// A normalizer for performing composing Unicode normalization.
    #[cfg(feature = "icu_normalizer")]
    pub(crate) icu_normalizer: icu_normalizer::ComposingNormalizerBorrowed<'static>,
} // SearchIndex

// -----------------------------------------------------------------------------
//
// Trait Implementations

impl<K: Ord + Clone> Clone for SearchIndex<K> {
    fn clone(&self) -> Self {
        Self {
            b_tree_map: self.b_tree_map.clone(),
            search_type: self.search_type.clone(),
            autocomplete_type: self.autocomplete_type.clone(),
            eddie_metric: self.eddie_metric.clone(),
            rapidfuzz_metric: self.rapidfuzz_metric.clone(),
            strsim_metric: self.strsim_metric.clone(),
            fuzzy_length: self.fuzzy_length,
            fuzzy_minimum_score: self.fuzzy_minimum_score,
            split_pattern: self.split_pattern.clone(),
            case_sensitive: self.case_sensitive,
            minimum_keyword_length: self.minimum_keyword_length,
            maximum_keyword_length: self.maximum_keyword_length,
            maximum_string_length: self.maximum_string_length,
            exclude_keywords: self.exclude_keywords.clone(),
            maximum_autocomplete_options: self.maximum_autocomplete_options,
            maximum_search_results: self.maximum_search_results,
            maximum_keys_per_keyword: self.maximum_keys_per_keyword,
            dump_keyword: self.dump_keyword.clone(),
            empty_b_tree_set: self.empty_b_tree_set.clone(),
            #[cfg(feature = "icu_normalizer")]
            icu_normalizer: icu_normalizer::ComposingNormalizer::new_nfkc(),
        }
    }
}

impl<K: Ord + PartialEq> PartialEq for SearchIndex<K> {
    fn eq(&self, other: &Self) -> bool {
        self.b_tree_map == other.b_tree_map
            && self.search_type == other.search_type
            && self.autocomplete_type == other.autocomplete_type
            && self.eddie_metric == other.eddie_metric
            && self.rapidfuzz_metric == other.rapidfuzz_metric
            && self.strsim_metric == other.strsim_metric
            && self.fuzzy_length == other.fuzzy_length
            && self.fuzzy_minimum_score == other.fuzzy_minimum_score
            && self.split_pattern == other.split_pattern
            && self.case_sensitive == other.case_sensitive
            && self.minimum_keyword_length == other.minimum_keyword_length
            && self.maximum_keyword_length == other.maximum_keyword_length
            && self.maximum_string_length == other.maximum_string_length
            && self.exclude_keywords == other.exclude_keywords
            && self.maximum_autocomplete_options == other.maximum_autocomplete_options
            && self.maximum_search_results == other.maximum_search_results
            && self.maximum_keys_per_keyword == other.maximum_keys_per_keyword
            && self.dump_keyword == other.dump_keyword
            && self.empty_b_tree_set == other.empty_b_tree_set
        // Note: icu_normalizer is intentionally excluded since
        // ComposingNormalizer doesn't implement PartialEq
    }
}