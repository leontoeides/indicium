use crate::simple::internal::FuzzyTopScores;
use kstring::KString;

// -----------------------------------------------------------------------------

impl<K: std::hash::Hash + std::cmp::Ord> crate::simple::search_index::SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the Damerau-Levenshtein string distance metric from Ilia
    /// Schelokov's [eddie](https://crates.io/crates/eddie) crate.
    ///
    /// When the user's last (partial) keyword that is meant to be autocompleted
    /// returns no matches, these `eddie_autocomplete_*` methods can be used to
    /// find the best match for substitution.
    ///
    /// * `index_range` limits which keywords to compare the user's keyword
    /// against. For example, if the `index_range` is "super" and the user's
    /// keyword is "supersonic": only search index keywords beginning with
    /// "super" will be compared against the user's keyword: "supersonic"
    /// against "superalloy", "supersonic" against "supergiant" and so on...
    //
    // Note: these `eddie_autocomplete_*` methods are very similar and may seem
    // repetitive with a lot of boiler plate. These were intentionally made more
    // "concrete" and less modular in order to be more efficient.

    pub(crate) fn eddie_autocomplete_global_damerau_levenshtein(
        &self,
        index_range: &str,
        user_keyword: &str,
    ) -> impl Iterator<Item = (&KString, &std::collections::BTreeSet<K>)> {

        // Instantiate eddie's Damerau-Levenshtein distance struct:
        let damerau_levenshtein = eddie::DamerauLevenshtein::new();

        // This structure will track the top scoring keywords:
        let mut top_scores: FuzzyTopScores<K, f64> =
            FuzzyTopScores::with_capacity(self.maximum_autocomplete_options);

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
            // For each keyword in the search index:
            .for_each(|(index_keyword, index_keys)| {
                // Using this keyword from the search index, calculate its
                // similarity to the user's keyword:
                let score = damerau_levenshtein.similarity(index_keyword, user_keyword);
                // Insert the score into the top scores (if it's normal and high
                // enough):
                if score.is_normal() && score >= self.fuzzy_minimum_score {
                    top_scores.insert(index_keyword, index_keys, score)
                } // if
            }); // for_each

        // Return the top scoring keywords athat could be used as autocomplete
        // options, and their keys, to the caller:
        top_scores.results()

    } // fn

} // impl