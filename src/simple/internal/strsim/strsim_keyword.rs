use crate::simple::search_index::SearchIndex;
use crate::simple::StrSimType;
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

    pub(crate) fn strsim_keyword(
        &self,
        user_keyword: &str,
    ) -> Option<&String> {

        if let Some(strsim_type) = &self.strsim_type {
            match strsim_type {
                StrSimType::DamerauLevenshtein => self.strsim_keyword_damerau_levenshtein(user_keyword),
                StrSimType::Jaro => self.strsim_keyword_jaro(user_keyword),
                StrSimType::JaroWinkler => self.strsim_keyword_jaro_winkler(user_keyword),
                StrSimType::Levenshtein => self.strsim_keyword_levenshtein(user_keyword),
                StrSimType::OsaDistance => self.strsim_keyword_osa_distance(user_keyword),
                StrSimType::SorensenDice => self.strsim_keyword_sorensen_dice(user_keyword),
            } // match
        } else {
            None
        } // if

    } // fn

} // impl