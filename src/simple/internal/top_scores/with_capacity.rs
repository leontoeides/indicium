use crate::simple::internal::TopScores;
use std::{cmp::Ord, cmp::PartialOrd, collections::HashMap, hash::Hash};

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord, S: PartialOrd> TopScores<'a, K, S> {

    // -------------------------------------------------------------------------
    //
    /// Instantiates a new "top scores" struct with the caller provided
    /// capacity. If the caller wants to track the "top 10 matches" for a user
    /// provided keyword, the caller would call `TopScores::with_capacity(10)`.

    pub(crate) fn with_capacity(capacity: usize) -> TopScores<'a, K, S> {

        TopScores {
            top: HashMap::with_capacity(capacity),
            bottom: None,
            capacity,
        } // TopScores

    } // fn with_capacity

} // impl TopScores