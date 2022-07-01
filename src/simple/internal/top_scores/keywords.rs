use crate::simple::internal::TopScores;
use std::{cmp::Ord, cmp::PartialOrd, hash::Hash};

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord, S: PartialOrd> TopScores<'a, K, S> {

    // -------------------------------------------------------------------------
    //
    /// Returns the top scoring keywords as results, in descending order.

    pub(crate) fn keywords(self) -> Vec<&'a str> {

        // Dump the contents of the `HashMap` so that the top scores can be
        // sorted:
        // Note: a sort could be avoided by using a `BTreeMap` to track the top
        // scores. However, that would require the score to implement `Ord` and
        // we need to accept floating-point scores.
        let mut vec: Vec<(&str, S)> = self.top
            .into_iter()
            .map(|(keyword, (_keys, score))| (keyword, score))
            .collect();

        // Sort the keywords in descending order of score:
        vec.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Return the keywords to the caller:
        vec.into_iter().map(|(keyword, _score)| keyword).collect()

    } // if keywords

} // impl TopScores