use crate::simple::internal::MAXIMUM_INTERNAL_AUTOCOMPLETE_RESULTS;
use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;
use std::collections::BTreeSet;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Returns matching autocompleted keywords for the provided search string.
    /// _This search method only accepts a single keyword as the search string._
    /// The partial search keyword must be an exact match.
    ///
    /// Note: This function is lower-level and for internal use only. It does
    /// not observe any settings such as _case-sensitivity_ or _maximum
    /// results_. These constraints should be observed at higher levels.

    pub(crate) fn internal_autocomplete_keyword(&self, keyword: &str) -> BTreeSet<(&String, &BTreeSet<K>)> {

        // Attempt to get matching keywords from `BTreeMap`:
        let autocomplete_options: BTreeSet<(&String, &BTreeSet<K>)> = self.b_tree_map
            // Get matching keywords for starting with (partial) keyword string:
            .range(String::from(keyword)..)
            // We did not specify an end bound for our `range` function (see
            // above.) `range` will return _every_ keyword greater than the
            // supplied keyword. The below `take_while` will effectively break
            // iteration when we reach a keyword that does not start with our
            // supplied (partial) keyword.
            .take_while(|(key, _value)| key.starts_with(&keyword))
            // If the index's keyword matches the user's keyword, don't return
            // it as a result. For example, if the user's keyword was "new" (as
            // in New York), do not return "new" as an auto-completed keyword:
            // .filter(|(key, _value)| *key != keyword)
            // Only return `MAXIMUM_INTERNAL_AUTOCOMPLETE_RESULTS` number of
            // keywords:
            .take(MAXIMUM_INTERNAL_AUTOCOMPLETE_RESULTS)
            // Collect all keyword autocompletions into a `BTreeSet`:
            .collect();

        // For debug builds:
        #[cfg(debug_assertions)]
        if autocomplete_options.len() >= MAXIMUM_INTERNAL_AUTOCOMPLETE_RESULTS {
            tracing::warn!(
                "Internal table limit of {} keywords has been exceeded. \
                Data has been dropped. This will impact accuracy of results. \
                For this data set, consider using a more comprehensive database solution like MeiliSearch.",
                MAXIMUM_INTERNAL_AUTOCOMPLETE_RESULTS
            ); // error!
        } // if

        autocomplete_options

    } // fn

} // impl