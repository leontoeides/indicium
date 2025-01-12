use crate::simple::{AutocompleteType, EddieMetric, RapidfuzzMetric, SearchIndex, SearchType, StrsimMetric};
use kstring::KString;
use std::collections::{BTreeMap, BTreeSet};

// -----------------------------------------------------------------------------
//
/// The [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html)
/// can be used to customize your search index. Use
/// `SearchIndexBuilder::default()` to start the builder chain, and `.build()`
/// to finish it.
///
/// If you're in a hurry, you can instantiate your search index with
/// `SearchIndex::default()` instead.
pub struct SearchIndexBuilder<K> {
    b_tree_map: BTreeMap<KString, BTreeSet<K>>,
    search_type: SearchType,
    autocomplete_type: AutocompleteType,
    eddie_metric: Option<EddieMetric>,
    rapidfuzz_metric: Option<RapidfuzzMetric>,
    strsim_metric: Option<StrsimMetric>,
    fuzzy_length: usize,
    fuzzy_minimum_score: f64,
    split_pattern: Option<Vec<char>>,
    case_sensitive: bool,
    minimum_keyword_length: usize,
    maximum_keyword_length: usize,
    maximum_string_length: Option<usize>,
    exclude_keywords: Option<Vec<KString>>,
    maximum_autocomplete_options: usize,
    maximum_search_results: usize,
    maximum_keys_per_keyword: usize,
    dump_keyword: Option<KString>,
} // SearchIndexBuilder

// -----------------------------------------------------------------------------

