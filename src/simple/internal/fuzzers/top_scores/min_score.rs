use crate::simple::internal::fuzzers::FuzzyTopScores;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord, S: PartialOrd> FuzzyTopScores<'a, K, S> {
    /// If the `FuzzyTopScores` scoreboard is full, this will return the lowest
    /// (minimum) score that must be beat to make it into the top scores.
    ///
    /// If the scoreboard isn't full, it will return a `None` because there's
    /// no score to beat in order to get in the scoreboard.
    pub(crate) fn min_score(&'a self) -> Option<&'a S> {
        if self.top.len() >= self.capacity {
            self.bottom.as_ref().map(|(_key, score)| score)
        } else {
            None
        } // if
    } // fn min_score
} // impl FuzzyTopScores