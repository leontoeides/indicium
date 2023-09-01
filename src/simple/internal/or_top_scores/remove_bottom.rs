use crate::simple::internal::OrTopScores;
use std::{cmp::Ord, hash::Hash};

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord> OrTopScores<'a, K> {

    // -------------------------------------------------------------------------
    //
    /// Removes the lowest top score from the list. This is normally done before
    /// replacing it with a higher score.

    pub(crate) fn remove_bottom(&mut self) {

        // Remove the lowest top score from the collection:
        if let Some(bottom) = &self.bottom { self.top.remove(&bottom.0); }

        // Remove the score from the lowest top score tracker. This will be
        // re-populated on the next `insert` call:
        self.bottom = None;

    } // fn remove_bottom

} // impl OrTopScores