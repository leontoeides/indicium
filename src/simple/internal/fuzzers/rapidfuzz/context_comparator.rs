#![allow(clippy::inline_always)]

use kstring::KString;

// -----------------------------------------------------------------------------

impl<K: std::hash::Hash + Ord> crate::simple::search_index::SearchIndex<K> {
    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the provided keyword and configured string similarity metric. This
    /// feature relies on the [rapidfuzz](https://crates.io/crates/rapidfuzz)
    /// crate.
    ///
    /// When the user's last (partial) keyword that is meant to be autocompleted
    /// returns no matches, this can be used to find the best matches for
    /// substitution.
    ///
    /// Only keywords associated with the provided `preceding_results` key-set
    /// can potentially be returned. This effectively makes any fuzzy
    /// autocompletion contextual.
    ///
    /// # Input
    ///
    /// * `preceding_keywords` · If the search string was `He who knows nothing,
    ///   loves nothing` the “preceding” keywords would be `["he", "who",
    ///   "knows", "nothing", "loves"]`.
    ///
    ///   This collection of keywords is used to prevent previously typed
    ///   keywords from being suggested. In this case, the system would _not_
    ///   suggest `nothing` as an autocomplete keyword since it's already
    ///   present in the search string.
    ///
    /// * `preceding_results` · A collection of keys for the preceding keywords.
    ///   Using the above example, these keys are the product of searching for
    ///   `he who knows nothing loves`.
    ///
    ///   These keys represent records in the search index. They're used to
    ///   constrain the keywords that are returned from this method to the
    ///   caller, and ensure that all returned keywords relate to these keys.
    ///   This is how contextual fuzzy matching is acheived.
    ///
    /// * `last_keyword` · If the search string was `He who knows nothing,
    ///   loves nothing` the “last” keyword would be `nothing`.
    ///
    ///   This keyword is used to search the search index. For example, this
    ///   could potentially return `nothingness`, and `nothings` as
    ///   autocompletion options if those words were present in the index.
    ///
    /// # Output
    ///
    /// This method returns an iterator over the top _n_ autocompletion options.
    ///
    /// Each item the returned iterator is comprised of a keyword, and the
    /// records associated with each keyword.
    ///
    /// The number of autocompletion options are defined by the
    /// `maximum_autocomplete_options` option in the search index.
    ///
    /// If no keywords or reasonable matches are found, this method will return
    /// an empty iterator.
    ///
    /// # Notes
    ///
    /// * This method differs from `rapidfuzz_context` in that this is a generic
    ///   method. This method will be monomorphized for each `rapidfuzz` string
    ///   similarity metric (`DamerauLevenshtein`, `Jaro`, `Osa`, etc.)
    ///
    ///   `rapidfuzz_context` will call these monomorphized methods using
    ///   dynamic-dispatch, based on the search index's string similarity metric
    ///   settings.
    #[inline(always)]
    pub(crate) fn rapidfuzz_context_comparator<'s, BC>(
        &'s self,
        preceding_keywords: &[KString],
        preceding_results: &std::collections::BTreeSet<&'s K>,
        last_keyword: &str,
        index_range: &str,
        top_scores: &mut crate::simple::internal::fuzzers::FuzzyTopScores::<'s, K, f64>,
    )
    where BC: crate::simple::internal::fuzzers::rapidfuzz::BatchComparator {
        // This moves a call to `is_empty` from a `filter` function, and outside
        // of a hot loop:
        let preceding_results_is_empty: bool = preceding_results.is_empty();

        // Initialize rapidfuzz's batch comparator with the user's keyword:
        let scorer = BC::new(last_keyword);

        // Scan the search index for the highest scoring keywords:
        self.b_tree_map
            // Get matching keywords starting with (partial) keyword string:
            .range(KString::from_ref(index_range)..)
            // We did not specify an end bound for our `range` function (see
            // above.) `range` will return _every_ keyword greater than the
            // supplied keyword. The below `take_while` will effectively break
            // iteration when we reach a keyword that does not start with our
            // supplied (partial) keyword.
            .take_while(|(index_keyword, _keys)|
                index_keyword.starts_with::<&str>(index_range.as_ref())
            )
            // Filter out repetitious keywords. If the keyword has already been
            // used in the preceding keywords, or if it matches what the user
            // has currently typed in (as the last partial keyword we're
            // autocompleting), don't return it as an option:
            .filter(|(index_keyword, _index_keys)|
                // For example: if the user typed "gold", and the
                // autocompletion options were "gold" and "golden", this filter
                // would remove "gold" as an autocompletion option because it's
                // currently what's already typed.
                //
                // This is commented-out because returning "gold" as an option
                // provides feedback to the user that their keyword exists in
                // the index.
                // index_keyword.as_str() != last_keyword &&

                // If the keyword is already present in the user's search
                // string, then don't suggest it as an autocompletion option:
                !preceding_keywords.contains(index_keyword)
            ) // filter
            // Only examine search index keywords that intersect with the caller
            // provided key-set. This ensures contextual fuzzy matching. This
            // will filter out search index keywords that aren't related to the
            // keys from the caller-provided `preceding_results` key set:
            .filter(|(_index_keyword, index_keys)| {
                preceding_results_is_empty || index_keys
                    .iter()
                    .any(|index_key| preceding_results.contains(index_key))
            }) // filter
            // For each keyword in the search index:
            .for_each(|(index_keyword, index_keys)|
                // Using this keyword from the search index, calculate its
                // similarity to the user's keyword.
                //
                // A `Some` is returned from `normalized_similarity` when the
                // index keyword meets the minimum score.
                //
                // If the top scores is full, then the minimum score is the
                // the bottom-most keyword's score. If the top scores isn't
                // full, then the `fuzzy_minimum_score` setting will be used:
                if let Some(score) = scorer.normalized_similarity(
                    index_keyword,
                    *top_scores.min_score().unwrap_or(&self.fuzzy_minimum_score)
                ) {
                    // Insert the score into the top scores (if it's normal):
                    top_scores.insert(index_keyword, index_keys, score);
                } // if
            ); // for_each
    } // fn
} // impl
