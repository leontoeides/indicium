use crate::simple::internal::TopScores;
use std::collections::BTreeSet;
use std::{cmp::Ord, cmp::PartialOrd, hash::Hash};

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord, S: PartialOrd> TopScores<'a, K, S> {

    // -------------------------------------------------------------------------
    //
    /// Returns the keys associated with the top scoring keywords as results.
    /// The ordering of keys is determining by the `Ord` implementation of `K`.

    pub(crate) fn keys(self) -> BTreeSet<&'a K> {

        // Iterate over the top scoring keywords, and flatten all the keys for
        // the top scoring keywords into a `BTreeSet`.
        self.top
            .into_iter()
            .flat_map(|(_keyword, (keys, _score))| keys)
            .collect()

    } // if keys

} // impl TopScores