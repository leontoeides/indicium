use crate::simple::internal::TopScores;
use std::{cmp::Ord, cmp::PartialOrd, collections::BTreeSet, hash::Hash};

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord, S: PartialOrd> TopScores<'a, K, S> {

    // -------------------------------------------------------------------------
    //
    /// Returns the top scoring keywords with their keys, in order of descending
    /// score.

    pub(crate) fn results(self) -> Vec<(&'a String, &'a BTreeSet<K>)> {

        // Dump the contents of the `HashMap` so that the top scores can be
        // sorted:
        //
        // Note: a sort could be avoided by using a `BTreeMap` to track the top
        // scores. However, that would require the score to implement `Ord` and
        // we need to accept floating-point scores from the `strsim` crate.
        let mut vec: Vec<(&String, (&BTreeSet<K>, S))> = self.top
            .into_iter()
            .collect();

        // Sort the keywords in order of descending score:
        vec.sort_unstable_by(|a, b| b.1.1.partial_cmp(&a.1.1).unwrap());

        // Return the keywords and keys to the caller:
        vec
            .into_iter()
            .map(|(keyword, (keys, _score))| (keyword, keys))
            .collect()

    } // if keywords

} // impl TopScores