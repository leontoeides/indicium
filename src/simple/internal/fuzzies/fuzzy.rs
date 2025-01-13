use std::collections::BTreeSet;

// -----------------------------------------------------------------------------
//
/// The `Fuzzy` trait allows `indicium` to treat the various string similarity
/// crates (such as `eddie`, `rapidfuzz`, `strsim`, etc.) generically.
pub trait Fuzzy<'s, K: Ord> {
    /// Scans the entire search index for the closest matching keyword.
    ///
    /// When the user's search string contains a keyword that returns no
    /// matches, this method can be used to find the best match for
    /// substitution.
    ///
    /// # Input
    ///
    /// * `keyword` · Keywords most similar to this specified user keyword will
    ///   be returned.
    ///
    /// # Output
    ///
    /// * This method will return `None` if no keywords could be found. Settings
    ///   such as `fuzzy_length` and `fuzzy_minimum_score` can affect the
    ///   outcome.
    ///
    /// # Notes
    ///
    /// * `global` means that all keywords in the search index will potentially
    ///   be examined.
    fn keyword_global(
        search_index: &'s crate::simple::search_index::SearchIndex<K>,
        keyword: &str,
    ) -> Option<&'s str>;

    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the provided keyword and configured string similarity metric.
    ///
    /// When the user's last (partial) keyword that is meant to be autocompleted
    /// returns no matches, this can be used to find the best match for
    /// substitution.
    ///
    /// # Input
    ///
    /// * `user_keyword` · Keywords most similar to this specified user keyword
    ///   will be returned.
    ///
    /// # Output
    ///
    /// * This method will return `None` if no keywords could be found. Settings
    ///   such as `fuzzy_length` and `fuzzy_minimum_score` can affect the
    ///   outcome.
    ///
    /// # Notes
    ///
    /// * `global` means that all keywords in the search index will potentially
    ///   be examined.
    fn autocomplete_global(
        search_index: &'s crate::simple::search_index::SearchIndex<K>,
        user_keyword: &str,
    ) -> Vec<(&'s kstring::KString, &'s std::collections::BTreeSet<K>)>;

    /// Scans the entire search index for the closest matching _n_ keywords.
    ///
    /// When the user's last (partial) keyword that is meant to be autocompleted
    /// returns no matches, this can be used to find the best match for
    /// substitution.
    ///
    /// # Input
    ///
    /// * `key_set` · A set of keys that represent the keywords preceding the
    ///   user keyword's we're autocompleting. These keys will be used to
    ///   constrain the keywords that will be examined. This is what will be
    ///   used to make the fuzzy autocompletion contextual.
    ///
    /// * `user_keyword` · Keywords most similar to this specified user keyword
    ///   will be returned.
    ///
    /// # Output
    ///
    /// * This method will return `None` if no keywords could be found. Settings
    ///   such as `fuzzy_length` and `fuzzy_minimum_score` can affect the
    ///   outcome.
    ///
    /// # Notes
    ///
    /// * `context` means that only keywords associated with the provided
    ///   key-set can be returned. This effectively makes the fuzzy
    ///   autocompletion contextual.
    fn autocomplete_context(
        search_index: &'s crate::simple::search_index::SearchIndex<K>,
        key_set: &BTreeSet<&K>,
        user_keyword: &str,
    ) -> Vec<(&'s kstring::KString, &'s BTreeSet<K>)>;
} // trait Fuzzy