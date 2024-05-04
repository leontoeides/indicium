use crate::simple::search_index::SearchIndex;
use crate::simple::StrsimMetric;
use kstring::KString;
use std::{collections::BTreeSet, hash::Hash};

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {
    // -------------------------------------------------------------------------
    //
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
        // Build an index keyword range to fuzzy match against.
        //
        // | Example | User Keyword                       | Length | Index Keyword Must Start With... |
        // |---------|------------------------------------|--------|----------------------------------|
        // | 1       | Supercalifragilisticexpialidocious | 2      | Su                               |
        // | 2       | Antidisestablishmentarianism       | 4      | Anti                             |
        // | 3       | Pseudopseudohypoparathyroidism     | 0      |                                  |
        //
        // * In example 1, since the length is set to `2`, the user's keyword
        // will only be fuzzy matched against keywords in the index beginning
        // with `su`.
        //
        // * In example 2, since the length is set to `4`, the user's keyword
        // will only be fuzzy matched against keywords in the index beginning
        // with `anti`.
        //
        // * In example 3, since the length is set to `0`, the user's keyword
        // will be fuzzy matched against every keyword in the index. This is OK
        // (or even desirable) if the search index isn't large, however, this
        // will be crippling slow on very large search indicies.
        let index_range: &str = if self.fuzzy_length > 0 {
            // The user keyword must be longer than the match length to be
            // evaluated for fuzzy-matches:
            if user_keyword.len() >= self.fuzzy_length {
                // Get the byte index of the _n_th character:
                let byte_index: Option<usize> = user_keyword
                    .char_indices()
                    .take(self.fuzzy_length)
                    .map(|(idx, _ch)| idx)
                    .max();
                // Use the first _n_ characters of the user's keyword. These
                // first characters are used to find search index keywords to
                // fuzzy match against:
                match byte_index {
                    Some(byte_index) => &user_keyword[0..byte_index],
                    None => return vec![],
                } // match
            } else {
                // The user's keyword is too short. Do not perform any fuzzy
                // matching:
                return vec![];
            } // if
        } else {
            // The match length is 0, compare user's keyword against all search
            // index keywords:
            ""
        }; // if

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
