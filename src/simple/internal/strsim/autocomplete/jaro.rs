use crate::simple::internal::TopScores;
use crate::simple::search_index::SearchIndex;
use std::{cmp::Ord, hash::Hash};
use strsim::jaro;

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the Jaro string similarity metric from Danny Guo's
    /// [strsim](https://crates.io/crates/strsim) crate.
    ///
    /// When the user's last (partial) keyword that is meant to be autocompleted
    /// returns no matches, these `strsim_autocomplete_*` methods can be used to
    /// find the best match for substitution.
    //
    // Note: these `strsim_autocomplete_*` methods are very similar and may seem
    // repetitive with a lot of boiler plate. These were intentionally made more
    // "concrete" and less modular in order to be more efficient.

    pub(crate) fn strsim_autocomplete_jaro(
        &self,
        user_keyword: &str,
    ) -> Vec<&str> {

        // Build an index keyword range to fuzzy match against.
        //
        // | Example | User Keyword                       | Length | Index Keyword Must Start With... |
        // |---------|------------------------------------|--------|----------------------------------|
        // | 1       | Supercalifragilisticexpialidocious | 2      |  Su                              |
        // | 2       | Antidisestablishmentarianism       | 4      |  Anti                            |
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
        // or even desirable if the search index isn't large, however, this will
        // be crippling slow on very large search indicies.
        let index_range: &str = if self.strsim_length > 0 {
            &user_keyword[0..self.strsim_length]
        } else {
            ""
        }; // if

        // This structure will track the top scoring keywords:
        let mut top_scores: TopScores<K, f64> =
            TopScores::with_capacity(self.maximum_autocomplete_options);

        // Scan the search index for the highest scoring keywords:
        self.b_tree_map
            // Get matching keywords starting with (partial) keyword string:
            .range(index_range.to_string()..)
            // We did not specify an end bound for our `range` function (see
            // above.) `range` will return _every_ keyword greater than the
            // supplied keyword. The below `take_while` will effectively break
            // iteration when we reach a keyword that does not start with our
            // supplied (partial) keyword.
            .take_while(|(index_keyword, _keys)| index_keyword.starts_with(&index_range))
            // For each keyword in the search index:
            .for_each(|(index_keyword, index_keys)| {
                // Using this keyword from the search index, calculate its
                // similarity to the user's keyword:
                let score = jaro(index_keyword, user_keyword);
                // Insert the score into the top scores (if it's normal and high
                // enough):
                if score.is_normal() {
                    top_scores.insert(index_keyword, index_keys, score)
                } // if
            }); // for_each

        // Return the top scoring keywords that could be used as autocomplete
        // options to the caller:
        top_scores.keywords()

    } // fn

} // impl