use crate::simple::search_index::SearchIndex;
use crate::simple::StrSimType;
use std::{cmp::Ord, hash::Hash};

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the configured string similarity metric. This feature relies on
    /// Danny Guo's [strsim](https://crates.io/crates/strsim) crate.
    ///
    /// When the user's last (partial) keyword that is meant to be autocompleted
    /// returns no matches, these `strsim_autocomplete_*` methods can be used to
    /// find the best match for substitution.

    pub fn strsim_autocomplete(
        &self,
        user_autocomplete: &str,
    ) -> Vec<&str> {

        if let Some(strsim_type) = &self.strsim_type {
            match strsim_type {

                StrSimType::DamerauLevenshtein =>
                    self.strsim_autocomplete_damerau_levenshtein(user_autocomplete),

                StrSimType::Jaro =>
                    self.strsim_autocomplete_jaro(user_autocomplete),

                StrSimType::JaroWinkler =>
                    self.strsim_autocomplete_jaro_winkler(user_autocomplete),

                StrSimType::Levenshtein =>
                    self.strsim_autocomplete_levenshtein(user_autocomplete),

                StrSimType::OsaDistance =>
                    self.strsim_autocomplete_osa_distance(user_autocomplete),

                StrSimType::SorensenDice =>
                    self.strsim_autocomplete_sorensen_dice(user_autocomplete),

            } // match
        } else {
            vec![]
        } // if

    } // fn

} // impl