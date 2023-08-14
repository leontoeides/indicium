use crate::simple::internal::TopScores;
use std::{clone::Clone, cmp::Ord, cmp::PartialOrd, hash::Hash};

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord, S: Clone + PartialOrd> TopScores<'a, K, S> {

    // -------------------------------------------------------------------------
    //
    /// Finds and caches the lowest (or bottom) top score.
    ///
    /// By caching the lowest score in the `TopScores` struct, comparisons can
    /// be done more efficiently.

    pub(crate) fn find_bottom(&mut self) {

        // Iterate over all _keyword-keys-score_ elements in the top scores:
        self.bottom = self.top
            .iter()
            // Find the lowest score in the top scores by using `min_by`.
            // Note that `min_by_key` was considered because it could be more
            // efficient but it requires `Ord` to be implemented for the `S`
            // score type which could be a floating-point number.
            .min_by(|(_a_keyword, (_a_keys, a_score)), (_b_keyword, (_b_keys, b_score))|
                a_score.partial_cmp(b_score).unwrap()
            ) // min_by
            // Remove the `keys` for the lowest score (or bottom) field since we
            // don't need them for comparisons or look-ups:
            .map(|(keyword, (_keys, score))| (*keyword, score.clone()));

    } // fn find_bottom

} // impl TopScores