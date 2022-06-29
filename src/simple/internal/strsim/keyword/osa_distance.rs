use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;
use strsim::osa_distance;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Scans the entire search index for the closest matching keyword using
    /// the OSA (optimal string alignment) distance string similarity metric
    /// from Danny Guo's [strsim](https://crates.io/crates/strsim) crate.
    ///
    /// When the user's search string contains a keyword that returns no
    /// matches, these `strsim_keyword_*` methods can be used to find the best
    /// match for substitution.
    //
    // Note: these `strsim_keyword_*` methods are very similar and may seem
    // repetitive with a lot of boiler plate. These were intentionally made more
    // "concrete" and less modular in order to be more efficient.

    pub(crate) fn strsim_keyword_osa_distance(
        &self,
        user_keyword: &str,
    ) -> Option<&String> {

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

        // Scan the search index for the highest scoring keyword:
        self.b_tree_map
            // Get matching keywords starting with (partial) keyword string:
            .range(index_range.to_string()..)
            // We did not specify an end bound for our `range` function (see
            // above.) `range` will return _every_ keyword greater than the
            // supplied keyword. The below `take_while` will effectively break
            // iteration when we reach a keyword that does not start with our
            // supplied (partial) keyword.
            .take_while(|(index_keyword, _keys)| index_keyword.starts_with(&index_range))
            // For each keyword in the search index, calculate its similarity
            // to the user's keyword. Map the `(keyword, keys)` tuple into
            // a `(keyword, score)` tuple:
            .map(|(index_keyword, _keys)|
                (index_keyword, osa_distance(index_keyword, user_keyword))
            ) // map
            // Find the `(keyword, score)` tuple with the highest score.
            // Note that `min_by_key` was considered because it's potentially
            // more efficient. It causes difficult lifetime issues so it was
            // abandoned in favour of `min_by`.
            .min_by(|(_a_keyword, a_score), (_b_keyword, b_score)|
                a_score.cmp(b_score)
            ) // min_by
            // Return the `keyword` portion of the `(keyword, score)` tuple
            // to the caller:
            .map(|(keyword, _score)| keyword)

    } // fn

} // impl