use crate::simple::search_index::SearchIndex;
use crate::simple::EddieMetric;
use kstring::KString;
use std::{collections::BTreeSet, hash::Hash};

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {
    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the provided keyword, key set, and configured string similarity
    /// metric. This feature relies on Ilia Schelokov's
    /// [eddie](https://crates.io/crates/eddie) crate.
    ///
    /// When the user's last (partial) keyword that is meant to be autocompleted
    /// returns no matches, these `eddie_autocomplete_*` methods can be used to
    /// find the best match for substitution.
    pub(crate) fn eddie_context_autocomplete(
        &self,
        key_set: &BTreeSet<&K>,
        user_keyword: &str,
    ) -> Vec<(&KString, &BTreeSet<K>)> {
        // Build an keyword index range to fuzzy match against. This is used to
        // restrict fuzzy-matching to the strings that match the first _n_
        // characters in the user's keyword. This helps reduce required compute.
        // If a `None` is returned then no fuzzy-matching should be performed:
        let Some(index_range) = self.index_range(user_keyword) else {
            return vec![]
        };

        // Attempt to find the top matches for the user's (partial) keyword
        // using the selected string similarity metric defined in the
        // `SearchIndex`:
        self.eddie_metric
            .as_ref()
            .map_or_else(Vec::new, |eddie_metric| match eddie_metric {
                EddieMetric::DamerauLevenshtein => self
                    .eddie_autocomplete_context_damerau_levenshtein(
                        index_range,
                        key_set,
                        user_keyword,
                    )
                    .collect(),

                EddieMetric::Jaro => self
                    .eddie_autocomplete_context_jaro(index_range, key_set, user_keyword)
                    .collect(),

                EddieMetric::JaroWinkler => self
                    .eddie_autocomplete_context_jaro_winkler(index_range, key_set, user_keyword)
                    .collect(),

                EddieMetric::Levenshtein => self
                    .eddie_autocomplete_context_levenshtein(index_range, key_set, user_keyword)
                    .collect(),
            }) // map_or_else
    } // fn
} // impl
