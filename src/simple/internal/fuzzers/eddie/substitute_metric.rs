#![allow(clippy::inline_always)]

// -----------------------------------------------------------------------------

impl<K: Ord> crate::simple::search_index::SearchIndex<K> {
    /// Scans the entire search index for the closest matching keyword using
    /// the the specified string similarity metric from the
    /// [eddie](https://crates.io/crates/eddie) crate.
    ///
    /// When the user's search string contains a keyword that returns no
    /// matches, this method can be used to find the best match to be used as a
    /// substitution.
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
    /// This method returns the single best matching alternative keyword.
    ///
    /// If no reasonable alternative keywords were found, a `None` will be
    /// returned.
    ///
    /// # Notes
    ///
    /// * This method differs from `eddie_substitute` in that this is a
    ///   generic method. This method will be monomorphized for each `eddie`
    ///   string similarity metric (`DamerauLevenshtein`, `Jaro`, etc.)
    ///
    ///   `eddie_substitute` will call these monomorphized methods
    ///   using dynamic-dispatch, based on the search index's string similarity
    ///   metric settings.
    #[must_use]
    #[inline(always)]
    pub fn eddie_substitute_comparator<'s, M>(
        &'s self,
        user_keyword: &str,
        index_range: &str,
    ) -> Option<&'s str>
    where M: crate::simple::internal::fuzzers::eddie::Metric {
        // Initialize eddie metric instance:
        let scorer = M::new();

        // Scan the search index for the highest scoring keyword:
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
            // For each keyword in the search index, calculate its similarity
            // to the user's keyword.
            //
            // If `Some` is returned (because the index keyword meets or exceeds
            // the minimum score) then map the `(keyword, keys)` tuple into
            // a `(keyword, score)` tuple.
            //
            // If `None` is returned from the batch comparator, then that
            // keyword didn't reach the minimum score and it will be filtered
            // out.
            .filter_map(|(index_keyword, _keys)| {
                // Using this keyword from the search index, calculate its
                // similarity to the user's keyword:
                let score = scorer.similarity(index_keyword, user_keyword);

                // Insert the score into the top scores (if it's normal and high
                // enough):
                if score.is_normal() { Some((index_keyword, score)) } else { None }
            }) // map
            // Find the `(keyword, score)` tuple with the highest score:
            .max_by(|(_a_keyword, a_score), (_b_keyword, b_score)|
                a_score.total_cmp(b_score)
            ) // max_by
            // Return the `keyword` portion of the `(keyword, score)` tuple
            // to the caller:
            .map(|(keyword, _score)| keyword.as_str())
    } // fn
} // impl