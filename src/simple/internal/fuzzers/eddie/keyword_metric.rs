#![allow(clippy::inline_always)]

// -----------------------------------------------------------------------------

impl<K: std::hash::Hash + Ord> crate::simple::search_index::SearchIndex<K> {
    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the provided keyword and configured string similarity metric. This
    /// feature relies on the [eddie](https://crates.io/crates/eddie) crate.
    ///
    /// When the user's keyword that is meant to be autocompleted returns no
    /// matches, this can be used to find the best matches for substitution.
    ///
    /// All keywords in the search index will potentially be examined.
    ///
    /// # Input
    ///
    /// * `user_keyword` · This keyword is used to search the search index.
    ///
    ///   For example, if the user provided the misspelled word `nthing`, this
    ///   could potentially return `nothing` as an alternative keyword if it
    ///   was present in the index.
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
    /// * This method differs from `eddie_keyword` in that this is a generic
    ///   method. This method will be monomorphized for each `eddie` string
    ///   similarity metric (`DamerauLevenshtein`, `Jaro`, etc.)
    ///
    ///   `eddie_keyword` will call these monomorphized methods using
    ///   dynamic-dispatch, based on the search index's string similarity metric
    ///   settings.
    #[inline(always)]
    pub(crate) fn eddie_keyword_comparator<'s, M>(
        &'s self,
        user_keyword: &str,
        index_range: &str,
        top_scores: &mut crate::simple::internal::fuzzers::FuzzyTopScores::<'s, K, f64>,
    )
    where M: crate::simple::internal::fuzzers::eddie::Metric {
        // Initialize eddie metric instance:
        let scorer = M::new();

        // Scan the search index for the highest scoring keywords:
        self.b_tree_map
            // Get matching keywords starting with (partial) keyword string:
            .range(kstring::KString::from_ref(index_range)..)
            // We did not specify an end bound for our `range` function (see
            // above.) `range` will return _every_ keyword greater than the
            // supplied keyword. The below `take_while` will effectively break
            // iteration when we reach a keyword that does not start with our
            // supplied (partial) keyword.
            .take_while(|(index_keyword, _keys)|
                index_keyword.starts_with::<&str>(index_range.as_ref())
            )
            // For example: if the user typed "gold", and the autocompletion
            // options were "gold" and "golden", this filter would remove "gold"
            // as an autocompletion option because it's currently what's already
            // typed.
            //
            // This is commented-out because returning "gold" as an option
            // provides feedback to the user that their keyword exists in
            // the index.
            /* .filter(|(index_keyword, _index_keys)|
                index_keyword.as_str() != user_keyword
            ) */
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
    } // fn
} // impl