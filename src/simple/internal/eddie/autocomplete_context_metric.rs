use kstring::KString;
use std::{collections::BTreeSet, hash::Hash};

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> crate::simple::search_index::SearchIndex<K> {
    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the provided keyword and configured string similarity metric. This
    /// feature relies on the [eddie](https://crates.io/crates/eddie)
    /// crate.
    ///
    /// When the user's last (partial) keyword that is meant to be autocompleted
    /// returns no matches, this can be used to find the best match for
    /// substitution.
    ///
    /// # Input
    ///
    /// * `index_range` · Limits which keywords to compare the user's keyword
    ///   against. For example, if the `index_range` is "super" and the user's
    ///   keyword is "supersonic", only index keywords beginning with "super"
    ///   will be fuzzy compared against the user's keyword: "supersonic"
    ///   against "superalloy", "supersonic" against "supergiant" and so on...
    ///
    /// * `key_set` · A set of keys that represent the keywords preceding the
    ///   user keyword's we're autocompleting. These keys will be used to
    ///   constrain the keywords that will be examined. This is what will be
    ///   used to make the fuzzy autocompletion contextual.
    ///
    /// * `user_keyword` · Keywords most similar to this specified user keyword
    ///   will be returned.
    ///
    /// # Output
    ///
    /// * This method will return `None` if no keywords could be found. Settings
    ///   such as `fuzzy_length` and `fuzzy_minimum_score` can affect the
    ///   outcome.
    ///
    /// # Notes
    ///
    /// * `context` means that only keywords associated with the provided
    ///   key-set can be returned. This effectively makes the fuzzy
    ///   autocompletion contextual.
    ///
    /// * This method differs from `eddie_autocomplete_context` in that this
    ///   is a generic method. This method will be monomorphized for each
    ///   `eddie` string similarity metric (`DamerauLevenshtein`, `Jaro`, etc.)
    ///
    ///   `eddie_autocomplete_context` will call these monomorphized methods
    ///   using dynamic-dispatch, based on the search index's string similarity
    ///   metric settings.
    pub(crate) fn eddie_autocomplete_context_comparator<M>(
        &self,
        index_range: &str,
        key_set: &BTreeSet<&K>,
        user_keyword: &str,
    ) -> impl Iterator<Item = (&KString, &BTreeSet<K>)>
    where M: crate::simple::internal::eddie::Metric {
        // This structure will track the top scoring keywords:
        let mut top_scores =
            crate::simple::internal::FuzzyTopScores::<K, f64>::with_capacity(
                self.maximum_autocomplete_options
            );

        // Initialize eddie metric instance:
        let scorer = M::new();

        // Moved out of `filter` function from the hot loop:
        let key_set_is_empty: bool = key_set.is_empty();

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
            .filter(|(_index_keyword, index_keys)| {
                key_set_is_empty || index_keys
                    .iter()
                    .any(|index_key| key_set.contains(index_key))
            }) // filter
            // For each keyword in the search index:
            .for_each(|(index_keyword, index_keys)| {
                // Using this keyword from the search index, calculate its
                // similarity to the user's keyword:
                let score = scorer.similarity(index_keyword, user_keyword);
                // Insert the score into the top scores (if it's normal and high
                // enough):
                if score.is_normal() && score >= self.fuzzy_minimum_score {
                    top_scores.insert(index_keyword, index_keys, score);
                } // if
            }); // for_each

        // Return the top scoring keywords that could be used as autocomplete
        // options, and their keys, to the caller:
        top_scores.results()
    } // fn
} // impl
