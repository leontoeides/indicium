use crate::simple::internal::FuzzyTopScores;
use kstring::KString;
use std::{collections::BTreeSet, hash::Hash};

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord, S: Clone + PartialOrd> FuzzyTopScores<'a, K, S> {
    // -----------------------------------------------------------------------------
    //
    /// Attempts to insert the provided _keyword_, _keys_, & _score_ into the
    /// top scores.
    ///
    /// If the caller provided score is higher than the current lowest top
    /// score, the caller's score will be inserted into the collection. If it
    /// provided score doesn't beat the lowest top score, it will be ignored.
    pub(crate) fn insert(&mut self, keyword: &'a KString, keys: &'a BTreeSet<K>, score: S) {
        // Check if the `FuzzyTopScores` struct has reached its maximum capacity:
        if self.top.len() >= self.capacity {
            // If the `FuzzyTopScores` is at capacity and the lowest top score
            // (the bottom) is currently unknown, find it:
            if self.bottom.is_none() {
                self.find_bottom();
            } // if

            // The lowest top score should be known at this point:
            if let Some(bottom) = &self.bottom {
                // If the caller's provided score is higher than the lowest
                // top score, we have a new score:
                if score > bottom.1 {
                    // Remove the old lowest top score (or bottom) from the
                    // collection:
                    self.remove_bottom();
                    // Insert the new score into the collection:
                    self.top.insert(keyword, (keys, score));
                } // if
            } // if
        } else {
            // The `FuzzyTopScores` struct has not reached its capacity, we may
            // blindly add the _keyword_, _keys_, & _score_ without checking the
            // lowest score:
            self.top.insert(keyword, (keys, score));
        } // if
    } // fn insert
} // impl FuzzyTopScores
