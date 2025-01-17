#![allow(clippy::inline_always)]

use crate::simple::EddieMetric;
use eddie::str::{
    Levenshtein,
    DamerauLevenshtein,
    Jaro,
    JaroWinkler,
};
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> crate::simple::search_index::SearchIndex<K> {
    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the provided keyword and configured string similarity metric. This
    /// feature relies on the [eddie](https://crates.io/crates/eddie)
    /// crate.
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
    ///
    /// * This method differs from `eddie_autocomplete_global_comparator` in
    ///   that this method will perform some common setup, and dynamically
    ///   dispatch to the generic method indicated by the chosen string
    ///   similarity metric (`DamerauLevenshtein`, `Jaro`, `Osa`, etc.)
    #[must_use]
    #[inline(always)]
    pub(crate) fn eddie_autocomplete_global(
        &self,
        user_keyword: &str,
    ) -> Vec<(&kstring::KString, &std::collections::BTreeSet<K>)> {
        // Build an keyword index range to fuzzy match against. This is used to
        // restrict fuzzy-matching to the strings that match the first _n_
        // characters in the user's keyword. This helps reduce required compute.
        // If a `None` is returned then no fuzzy-matching should be performed:
        let Some(index_range) = self.index_range(user_keyword) else {
            return vec![]
        };

        // If no string similarity metric was defined in the search index, fuzzy
        // string matching is effectively turned off. Return a `None` to the
        // caller:
        let Some(eddie_metric) = self.eddie_metric.as_ref() else {
            return vec![]
        };

        // Attempt to find the top matches for the user's (partial) keyword
        // using the selected string similarity metric defined in the
        // `SearchIndex`:
        match eddie_metric {
            EddieMetric::Levenshtein => self
                .eddie_autocomplete_global_comparator::<Levenshtein>(
                    &index_range,
                    user_keyword
                ).collect(),

            EddieMetric::DamerauLevenshtein => self
                .eddie_autocomplete_global_comparator::<DamerauLevenshtein>(
                    &index_range,
                    user_keyword
                ).collect(),

            EddieMetric::Jaro => self
                .eddie_autocomplete_global_comparator::<Jaro>(
                    &index_range,
                    user_keyword
                ).collect(),

            EddieMetric::JaroWinkler => self
                .eddie_autocomplete_global_comparator::<JaroWinkler>(
                    &index_range,
                    user_keyword
                ).collect(),
        } // match
    } // fn
} // impl
