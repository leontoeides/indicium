//! Used for tracking the top scoring keys (read: result items) for the user's
//! keywords.

mod find_bottom;
mod insert;
mod remove_bottom;
mod results;
mod with_capacity;

// -----------------------------------------------------------------------------

// Conditionally select hash map type based on feature flags:
#[cfg(feature = "gxhash")]
type HashMap<K, V> = std::collections::HashMap<K, V, gxhash::GxBuildHasher>;
#[cfg(all(feature = "ahash", not(feature = "gxhash")))]
use ahash::HashMap;
#[cfg(all(not(feature = "ahash"), not(feature = "gxhash")))]
use std::collections::HashMap;

// Static dependencies:
use std::{hash::Hash};

// -----------------------------------------------------------------------------
//
/// Tracks the top scoring keys. This is intended to track the best _n_ matches
/// for returning search results.

#[derive(Debug, Default)]
pub struct SearchTopScores<'a, K: Hash + Ord> {
    /// Tracks the top _n_ scores.
    pub(crate) top: HashMap<&'a K, usize>,
    /// Tracks lowest of the top scores.
    pub(crate) bottom: Option<(&'a K, usize)>,
    /// Number of top scores to keep.
    pub(crate) capacity: usize,
} // SearchTopScores
