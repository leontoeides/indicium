use crate::simple::internal::OrTopScores;
use std::{cmp::Ord, collections::HashMap, hash::Hash};

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord> OrTopScores<'a, K> {

    // -------------------------------------------------------------------------
    //
    /// Instantiates a new "top scores" struct with the caller provided
    /// capacity. If the caller wants to track the "top 10 matches" for a user
    /// provided keyword, the caller would call `OrTopScores::with_capacity(10)`.

    pub(crate) fn with_capacity(capacity: usize) -> OrTopScores<'a, K> {

        OrTopScores {
            top: HashMap::with_capacity(capacity),
            bottom: None,
            capacity,
        } // OrTopScores

    } // fn with_capacity

} // impl OrTopScores