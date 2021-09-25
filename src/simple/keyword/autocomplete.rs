use crate::simple::search_index::SearchIndex;
use std::collections::BTreeSet;
use std::fmt::Debug;

// -----------------------------------------------------------------------------

impl<K: Debug> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Return all matching _typeahead_ or _autocomplete_ keywords for the
    /// provided keyword.
    ///
    /// The provided string is expected to be only a single keyword. For
    /// multi-keyword support see the `autocomplete` method.

    pub fn keyword_autocomplete(&self, keyword: &str) -> BTreeSet<&String> {

        // If case sensitivity set, leave case intact. Otherwise, convert
        // keyword to lower case:
        let keyword = match self.case_sensitive {
            true => keyword.to_string(),
            false => keyword.to_lowercase(),
        }; // match

        // Attempt to get matching keywords from `BTreeMap`:
        self.b_tree_map
            // Get matching keywords for starting with (partial) keyword string:
            .range(keyword.to_string()..)
            // `range` returns a key-value pair. We're autocompleting the key,
            // so discard the value:
            .map(|(key, _value)| key)
            // We did not specify an end bound for our `range` function (see
            // above.) `range` will return _every_ keyword greater than the
            // supplied keyword. The below `take_while` will effectively break
            // iteration when we reach a keyword that does not start with our
            // supplied (partial) keyword.
            .take_while(|key| key.starts_with(&keyword))
            // If the index's keyword matches the user's keyword, don't return
            // it as a result. For example, if the user's keyword was "new" (as
            // in New York), do not return "new" as an auto-completed keyword:
            .filter(|key| *key != &keyword)
            // Only return `maximum_autocomplete_results` number of keywords:
            .take(self.maximum_autocomplete_results)
            // Collect all keyword autocompletions into a `BTreeSet`:
            .collect()

    } // fn

} // impl