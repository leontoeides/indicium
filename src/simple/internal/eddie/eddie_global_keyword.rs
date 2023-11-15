use crate::simple::search_index::SearchIndex;
use crate::simple::StrSimType;
use kstring::KString;
use std::{cmp::Ord, hash::Hash};

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Scans the entire search index for the closest matching keyword using
    /// the configured string similarity metric. This feature relies on Danny
    /// Guo's [strsim](https://crates.io/crates/strsim) crate.
    ///
    /// When the user's search string contains a keyword that returns no
    /// matches, these `strsim_keyword_*` methods can be used to find the best
    /// match for substitution.

    pub(crate) fn strsim_global_keyword(
        &self,
        user_keyword: &str,
    ) -> Option<&KString> {

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
        let index_range: &str = if self.strsim_length > 0 {
            // The user keyword must be longer than the match length to be
            // evaluated for fuzzy-matches:
            if user_keyword.len() >= self.strsim_length {
                // Use the first _n_ characters of the user's keyword to find
                // search index keywords to compare against:
                &user_keyword[0..self.strsim_length]
            } else {
                // The user's keyword is too short. Do not perform any fuzzy
                // matching:
                return None
            } // if
        } else {
            // The match length is 0, compare user's keyword against all search
            // index keywords:
            ""
        }; // if

        // Attempt to find the closest match for the user's keyword using the
        // selected string similarity metric defined in the `SearchIndex`:
        if let Some(strsim_type) = &self.strsim_type {

            match strsim_type {

                StrSimType::DamerauLevenshtein =>
                    self.strsim_keyword_global_damerau_levenshtein(index_range, user_keyword),

                StrSimType::Jaro =>
                    self.strsim_keyword_global_jaro(index_range, user_keyword),

                StrSimType::JaroWinkler =>
                    self.strsim_keyword_global_jaro_winkler(index_range, user_keyword),

                StrSimType::Levenshtein =>
                    self.strsim_keyword_global_levenshtein(index_range, user_keyword),

                StrSimType::SorensenDice =>
                    self.strsim_keyword_global_sorensen_dice(index_range, user_keyword),

            } // match

        } else {

            // No string similarity metric was defined in the `SearchIndex`
            // settings. Fuzzy string matching effectively turned off.
            // Return a `None` to the caller:
            None

        } // if

    } // fn

} // impl