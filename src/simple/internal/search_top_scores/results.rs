use crate::simple::internal::SearchTopScores;
use std::{hash::Hash};

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord> SearchTopScores<'a, K> {
    // -------------------------------------------------------------------------
    //
    /// Returns the top scoring keywords with their keys, in order of descending
    /// score.

    pub(crate) fn results(self) -> impl Iterator<Item = (&'a K, usize)> {
        // Dump the contents of the `HashMap` so that the top scores can be
        // sorted:
        //
        // Note: a sort could be avoided by using a `BTreeMap` to track the top
        // scores. However, that would require the score to implement `Ord` and
        // we need to accept floating-point scores from the `strsim` crate.
        let mut vec: Vec<(&K, usize)> = self.top.into_iter().collect();

        // Sort so that tied scores are in order of key, ascending:
        vec.sort_unstable_by(|a, b| a.0.cmp(b.0));

        // Sort the keywords in order of descending score:
        vec.sort_by(|a, b| b.1.cmp(&a.1));

        // Return the keywords and keys to the caller:
        vec.into_iter()
    } // if keywords
} // impl SearchTopScores
