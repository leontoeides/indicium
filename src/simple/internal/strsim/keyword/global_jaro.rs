use crate::simple::search_index::SearchIndex;
use kstring::KString;
use strsim::jaro;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {
    // -------------------------------------------------------------------------
    //
    /// Scans the entire search index for the closest matching keyword using
    /// the Jaro string similarity metric from Danny Guo's
    /// [strsim](https://crates.io/crates/strsim) crate.
    ///
    /// When the user's search string contains a keyword that returns no
    /// matches, these `strsim_keyword_*` methods can be used to find the best
    /// match for substitution.
    ///
    /// * `index_range` limits which keywords to compare the user's keyword
    ///   against. For example, if the `index_range` is "super" and the user's
    ///   keyword is "supersonic": only search index keywords beginning with
    ///   "super" will be compared against the user's keyword: "supersonic"
    ///   against "superalloy", "supersonic" against "supergiant" and so on...
    //
    // Note: these `strsim_keyword_*` methods are very similar and may seem
    // repetitive with a lot of boiler plate. These were intentionally made more
    // "concrete" and less modular in order to be more efficient.

    pub(crate) fn strsim_keyword_global_jaro(
        &self,
        index_range: &str,
        user_keyword: &str,
    ) -> Option<&KString> {
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
            // to the user's keyword. Map the `(keyword, keys)` tuple into
            // a `(keyword, score)` tuple:
            .map(|(index_keyword, _keys)| (index_keyword, jaro(index_keyword, user_keyword))) // map
            // Search index keyword must meet minimum score to be considered as
            // a fuzzy match:
            .filter(|(_keyword, score)| score >= &self.fuzzy_minimum_score)
            // Find the `(keyword, score)` tuple with the highest score:
            .max_by(|(_a_keyword, a_score), (_b_keyword, b_score)| {
                a_score.total_cmp(b_score)
            }) // max_by
            // Return the `keyword` portion of the `(keyword, score)` tuple
            // to the caller:
            .map(|(keyword, _score)| keyword)
    } // fn
} // impl
