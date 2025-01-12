use kstring::KString;

// -----------------------------------------------------------------------------

impl<K: Ord> crate::simple::search_index::SearchIndex<K> {
    // -------------------------------------------------------------------------
    //
    /// Scans the entire search index for the closest matching keyword using
    /// the the specified string similarity metric from the
    /// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate.
    ///
    /// When the user's search string contains a keyword that returns no
    /// matches, these `rapidfuzz_keyword_*` methods can be used to find the
    /// match for substitution.
    ///
    /// * `index_range` limits which keywords to compare the user's keyword
    ///   against. For example, if the `index_range` is "super" and the user's
    ///   keyword is "supersonic": only search index keywords beginning with
    ///   "super" will be compared against the user's keyword: "supersonic"
    ///   against "superalloy", "supersonic" against "supergiant" and so on...
    //
    // Note: these `rapidfuzz_keyword_*` methods are very similar and may seem
    // repetitive with a lot of boiler plate. These were intentionally made more
    // "concrete" and less modular in order to be more efficient.
    pub(crate) fn rapidfuzz_keyword_global_generic<BC>(
        &self,
        index_range: &str,
        user_keyword: &str,
    ) -> Option<&KString>
    where BC: crate::simple::internal::rapidfuzz::BatchComparator {
        // Initialize rapidfuzz's batch comparator with the user's keyword:
        let scorer = BC::new(user_keyword);

        // Scan the search index for the highest scoring keyword:
        self.b_tree_map
            // Get matching keywords starting with (partial) keyword string:
            .range(KString::from_ref(index_range)..)
            // We did not specify an end bound for our `range` function (see
            // above.) `range` will return _every_ keyword greater than the
            // supplied keyword. The below `take_while` will effectively break
            // iteration when we reach a keyword that does not start with our
            // supplied (partial) keyword.
            .take_while(|(index_keyword, _keys)| index_keyword.starts_with(index_range))
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
                scorer
                    .normalized_similarity(index_keyword, self.fuzzy_minimum_score)
                    .map(|score| (index_keyword, score))
            }) // map
            // Find the `(keyword, score)` tuple with the highest score:
            .max_by(|(_a_keyword, a_score), (_b_keyword, b_score)| {
                a_score.total_cmp(b_score)
            }) // max_by
            // Return the `keyword` portion of the `(keyword, score)` tuple
            // to the caller:
            .map(|(keyword, _score)| keyword)
    } // fn
} // impl
