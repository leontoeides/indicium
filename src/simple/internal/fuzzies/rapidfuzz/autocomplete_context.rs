#![allow(clippy::inline_always)]

use crate::simple::internal::fuzzies::rapidfuzz::{
    DamerauLevenshtein,
    Hamming,
    Indel,
    Jaro,
    JaroWinkler,
    LcsSeq,
    Levenshtein,
    Osa,
    Postfix,
    Prefix,
};
use crate::simple::RapidfuzzMetric;
use std::{collections::BTreeSet, hash::Hash};

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> crate::simple::search_index::SearchIndex<K> {
    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the provided keyword and configured string similarity metric. This
    /// feature relies on the [rapidfuzz](https://crates.io/crates/rapidfuzz)
    /// crate.
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
    ///
    /// * This method differs from `rapidfuzz_autocomplete_context_comparator`
    ///   in that this method will perform some common setup, and dynamically
    ///   dispatch to the generic method indicated by the chosen string
    ///   similarity metric (`DamerauLevenshtein`, `Jaro`, `Osa`, etc.)
    #[must_use]
    #[inline(always)]
    pub(crate) fn rapidfuzz_autocomplete_context<'s>(
        &'s self,
        key_set: &BTreeSet<&K>,
        user_keyword: &str,
    ) -> Box<dyn Iterator<Item = (&'s kstring::KString, &'s BTreeSet<K>)> + 's> {
        // Build an keyword index range to fuzzy match against. This is used to
        // restrict fuzzy-matching to the strings that match the first _n_
        // characters in the user's keyword. This helps reduce required compute.
        // If a `None` is returned then no fuzzy-matching should be performed:
        let Some(index_range) = self.index_range(user_keyword) else {
            return Box::new(Vec::<(&kstring::KString, &BTreeSet<K>)>::new().into_iter())
        };

        // If no string similarity metric was defined in the search index, fuzzy
        // string matching is effectively turned off. Return a `None` to the
        // caller:
        let Some(rapidfuzz_metric) = self.rapidfuzz_metric.as_ref() else {
            return Box::new(Vec::<(&kstring::KString, &BTreeSet<K>)>::new().into_iter())
        };

        // Attempt to find the top matches for the user's (partial) keyword
        // using the selected string similarity metric defined in the
        // `SearchIndex`:
        match rapidfuzz_metric {
            RapidfuzzMetric::DamerauLevenshtein => Box::new(self
                .rapidfuzz_autocomplete_context_comparator::<DamerauLevenshtein>(
                    &index_range,
                    key_set,
                    user_keyword
                )),

            RapidfuzzMetric::Hamming => Box::new(self
                .rapidfuzz_autocomplete_context_comparator::<Hamming>(
                    &index_range,
                    key_set,
                    user_keyword
                )),

            RapidfuzzMetric::Indel => Box::new(self
                .rapidfuzz_autocomplete_context_comparator::<Indel>(
                    &index_range,
                    key_set,
                    user_keyword
                )),

            RapidfuzzMetric::Jaro => Box::new(self
                .rapidfuzz_autocomplete_context_comparator::<Jaro>(
                    &index_range,
                    key_set,
                    user_keyword
                )),

            RapidfuzzMetric::JaroWinkler => Box::new(self
                .rapidfuzz_autocomplete_context_comparator::<JaroWinkler>(
                    &index_range,
                    key_set,
                    user_keyword
                )),

            RapidfuzzMetric::LcsSeq => Box::new(self
                .rapidfuzz_autocomplete_context_comparator::<LcsSeq>(
                    &index_range,
                    key_set,
                    user_keyword
                )),

            RapidfuzzMetric::Levenshtein => Box::new(self
                .rapidfuzz_autocomplete_context_comparator::<Levenshtein>(
                    &index_range,
                    key_set,
                    user_keyword
                )),

            RapidfuzzMetric::Osa => Box::new(self
                .rapidfuzz_autocomplete_context_comparator::<Osa>(
                    &index_range,
                    key_set,
                    user_keyword
                )),

            RapidfuzzMetric::Postfix => Box::new(self
                .rapidfuzz_autocomplete_context_comparator::<Postfix>(
                    &index_range,
                    key_set,
                    user_keyword
                )),

            RapidfuzzMetric::Prefix => Box::new(self
                .rapidfuzz_autocomplete_context_comparator::<Prefix>(
                    &index_range,
                    key_set,
                    user_keyword
                )),
        } // match
    } // fn
} // impl
