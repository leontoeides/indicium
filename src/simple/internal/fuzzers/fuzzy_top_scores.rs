//! Used for tracking the top string similarity scores for (fuzzy matching) user
//! keywords that are not found in the search index.

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
use kstring::KString;
use std::collections::BTreeSet;
use std::hash::Hash;

// -----------------------------------------------------------------------------
//
/// Tracks the top scoring keywords for fuzzy string matching.
///
/// This structure maintains a bounded collection of the best _n_ matches,
/// automatically evicting the lowest-scoring entry when capacity is reached
/// and a better match is found. It's designed for efficient "top-k" tracking
/// during fuzzy search operations where you want to find the best matches
/// without storing all candidates.
#[derive(Default)]
pub struct FuzzyTopScores<'a, K: Hash + Ord, S: PartialOrd> {
    /// The top _n_ scores, mapping keywords to their associated keys and score.
    pub(crate) top: HashMap<&'a KString, (&'a BTreeSet<K>, S)>,

    /// The lowest score currently in the top collection. Used to quickly
    /// determine if a new score qualifies for insertion without scanning.
    pub(crate) bottom: Option<(&'a KString, S)>,

    /// Maximum number of top scores to retain.
    pub(crate) capacity: usize,
}

// -----------------------------------------------------------------------------
//
// Method Implementations

impl<'a, K: Hash + Ord, S: Clone + PartialOrd> FuzzyTopScores<'a, K, S> {
    /// Creates a new top scores tracker with the specified capacity.
    ///
    /// The capacity determines how many top-scoring matches to retain. For
    /// example, `FuzzyTopScores::with_capacity(10)` tracks the 10 best matches
    /// for a user-provided keyword.
    #[allow(clippy::default_trait_access)]
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        FuzzyTopScores {
            top: HashMap::with_capacity_and_hasher(
                capacity,
                std::default::Default::default()
            ),
            bottom: None,
            capacity,
        }
    }

    /// Attempts to insert a keyword with its associated keys and score.
    ///
    /// * If the collection is below capacity, the entry is inserted directly.
    ///
    /// * If at capacity, the entry is only inserted if its score exceeds the
    ///   current minimum, evicting the lowest-scoring entry in the process.
    ///   Scores that don't beat the minimum are silently ignored.
    pub(crate) fn insert(
        &mut self,
        keyword: &'a KString,
        keys: &'a BTreeSet<K>, score: S
    ) {
        // If we're at capacity, only insert if the score beats the current
        // bottom:
        if self.top.len() >= self.capacity {
            let dominated_by_bottom = self
                .bottom
                .as_ref()
                .is_some_and(|(_, bottom_score)| score <= *bottom_score);

            if dominated_by_bottom {
                // Score doesn't qualify for the top scores:
                return;
            }

            // New score beats the bottom. Remove the old bottom first:
            self.remove_bottom();
        }

        // Insert the new entry:
        self.top.insert(keyword, (keys, score.clone()));

        // Update bottom tracking. If this is the new minimum (or we have no
        // bottom yet), record it:
        let dominated_by_new_score = self
            .bottom
            .as_ref()
            .is_none_or(|(_, bottom_score)| score < *bottom_score);

        if dominated_by_new_score {
            self.bottom = Some((keyword, score));
        }
    }

    /// Returns the minimum score needed to enter the top scores, if at
    /// capacity.
    ///
    /// Returns `None` if the collection isn't full yet (any score qualifies).
    /// This is useful for setting cutoff thresholds in similarity algorithms to
    /// avoid computing scores that can't possibly qualify.
    #[cfg(feature = "rapidfuzz")]
    pub(crate) fn min_score(&self) -> Option<&S> {
        if self.top.len() >= self.capacity {
            self.bottom.as_ref().map(|(_, score)| score)
        } else {
            None
        }
    }

    /// Removes the lowest-scoring entry from the collection.
    ///
    /// After removal, performs an O(n) scan to find the new minimum. This is
    /// acceptable because removals only happen when a better score is about to
    /// be inserted, amortizing the cost across insertions.
    fn remove_bottom(&mut self) {
        // Remove the current bottom entry from the map:
        if let Some((bottom_keyword, _)) = &self.bottom {
            self.top.remove(bottom_keyword);
        }

        // Find the new bottom by scanning all remaining entries:
        self.bottom = self
            .top
            .iter()
            .min_by(|(_, (_, a_score)), (_, (_, b_score))| {
                a_score.partial_cmp(b_score).unwrap()
            })
            .map(|(keyword, (_, score))| (*keyword, score.clone()));
    }

    /// Consumes the tracker and returns results in descending score order.
    ///
    /// Returns an iterator yielding keyword-keys pairs, ordered from highest
    /// to lowest score. The scores themselves are discarded since callers
    /// typically only need the ranked results.
    #[inline]
    pub(crate) fn results(
        self
    ) -> impl Iterator<Item = (&'a KString, &'a BTreeSet<K>)> {
        // Collect into a Vec for sorting. A BTreeMap would avoid this but
        // would require `S: Ord`, which excludes floating-point scores:
        let mut vec: Vec<_> = self.top.into_iter().collect();

        // Sort by descending score:
        vec.sort_unstable_by(|(_, (_, a_score)), (_, (_, b_score))| {
            b_score.partial_cmp(a_score).unwrap()
        });

        // Yield only the keyword and keys:
        vec.into_iter().map(|(keyword, (keys, _))| (keyword, keys))
    }
}