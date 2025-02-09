use kstring::KString;
use std::collections::BTreeSet;
use std::hash::Hash;

// -----------------------------------------------------------------------------
//
/// The `Fuzzy` trait allows `indicium` to treat the various string similarity
/// crates (such as `eddie`, `rapidfuzz`, `strsim`, etc.) generically.
pub trait Fuzzy<'s, K: Hash + Ord> {
    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the provided keyword and configured string similarity crate and
    /// metric.
    ///
    /// When the user's keyword that's meant to be autocompleted returns no
    /// matches, this can be used to find the best matches for substitution.
    ///
    /// # Input
    ///
    /// * `search_index` · A reference to the search index.
    ///
    /// * `last_autocomplete_options` · The current autocompletion options.
    ///
    ///   If this list is empty (because no exact keyword matches could be
    ///   found), it will be populated using fuzzy matches.
    ///
    /// * `keyword` · The keyword being searched for.
    ///
    /// # Output
    ///
    /// This method will attempt to update the `results` autocompletion options,
    /// if it's empty.
    ///
    /// # Notes
    ///
    /// * This method is a small variation of `live_search_keyword`.
    ///   This method is meant to be more general, while the other is optimized
    ///   specifically for use in `Live` searches.
    ///
    /// * This method expects the input to be normalized already, i.e. if the
    ///   search is meant to be case-insensitive then the inputs should be in
    ///   lowercase.
    fn autocomplete_keyword(
        search_index: &'s crate::simple::search_index::SearchIndex<K>,
        last_autocomplete_options: &mut Vec<&'s str>,
        keyword: &str,
    );

    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the provided keyword and configured string similarity crate and
    /// metric.
    ///
    /// When the user's keyword that's meant to be autocompleted returns no
    /// matches, this can be used to find the best matches for substitution.
    ///
    /// # Input
    ///
    /// * `search_index` · A reference to the search index.
    ///
    /// * `preceding_keywords` · If the search string was `He who knows nothing,
    ///   loves nothing` the “preceding” keywords would be `he who knows nothing
    ///   loves`.
    ///
    ///   This collection of keywords is used to prevent previously typed
    ///   keywords from being suggested. In this case, the system would _not_
    ///   suggest `nothing` as an autocomplete keyword since it's already
    ///   present in the search string.
    ///
    /// * `last_results` · The current autocompletion options.
    ///
    ///   If this list is empty (because no exact keyword matches could be
    ///   found), it will be populated using fuzzy matches.
    ///
    /// * `last_keyword` · If the search string was `He who knows nothing,
    ///   loves nothing` the “last” keyword would be `nothing`.
    ///
    ///   This keyword is used to search the search index. For example, this
    ///   could potentially return `nothingness`, and `nothings` as
    ///   autocompletion options if those words were present in the index.
    ///
    /// # Output
    ///
    /// This method will attempt to update the `last_autocomplete_options`
    /// autocompletion options, if it's empty.
    ///
    /// # Notes
    ///
    /// * This method expects the input to be normalized already, i.e. if the
    ///   search is meant to be case-insensitive then the inputs should be in
    ///   lowercase.
    fn autocomplete_global(
        search_index: &'s crate::simple::search_index::SearchIndex<K>,
        preceding_keywords: &[KString],
        last_autocomplete_options: &mut Vec<&'s KString>,
        last_keyword: &str,
    );

    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the provided keyword and configured string similarity crate and
    /// metric.
    ///
    /// When the user's keyword that's meant to be autocompleted returns no
    /// matches, this can be used to find the best matches for substitution.
    ///
    /// # Input
    ///
    /// * `search_index` · A reference to the search index.
    ///
    /// * `preceding_keywords` · If the search string was `He who knows nothing,
    ///   loves nothing` the “preceding” keywords would be `he who knows nothing
    ///   loves`.
    ///
    ///   This collection of keywords is used to prevent previously typed
    ///   keywords from being suggested. In this case, the system would _not_
    ///   suggest `nothing` as an autocomplete keyword since it's already
    ///   present in the search string.
    ///
    /// * `last_keyword` · If the search string was `He who knows nothing,
    ///   loves nothing` the “last” keyword would be `nothing`.
    ///
    ///   This keyword is used to search the search index. For example, this
    ///   could potentially return `nothingness`, and `nothings` as
    ///   autocompletion options if those words were present in the index.
    ///
    /// * `last_autocomplete_options` · The current autocompletion options.
    ///
    ///   If this list is empty (because no exact keyword matches could be
    ///   found), it will be populated using fuzzy matches.
    ///
    /// # Output
    ///
    /// This method will attempt to update the `last_autocomplete_options`
    /// autocompletion options, if it's empty.
    ///
    /// # Notes
    ///
    /// * This method is a small variation of `live_search_context`.
    ///   This method is meant to be more general, while the other is optimized
    ///   specifically for use in `Live` searches.
    ///
    /// * This method expects the input to be normalized already, i.e. if the
    ///   search is meant to be case-insensitive then the inputs should be in
    ///   lowercase.
    fn autocomplete_context(
        search_index: &'s crate::simple::search_index::SearchIndex<K>,
        preceding_keywords: &[KString],
        preceding_results: &std::collections::BTreeSet<&'s K>,
        last_keyword: &str,
        last_autocomplete_options: &mut Vec<&'s KString>,
    );

    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the provided keyword and configured string similarity crate and
    /// metric.
    ///
    /// When the user's keyword that's meant to be autocompleted returns no
    /// matches, this can be used to find the best matches for substitution.
    ///
    /// # Notes
    ///
    /// * This method is a small variation of `autocomplete_keyword`.
    ///   This method is meant specifically for use in `Live` searches, while
    ///   the other is meant to be more general.
    fn live_search_keyword(
        search_index: &'s crate::simple::search_index::SearchIndex<K>,
        search_results: &mut BTreeSet<&'s K>,
        user_keyword: &str,
    );

    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the provided keyword and configured string similarity crate and
    /// metric.
    ///
    /// When the user's keyword that's meant to be autocompleted returns no
    /// matches, this can be used to find the best matches for substitution.
    ///
    /// # Notes
    ///
    /// * This method is a small variation of `autocomplete_context`.
    ///   This method is meant specifically for use in `Live` searches, while
    ///   the other is meant to be more general.
    fn live_search_context(
        search_index: &'s crate::simple::search_index::SearchIndex<K>,
        search_results: &std::collections::BTreeSet<&'s K>,
        preceding_keywords: &[KString],
        last_results: &mut std::collections::BTreeSet<&'s K>,
        last_keyword: &str,
    );
} // trait Fuzzy