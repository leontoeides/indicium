use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord> crate::simple::internal::SearchTopScores<'a, K> {
    /// Attempts to insert the provided _keyword_, _keys_, & _score_ into the
    /// top scores.
    ///
    /// If the caller provided score is higher than the current lowest top
    /// score, the caller's score will be inserted into the collection. If it
    /// provided score doesn't beat the lowest top score, it will be ignored.
    pub(crate) fn insert(&mut self, key: &'a K, score: usize) {
        // Check if the `SearchTopScores` struct has reached its maximum capacity:
        if self.top.len() >= self.capacity {
            // If the `SearchTopScores` is at capacity and the lowest top score (the
            // bottom) is currently unknown, find it:
            if self.bottom.is_none() {
                self.find_bottom();
            }

            // The lowest top score should be known at this point:
            if let Some(bottom) = &self.bottom {
                // If the caller's provided score is higher than the lowest
                // top score, we have a new score:
                if score > bottom.1 {
                    // Remove the old lowest top score (or bottom) from the
                    // collection:
                    self.remove_bottom();
                    // Insert the new score into the collection:
                    self.top.insert(key, score);
                } // if
            } // if
        } else {
            // The `SearchTopScores` struct has not reached its capacity, we may
            // blindly add the _keyword_, _keys_, & _score_ without checking the
            // lowest score:
            self.top.insert(key, score);
        } // if
    } // fn insert
} // impl SearchTopScores
