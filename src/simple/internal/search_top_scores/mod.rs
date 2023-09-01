//! Used for tracking the top scoring keys (read: result items) for the user's
//! keywords.

mod find_bottom;
mod insert;
mod remove_bottom;
mod results;
mod with_capacity;

// -----------------------------------------------------------------------------

use std::collections::HashMap;
use std::{cmp::Ord, hash::Hash};

// -----------------------------------------------------------------------------
//
/// Tracks the top scoring keys. This is intended to track the best _n_ matches
/// for returning search results.

#[derive(Default)]
pub(crate) struct SearchTopScores<'a, K: Hash + Ord> {
    /// Tracks the top _n_ scores.
    pub(crate) top: HashMap<&'a K, usize>,
    /// Tracks lowest of the top scores.
    pub(crate) bottom: Option<(&'a K, usize)>,
    /// Number of top scores to keep.
    pub(crate) capacity: usize,
} // SearchTopScores