use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;
use strsim::jaro;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Scans the entire search index for the closest matching keyword using
    /// the Jaro string similarity metric from Danny Guo's
    /// [strsim](https://crates.io/crates/strsim) crate.
    ///
    /// When the user's search string contains a keyword that returns no
    /// matches, these `strsim_keyword_*` methods can be used to find the best
    /// match for substitution.

    pub(crate) fn strsim_keyword_jaro(
        &self,
        user_keyword: &str,
    ) -> Option<&String> {

        // Scan the search index for the highest scoring keyword:
        self.b_tree_map
            // Iterate over all keywords and their keys:
            .iter()
            // For each keyword in the search index, calculate its similarity
            // to the user's keyword. Map the `(keyword, keys)` tuple into
            // a `(keyword, score)` tuple:
            .map(|(index_keyword, _keys)|
                (index_keyword, jaro(index_keyword, user_keyword))
            ) // map
            // Find the `(keyword, score)` tuple with the highest score:
            .max_by(|(_a_keyword, a_score), (_b_keyword, b_score)|
                a_score.partial_cmp(b_score).unwrap()
            ) // max_by
            // Return the `keyword` portion of the `(keyword, score)` tuple
            // to the caller:
            .map(|(keyword, _score)| keyword)

    } // fn

} // impl