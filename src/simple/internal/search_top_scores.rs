//! Used for tracking the top scoring keys (read: result items) for the user's
//! keywords.

// -----------------------------------------------------------------------------

// Conditionally select hash map type based on feature flags:
#[cfg(feature = "gxhash")]
type HashMap<K, V> = std::collections::HashMap<K, V, gxhash::GxBuildHasher>;

#[cfg(feature = "ahash")]
use ahash::HashMap;

#[cfg(feature = "rustc-hash")]
use rustc_hash::FxHashMap as HashMap;

#[cfg(all(
    not(feature = "ahash"),
    not(feature = "gxhash"),
    not(feature = "rustc-hash")
))]
use std::collections::HashMap;

// Static dependencies:
use std::hash::Hash;

// -----------------------------------------------------------------------------

/// Tracks the top scoring keys for search result ranking.
///
/// This structure maintains a bounded collection of the best _n_ scoring keys,
/// automatically evicting the lowest-scoring entry when capacity is reached
/// and a better match is found. Used to efficiently collect the top-k search
/// results without storing all candidates.
#[derive(Debug, Default)]
pub struct SearchTopScores<'a, K: Hash + Ord> {
    /// The top _n_ scores, mapping keys to their scores.
    pub(crate) top: HashMap<&'a K, usize>,

    /// The lowest score currently in the top collection. Used to quickly
    /// determine if a new score qualifies for insertion without scanning.
    pub(crate) bottom: Option<(&'a K, usize)>,

    /// Maximum number of top scores to retain.
    pub(crate) capacity: usize,
}

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord> crate::simple::internal::SearchTopScores<'a, K> {
    /// Creates a new top scores tracker with the specified capacity.
    ///
    /// The capacity determines how many top-scoring keys to retain. For
    /// example, `SearchTopScores::with_capacity(10)` tracks the 10 best
    /// matching keys for returning search results.
    #[allow(clippy::default_trait_access)]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            top: HashMap::with_capacity_and_hasher(
                capacity,
                std::default::Default::default()
            ),
            bottom: None,
            capacity,
        }
    }

    /// Attempts to insert a key with its score into the top scores.
    ///
    /// * If the collection is below capacity, the entry is inserted directly.
    ///
    /// * If at capacity, the entry is only inserted if its score exceeds the
    ///   current minimum, evicting the lowest-scoring entry in the process.
    ///   Scores that don't beat the minimum are silently ignored.
    pub(crate) fn insert(&mut self, key: &'a K, score: usize) {
        let below_threshold = self.top.len() >= self.capacity && self
            .bottom
            .as_ref()
            .is_some_and(|(_, bottom_score)| score <= *bottom_score);

        if !below_threshold {
            // If at capacity, the new score beats the bottom, so remove it
            // first:
            if self.top.len() >= self.capacity {
                self.remove_bottom();
            }

            // Insert the new entry:
            self.top.insert(key, score);

            // Update bottom tracking. If this is the new minimum (or we have no
            // bottom yet), record it:
            let is_new_bottom = self
                .bottom
                .as_ref()
                .is_none_or(|(_, bottom_score)| score < *bottom_score);

            if is_new_bottom {
                self.bottom = Some((key, score));
            }
        }
    }

    /// Removes the lowest-scoring entry from the collection.
    ///
    /// After removal, performs an O(n) scan to find the new minimum. This is
    /// acceptable because removals only happen when a better score is about
    /// to be inserted, amortizing the cost across insertions.
    fn remove_bottom(&mut self) {
        // Remove the current bottom entry from the map:
        if let Some((bottom_key, _)) = &self.bottom {
            self.top.remove(bottom_key);
        }

        // Find the new bottom by scanning all remaining entries:
        self.bottom = self
            .top
            .iter()
            .min_by(|(_, a_score), (_, b_score)| a_score.cmp(b_score))
            .map(|(key, score)| (*key, *score));
    }

    /// Consumes the tracker and returns results in ranked order.
    ///
    /// Returns an iterator yielding key-score pairs, ordered by descending
    /// score. Keys with equal scores are ordered by ascending key value for
    /// deterministic output.
    pub(crate) fn results(self) -> impl Iterator<Item = (&'a K, usize)> {
        // Collect into a Vec for sorting. A BTreeMap would avoid this but
        // would require a compound key of (score, K) which complicates the API:
        let mut vec: Vec<_> = self.top.into_iter().collect();

        // Single sort with composite comparator: descending score, then
        // ascending key for deterministic tie-breaking:
        vec.sort_unstable_by(|(a_key, a_score), (b_key, b_score)| {
            b_score.cmp(a_score).then_with(|| a_key.cmp(b_key))
        });

        vec.into_iter()
    }
}