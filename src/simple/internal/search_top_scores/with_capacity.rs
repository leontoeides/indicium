// Conditionally select hash map type based on feature flags:
#[cfg(feature = "gxhash")]
type HashMap<K, V> = std::collections::HashMap<K, V, gxhash::GxBuildHasher>;
#[cfg(all(feature = "ahash", not(feature = "gxhash")))]
use ahash::HashMap;
#[cfg(all(not(feature = "ahash"), not(feature = "gxhash")))]
use std::collections::HashMap;

// Static dependencies:
use crate::simple::internal::SearchTopScores;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord> SearchTopScores<'a, K> {
    // -------------------------------------------------------------------------
    #![allow(clippy::default_trait_access)]
    /// Instantiates a new "top scores" struct with the caller provided
    /// capacity. If the caller wants to track the "top 10 matches" for a user
    /// provided keyword, the caller would call `SearchTopScores::with_capacity(10)`.

    pub(crate) fn with_capacity(capacity: usize) -> Self {
        SearchTopScores {
            top: HashMap::with_capacity_and_hasher(capacity, std::default::Default::default()),
            bottom: None,
            capacity,
        } // SearchTopScores
    } // fn with_capacity
} // impl SearchTopScores
