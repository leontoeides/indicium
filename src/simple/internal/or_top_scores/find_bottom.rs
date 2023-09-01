use crate::simple::internal::OrTopScores;
use std::{cmp::Ord, cmp::PartialOrd, hash::Hash};

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord> OrTopScores<'a, K> {

    // -------------------------------------------------------------------------
    //
    /// Finds and caches the lowest (or bottom) top score.
    ///
    /// By caching the lowest score in the `OrTopScores` struct, comparisons can
    /// be done more efficiently.

    pub(crate) fn find_bottom(&mut self) {

        // Iterate over all _key-keys-score_ elements in the top scores:
        self.bottom = self.top
            .iter()
            // Find the lowest score in the top scores by using `min_by`.
            // Note that `min_by_key` was considered because it could be more
            // efficient but it requires `Ord` to be implemented for the `S`
            // score type which could be a floating-point number.
            .min_by(|(_a_key, a_score), (_b_key, b_score)|
                a_score.partial_cmp(b_score).unwrap()
            ) // min_by
            // Remove the `keys` for the lowest score (or bottom) field since we
            // don't need them for comparisons or look-ups:
            .map(|(key, score)| (*key, *score));

    } // fn find_bottom

} // impl OrTopScores