impl<K: Clone + Ord> From<SearchIndex<K>> for SearchIndexBuilder<K> {
    /// Convert to `SearchIndexBuilder<K>` struct from `SearchIndex<K>` struct.
    fn from(search_index: SearchIndex<K>) -> Self {
        Self {
            b_tree_map: search_index.b_tree_map,
            search_type: search_index.search_type,
            autocomplete_type: search_index.autocomplete_type,
            eddie_metric: search_index.eddie_metric,
            rapidfuzz_metric: search_index.rapidfuzz_metric,
            strsim_metric: search_index.strsim_metric,
            fuzzy_length: search_index.fuzzy_length,
            fuzzy_minimum_score: search_index.fuzzy_minimum_score,
            split_pattern: search_index.split_pattern,
            case_sensitive: search_index.case_sensitive,
            minimum_keyword_length: search_index.minimum_keyword_length,
            maximum_keyword_length: search_index.maximum_keyword_length,
            maximum_string_length: search_index.maximum_string_length,
            exclude_keywords: search_index.exclude_keywords,
            maximum_autocomplete_options: search_index.maximum_autocomplete_options,
            maximum_search_results: search_index.maximum_search_results,
            maximum_keys_per_keyword: search_index.maximum_keys_per_keyword,
            dump_keyword: search_index.dump_keyword,
        } // SearchIndexBuilder
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl<K: Clone + Ord> From<SearchIndexBuilder<K>> for SearchIndex<K> {
    /// Convert to `SearchIndex<K>` struct from `SearchIndexBuilder<K>` struct.
    fn from(search_index: SearchIndexBuilder<K>) -> Self {
        Self {
            b_tree_map: search_index.b_tree_map,
            search_type: search_index.search_type,
            autocomplete_type: search_index.autocomplete_type,
            eddie_metric: search_index.eddie_metric,
            rapidfuzz_metric: search_index.rapidfuzz_metric,
            strsim_metric: search_index.strsim_metric,
            fuzzy_length: search_index.fuzzy_length,
            fuzzy_minimum_score: search_index.fuzzy_minimum_score,
            split_pattern: search_index.split_pattern,
            case_sensitive: search_index.case_sensitive,
            minimum_keyword_length: search_index.minimum_keyword_length,
            maximum_keyword_length: search_index.maximum_keyword_length,
            maximum_string_length: search_index.maximum_string_length,
            exclude_keywords: search_index.exclude_keywords,
            maximum_autocomplete_options: search_index.maximum_autocomplete_options,
            maximum_search_results: search_index.maximum_search_results,
            maximum_keys_per_keyword: search_index.maximum_keys_per_keyword,
            dump_keyword: search_index.dump_keyword,
        } // SearchIndexBuilder
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl<K: Clone + Ord> Default for SearchIndexBuilder<K> {
    /// Initialize `SearchIndexBuilder` with default settings.
    fn default() -> Self {
        Self::from(SearchIndex::default())
    } // fn
} // impl Default

// -----------------------------------------------------------------------------

impl<K: Clone + Ord> SearchIndexBuilder<K> {
    /// Search type (or logical conjuction). Used to determine how to connect
    /// search results for each keyword. See [`SearchType`] for more
    /// information.
    ///
    /// **Default:** `SearchType::Live`
    ///
    /// [`SearchType`]: enum.SearchType.html
    #[must_use]
    pub const fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = search_type;
        self
    } // fn

    /// Autocomplete type (or keyword scope). Used to determine if or how to
    /// filtering keyword results for autocompletion. See [`AutocompleteType`]
    /// for more information.
    ///
    /// **Default:** `AutocompleteType::Context`
    ///
    /// [`AutocompleteType`]: enum.AutocompleteType.html
    #[must_use]
    pub const fn autocomplete_type(mut self, autocomplete_type: AutocompleteType) -> Self {
        self.autocomplete_type = autocomplete_type;
        self
    } // fn

    /// String similarity metric type from Danny Guo's
    /// [strsim](https://crates.io/crates/strsim) crate. Used for fuzzy matching
    /// user's keywords when no exact matches were found. See [`StrsimMetric`] for
    /// more information.
    ///
    /// **Default:** `StrsimMetric::Levenshtein`
    ///
    /// [`StrsimMetric`]: enum.StrsimMetric.html
    #[cfg(feature = "strsim")]
    #[must_use]
    pub const fn strsim_metric(mut self, strsim_metric: Option<StrsimMetric>) -> Self {
        self.strsim_metric = strsim_metric;
        self
    } // fn

    /// String similarity metric type from Ilia Schelokov's
    /// [eddie](https://crates.io/crates/eddie) crate. Used for fuzzy matching
    /// user's keywords when no exact matches were found. See [`EddieMetric`]
    /// for more information.
    ///
    /// **Default:** `EddieMetric::Levenshtein`
    ///
    /// [`EddieMetric`]: enum.EddieMetric.html
    #[cfg(feature = "eddie")]
    #[must_use]
    pub const fn eddie_metric(mut self, eddie_metric: Option<EddieMetric>) -> Self {
        self.eddie_metric = eddie_metric;
        self
    } // fn

    /// String similarity metric type from the
    /// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate. Used for fuzzy
    /// matching user's keywords when no exact matches were found. See
    /// [`RapidfuzzMetric`] for more information.
    ///
    /// **Default:** `RapidfuzzMetric::Levenshtein`
    ///
    /// [`RapidfuzzMetric`]: enum.RapidfuzzMetric.html
    #[cfg(feature = "rapidfuzz")]
    #[must_use]
    pub const fn rapidfuzz_metric(mut self, rapidfuzz_metric: Option<RapidfuzzMetric>) -> Self {
        self.rapidfuzz_metric = rapidfuzz_metric;
        self
    } // fn

    /// String's minimum length (in chars or codepoints) to use "approximate
    /// string matching" or "fuzzy matching."
    ///
    /// #### Examples
    ///
    /// | Example | User Keyword                       | Minimum Length | Index Keyword Must Start With... |
    /// |---------|------------------------------------|----------------|----------------------------------|
    /// | 1       | Supercalifragilisticexpialidocious | 2              | Su                               |
    /// | 2       | Antidisestablishmentarianism       | 4              | Anti                             |
    /// | 3       | Pseudopseudohypoparathyroidism     | 0              |                                  |
    ///
    /// * In example **1**, since the length is set to `2`, the user's keyword
    ///   will only be fuzzy matched against keywords in the search index that
    ///   begin with `su`.
    ///
    /// * In example **2**, since the length is set to `4`, the user's keyword
    ///   will only be fuzzy matched against keywords in the search index that
    ///   begin with `anti`.
    ///
    /// * In example **3**, since the length is set to `0`, the user's keyword
    ///   will be fuzzy matched against every keyword in the search index. This
    ///   is OK (or even desirable) if the search index is small, however, this
    ///   will be crippling slow on very large search indicies.
    ///
    /// **Default:** `3` characters
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    #[must_use]
    pub const fn fuzzy_length(mut self, fuzzy_length: usize) -> Self {
        self.fuzzy_length = fuzzy_length;
        self
    } // fn

    /// Keyword's minimum score to be used as a possible fuzzy match. Must be a
    /// value between 0.0 and 1.0 (inclusive), where 1.0 means the strings are
    /// the same.
    ///
    /// When there aren't many good possible matches for a user's keyword, the
    /// quality of the suggestions and substitutions can become very poor. The
    /// minimum score helps ensure the suggestion and subtitutions are
    /// reasonable.
    ///
    /// If there are no reasonable suggestions or subsitutions, nothing will
    /// be returned to the user.
    ///
    /// **Default:** `0.3`
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    #[must_use]
    pub const fn fuzzy_minimum_score(mut self, fuzzy_minimum_score: f64) -> Self {
        self.fuzzy_minimum_score = fuzzy_minimum_score;
        self
    } // fn

    /// Characters used to split strings into keywords.
    ///
    /// **Default:** [ `tab`, `new line`, `carrier return`, `space`, `!`, `"`,
    /// `&`, `(`, `)`, `*`, `+`, `,`, `-`, `.`, `/`, `:`, `;`, `<`, `=`, `>`,
    /// `?`, `[`, `\`, `]`, `^`, `'`, `{`, `|`, `}`, `~`, ` `, `¡`, `«`, `»`,
    /// `¿`, `×`, `÷`, `ˆ`, `‘`, `’`, `“`, `”`, `„`, `‹`, `›` ]
    #[must_use]
    pub fn split_pattern(mut self, split_pattern: Option<Vec<char>>) -> Self {
        self.split_pattern = split_pattern;
        self
    } // fn

    /// Indicates whether the search index is case sensitive or not. If set to
    /// false (case insensitive), all keywords will be normalized to lower case.
    ///
    /// **Default:** `false`
    #[must_use]
    pub const fn case_sensitive(mut self, case_sensitive: bool) -> Self {
        self.case_sensitive = case_sensitive;
        self
    } // fn

    /// Minimum keyword length (in chars or codepoints) to be indexed. If the
    /// keyword is shorter the keyword will not be indexed.
    ///
    /// **Default:** `1`
    #[must_use]
    pub const fn min_keyword_len(mut self, minimum_keyword_length: usize) -> Self {
        self.minimum_keyword_length = minimum_keyword_length;
        self
    } // fn

    /// Maximum keyword length (in chars or codepoints) to be indexed. If the
    /// keyword is longer the keyword will not be indexed.
    ///
    /// **Default:** `24`
    #[must_use]
    pub const fn max_keyword_len(mut self, maximum_keyword_length: usize) -> Self {
        self.maximum_keyword_length = maximum_keyword_length;
        self
    } // fn

    /// Maximum string length (in chars or codepoints) to be indexed. If set,
    /// Indicium will index the record's _full field text_ & _whole strings_ as
    /// a single keyword for autocompletion purposes.
    ///
    /// **Default:** `Some(24)`
    #[must_use]
    pub const fn max_string_len(mut self, maximum_string_length: Option<usize>) -> Self {
        self.maximum_string_length = maximum_string_length;
        self
    } // fn

    /// List of keywords that should not be indexed. It might be a good idea to
    /// exclude minor words - short conjunctions, articles, and short
    /// prepositions from your search index. For example, words such as `and`,
    /// `as`, `a`, `as`, `at`, etc. See also: the [`profile`] utility method.
    ///
    /// [`profile`]: struct.SearchIndex.html#method.profile
    #[must_use]
    pub fn exclude_keywords(mut self, exclude_keywords: Option<Vec<String>>) -> Self {
        self.exclude_keywords =
            exclude_keywords.map(|vec| vec.into_iter().map(std::convert::Into::into).collect());
        self
    } // fn

    /// Maximum number of auto-complete options to return. This setting can be
    /// overidden by some function arguments.
    ///
    /// **Default:** `5`
    #[must_use]
    pub const fn max_autocomplete_options(mut self, maximum_autocomplete_options: usize) -> Self {
        self.maximum_autocomplete_options = maximum_autocomplete_options;
        self
    } // fn

    /// Maximum number of search results to return. This setting can be
    /// overidden by some function arguments.
    ///
    /// **Default:** `100`
    #[must_use]
    pub const fn max_search_results(mut self, maximum_search_results: usize) -> Self {
        self.maximum_search_results = maximum_search_results;
        self
    } // fn

    /// Maximum number of keys per keyword. If there are too many records
    /// attached to a single keyword, performance can begin to degrade. This
    /// setting limits the number of keys that may be attached to a keyword. See
    /// also: the `exclude_keywords` list and the `profile` method.
    ///
    /// **Default:** `40_960`
    #[must_use]
    pub const fn max_keys_per_keyword(mut self, maximum_keys_per_keyword: usize) -> Self {
        self.maximum_keys_per_keyword = maximum_keys_per_keyword;
        self
    } // fn

    /// A special keyword that will return or "dump" all keys (or records) in
    /// the search index. This is helpful for the `Select2` module, where it
    /// should be returning all records if the search string is empty.
    ///
    /// **Default:** `Some("\0".to_string())`
    #[must_use]
    pub fn dump_keyword(mut self, dump_keyword: Option<String>) -> Self {
        self.dump_keyword = dump_keyword.map(std::convert::Into::into);
        self
    } // fn

    /// Build `SearchIndex` from the settings given to the `SearchIndexBuilder`.
    #[must_use]
    pub fn build(self) -> SearchIndex<K> {
        SearchIndex::from(self)
    } // fn
} // impl
