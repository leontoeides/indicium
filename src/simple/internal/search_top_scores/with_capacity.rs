use ahash::{HashMap, HashMapExt};
use crate::simple::internal::SearchTopScores;
use std::{cmp::Ord, hash::Hash};

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord> SearchTopScores<'a, K> {

    // -------------------------------------------------------------------------
    //
    /// Instantiates a new "top scores" struct with the caller provided
    /// capacity. If the caller wants to track the "top 10 matches" for a user
    /// provided keyword, the caller would call `SearchTopScores::with_capacity(10)`.

    pub(crate) fn with_capacity(capacity: usize) -> SearchTopScores<'a, K> {

        SearchTopScores {
            top: HashMap::with_capacity(capacity),
            bottom: None,
            capacity,
        } // SearchTopScores

    } // fn with_capacity

} // impl SearchTopScores