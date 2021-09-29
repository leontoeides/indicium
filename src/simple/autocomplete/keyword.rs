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
    /// The search string is expected to only contain a single keyword. This is
    /// the lightest and fastest autocompletion type. It is good for compact
    /// interfaces or where records are very simple. Results are returned in
    /// lexographic order.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use indicium::simple::SearchIndex;
    /// # use std::collections::BTreeSet;
    /// # use std::collections::HashMap;
    ///
    /// let mut search_index: SearchIndex<usize> = SearchIndex::default();
    ///
    /// let autocomplete_options: BTreeSet<&String> =
    ///     search_index.autocomplete_keyword(&"ass".to_string());
    ///
    /// assert_eq!(
    ///     // Convert `BTreeSet<&String>` to `Vec<&String>`:
    ///     autocomplete_options.iter().cloned().collect::<Vec<&String>>(),
    ///     vec!["assassin", "assistance"]
    /// );
    /// ```

    pub fn autocomplete_keyword(&self, keyword: &str) -> BTreeSet<&String> {

        // If case sensitivity set, leave case intact. Otherwise, convert
        // keyword to lower case:
        let keyword = match self.case_sensitive {
            true => keyword.to_string(),
            false => keyword.to_lowercase(),
        }; // match

        // Attempt to get matching keywords from `BTreeMap`:
        self.b_tree_map
            // Get matching keywords for starting with (partial) keyword string:
            .range(String::from(&keyword)..)
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