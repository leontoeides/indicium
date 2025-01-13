use crate::simple::search_index::SearchIndex;
use crate::simple::StrsimMetric;
use kstring::KString;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {
    /// Scans the entire search index for the closest matching keyword using
    /// the configured string similarity metric. This feature relies on Danny
    /// Guo's [strsim](https://crates.io/crates/strsim) crate.
    ///
    /// When the user's search string contains a keyword that returns no
    /// matches, these `strsim_keyword_*` methods can be used to find the best
    /// match for substitution.
    pub(crate) fn strsim_global_keyword(&self, user_keyword: &str) -> Option<&KString> {
        // Build an keyword index range to fuzzy match against. This is used to
        // restrict fuzzy-matching to the strings that match the first _n_
        // characters in the user's keyword. This helps reduce required compute.
        // If a `None` is returned then no fuzzy-matching should be performed:
        let index_range = self.index_range(user_keyword)?

        // Attempt to find the closest match for the user's keyword using the
        // selected string similarity metric defined in the `SearchIndex`:
        #[allow(clippy::option_if_let_else)] // I hate this lint
        if let Some(strsim_metric) = &self.strsim_metric {
            match strsim_metric {
                StrsimMetric::DamerauLevenshtein => {
                    self.strsim_keyword_global_damerau_levenshtein(index_range, user_keyword)
                }

                StrsimMetric::Jaro => self.strsim_keyword_global_jaro(index_range, user_keyword),

                StrsimMetric::JaroWinkler => {
                    self.strsim_keyword_global_jaro_winkler(index_range, user_keyword)
                }

                StrsimMetric::Levenshtein => {
                    self.strsim_keyword_global_levenshtein(index_range, user_keyword)
                }

                StrsimMetric::SorensenDice => {
                    self.strsim_keyword_global_sorensen_dice(index_range, user_keyword)
                }
            } // match
        } else {
            // No string similarity metric was defined in the `SearchIndex`
            // settings. Fuzzy string matching effectively turned off.
            // Return a `None` to the caller:
            None
        } // if
    } // fn
} // impl
