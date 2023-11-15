use crate::simple::internal::StrsimTopScores;
use crate::simple::search_index::SearchIndex;
use kstring::KString;
use std::{cmp::Ord, collections::BTreeSet, hash::Hash};
use strsim::jaro_winkler;

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the Jaro-Winkler string similarity metric from Danny Guo's
    /// [strsim](https://crates.io/crates/strsim) crate.
    ///
    /// When the user's last (partial) keyword that is meant to be autocompleted
    /// returns no matches, these `strsim_autocomplete_*` methods can be used to
    /// find the best match for substitution.
    ///
    /// * `index_range` limits which keywords to compare the user's keyword
    /// against. For example, if the `index_range` is "super" and the user's
    /// keyword is "supersonic": only search index keywords beginning with
    /// "super" will be compared against the user's keyword: "supersonic"
    /// against "superalloy", "supersonic" against "supergiant" and so on...
    ///
    /// * `key_set` limits which keywords to compare the user's keyword
    /// against. For a search index keyword to be considered as a fuzzy match,
    /// it must contain at least one key that is in this key set. This is how
    /// fuzzy matching is made contextual.
    //
    // Note: these `strsim_autocomplete_*` methods are very similar and may seem
    // repetitive with a lot of boiler plate. These were intentionally made more
    // "concrete" and less modular in order to be more efficient.

    pub(crate) fn strsim_autocomplete_context_jaro_winkler(
        &self,
        index_range: &str,
        key_set: &BTreeSet<&K>,
        user_keyword: &str,
    ) -> impl Iterator<Item = (&KString, &BTreeSet<K>)> {

        // This structure will track the top scoring keywords:
        let mut top_scores: StrsimTopScores<K, f64> =
            StrsimTopScores::with_capacity(self.maximum_autocomplete_options);

        // Scan the search index for the highest scoring keywords:
        self.b_tree_map
            // Get matching keywords starting with (partial) keyword string:
            .range(KString::from_ref(index_range)..)
            // We did not specify an end bound for our `range` function (see
            // above.) `range` will return _every_ keyword greater than the
            // supplied keyword. The below `take_while` will effectively break
            // iteration when we reach a keyword that does not start with our
            // supplied (partial) keyword.
            .take_while(|(index_keyword, _keys)| index_keyword.starts_with(index_range))
            // Only examine search index keywords that intersect with the caller
            // provided key-set. This ensures contextual fuzzy matching. This
            // will filter out search index keywords that don't contain any keys
            // from the caller provided key set:
            .filter(|(_index_keyword, index_keys)|
                key_set.is_empty() ||
                    index_keys.iter().any(|index_key| key_set.contains(index_key))
            ) // filter
            // For each keyword in the search index:
            .for_each(|(index_keyword, index_keys)| {
                // Using this keyword from the search index, calculate its
                // similarity to the user's keyword:
                let score = jaro_winkler(index_keyword, user_keyword);
                // Insert the score into the top scores (if it's normal and high
                // enough):
                if score.is_normal() && score >= self.fuzzy_minimum_score {
                    top_scores.insert(index_keyword, index_keys, score)
                } // if
            }); // for_each

        // Return the top scoring keywords that could be used as autocomplete
        // options, and their keys, to the caller:
        top_scores.results()

    } // fn

} // impl