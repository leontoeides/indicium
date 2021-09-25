use crate::simple::internal::MAXIMUM_INTERNAL_AUTOCOMPLETE_RESULTS;
use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;
use std::collections::BTreeSet;
use std::fmt::Debug;

// -----------------------------------------------------------------------------

impl<K: Debug + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Return all matching _typeahead_ or _autocomplete_ keywords for the
    /// provided keyword.
    ///
    /// The provided string is expected to be only a single keyword. For
    /// multi-keyword support see the `autocomplete` method.
    ///
    /// Note: This function is lower-level and for internal use only. It does
    /// not observe any settings such as _case-sensitivity_ or _maximum
    /// results_. These constraints should be observed at higher levels.

    pub(crate) fn internal_keyword_autocomplete(&self, keyword: &str) -> BTreeSet<(&String, &Vec<K>)> {

        // Attempt to get matching keywords from `BTreeMap`:
        self.b_tree_map
            // Get matching keywords for starting with (partial) keyword string:
            .range(keyword.to_string()..)
            // We did not specify an end bound for our `range` function (see
            // above.) `range` will return _every_ keyword greater than the
            // supplied keyword. The below `take_while` will effectively break
            // iteration when we reach a keyword that does not start with our
            // supplied (partial) keyword.
            .take_while(|(key, _value)| key.starts_with(&keyword))
            // If the index's keyword matches the user's keyword, don't return
            // it as a result. For example, if the user's keyword was "new" (as
            // in New York), do not return "new" as an auto-completed keyword:
            .filter(|(key, _value)| *key != keyword)
            // Only return `MAXIMUM_INTERNAL_AUTOCOMPLETE_RESULTS` number of
            // keywords:
            .take(MAXIMUM_INTERNAL_AUTOCOMPLETE_RESULTS)
            // Collect all keyword autocompletions into a `BTreeSet`:
            .collect()

    } // fn

} // impl