use crate::simple::internal::TopScores;
use crate::simple::search_index::SearchIndex;
use std::{cmp::Ord, hash::Hash};
use strsim::levenshtein;

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the Levenshtein string similarity metric from Danny Guo's
    /// [strsim](https://crates.io/crates/strsim) crate.
    ///
    /// When the user's last (partial) keyword that is meant to be autocompleted
    /// returns no matches, these `strsim_autocomplete_*` methods can be used to
    /// find the best match for substitution.

    pub(crate) fn strsim_autocomplete_levenshtein(
        &self,
        user_keyword: &str,
    ) -> Vec<&str> {

        // This structure will track the top scoring keywords:
        let mut top_scores: TopScores<K, usize> =
            TopScores::with_capacity(self.maximum_autocomplete_options);

        // Scan the search index for the highest scoring keywords:
        self.b_tree_map
            // Iterate over all keywords and their keys:
            .iter()
            // For each keyword in the search index:
            .for_each(|(index_keyword, index_keys)| {
                // Using this keyword from the search index, calculate its
                // similarity to the user's keyword:
                let score = levenshtein(index_keyword, user_keyword);
                // Insert the score into the top scores (if it's a high enough):
                top_scores.insert(index_keyword, index_keys, score)
            }); // for_each

        // Return the top scoring keywords that could be used as autocomplete
        // options to the caller:
        top_scores.keywords()

    } // fn

} // impl