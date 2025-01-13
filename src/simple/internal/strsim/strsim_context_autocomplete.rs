use crate::simple::search_index::SearchIndex;
use crate::simple::StrsimMetric;
use kstring::KString;
use std::{collections::BTreeSet, hash::Hash};

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {
    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the provided keyword, key set, and configured string similarity
    /// metric. This feature relies on Danny Guo's
    /// [strsim](https://crates.io/crates/strsim) crate.
    ///
    /// When the user's last (partial) keyword that is meant to be autocompleted
    /// returns no matches, these `strsim_autocomplete_*` methods can be used to
    /// find the best match for substitution.
    pub(crate) fn strsim_context_autocomplete(
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
        #[allow(clippy::option_if_let_else)] // This lint makes things less readable 👎
        if let Some(strsim_metric) = &self.strsim_metric {
            match strsim_metric {
                StrsimMetric::DamerauLevenshtein => self
                    .strsim_autocomplete_context_damerau_levenshtein(
                        index_range,
                        key_set,
                        user_keyword,
                    )
                    .collect(),

                StrsimMetric::Jaro => self
                    .strsim_autocomplete_context_jaro(index_range, key_set, user_keyword)
                    .collect(),

                StrsimMetric::JaroWinkler => self
                    .strsim_autocomplete_context_jaro_winkler(index_range, key_set, user_keyword)
                    .collect(),

                StrsimMetric::Levenshtein => self
                    .strsim_autocomplete_context_levenshtein(index_range, key_set, user_keyword)
                    .collect(),

                StrsimMetric::SorensenDice => self
                    .strsim_autocomplete_context_sorensen_dice(index_range, key_set, user_keyword)
                    .collect(),
            } // match
        } else {
            // No string similarity metric was defined in the `SearchIndex`
            // settings. Fuzzy string matching effectively turned off.
            // Return an empty `Vec` to the caller:
            vec![]
        } // if
    } // fn
} // impl
