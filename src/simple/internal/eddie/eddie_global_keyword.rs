use crate::simple::search_index::SearchIndex;
use crate::simple::EddieMetric;
use kstring::KString;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {
    /// Scans the entire search index for the closest matching keyword using
    /// the configured string similarity metric. This feature relies on Ilia
    /// Schelokov's [eddie](https://crates.io/crates/eddie) crate.
    ///
    /// When the user's search string contains a keyword that returns no
    /// matches, these `eddie_keyword_*` methods can be used to find the best
    /// match for substitution.
    pub(crate) fn eddie_global_keyword(
        &self,
        user_keyword: &str
    ) -> Option<&KString> {
        // Build an keyword index range to fuzzy match against. This is used to
        // restrict fuzzy-matching to the strings that match the first _n_
        // characters in the user's keyword. This helps reduce required compute.
        // If a `None` is returned then no fuzzy-matching should be performed:
        let Some(index_range) = self.index_range(user_keyword) else {
            return vec![]
        };

        // Attempt to find the closest match for the user's keyword using the
        // selected string similarity metric defined in the `SearchIndex`:
        self.eddie_metric
            .as_ref()
            .and_then(|eddie_metric| match eddie_metric {
                EddieMetric::DamerauLevenshtein => {
                    self.eddie_keyword_global_damerau_levenshtein(index_range, user_keyword)
                }

                EddieMetric::Jaro => self.eddie_keyword_global_jaro(index_range, user_keyword),

                EddieMetric::JaroWinkler => {
                    self.eddie_keyword_global_jaro_winkler(index_range, user_keyword)
                }

                EddieMetric::Levenshtein => {
                    self.eddie_keyword_global_levenshtein(index_range, user_keyword)
                }
            }) // map_or
    } // fn
} // impl
