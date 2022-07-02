mod find_bottom;
mod insert;
mod keys;
mod keywords;
mod results;
mod remove_bottom;
mod with_capacity;

// -----------------------------------------------------------------------------

use std::collections::{BTreeSet, HashMap};
use std::{cmp::Ord, cmp::PartialOrd, hash::Hash};

// -----------------------------------------------------------------------------
//
/// Tracks the top scoring keywords. This is intended to track the best _n_
/// matches for fuzzy string matching.

#[derive(Default)]
pub(crate) struct TopScores<'a, K: Hash + Ord, S: PartialOrd> {
    /// Tracks the top _n_ scores.
    pub(crate) top: HashMap<&'a String, (&'a BTreeSet<K>, S)>,
    /// Tracks lowest of the top scores.
    pub(crate) bottom: Option<(&'a String, S)>,
    /// Number of top scores to keep.
    pub(crate) capacity: usize,
} // TopScores