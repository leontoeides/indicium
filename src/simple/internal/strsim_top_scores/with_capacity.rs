use crate::simple::internal::StrsimTopScores;
use std::{cmp::Ord, cmp::PartialOrd, collections::HashMap, hash::Hash};

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord, S: PartialOrd> StrsimTopScores<'a, K, S> {

    // -------------------------------------------------------------------------
    //
    /// Instantiates a new "top scores" struct with the caller provided
    /// capacity. If the caller wants to track the "top 10 matches" for a user
    /// provided keyword, the caller would call `StrsimTopScores::with_capacity(10)`.

    pub(crate) fn with_capacity(capacity: usize) -> StrsimTopScores<'a, K, S> {

        StrsimTopScores {
            top: HashMap::with_capacity(capacity),
            bottom: None,
            capacity,
        } // StrsimTopScores

    } // fn with_capacity

} // impl StrsimTopScores