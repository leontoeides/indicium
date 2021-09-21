use crate::simple::search_index::SearchIndex;
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + Eq + Hash + PartialEq> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Return all matching _typeahead_ or _autocomplete_ keywords for the
    /// provided keyword.
    ///
    /// The provided string is expected to be only a single keyword. For
    /// multi-keyword support see the `autocomplete` method.

    pub fn keyword_autocomplete(&self, string: &str) -> Vec<&String> {

        // If case sensitivity set, leave case intact. Otherwise, convert
        // keyword to lower case:
        let string = match self.case_sensitive {
            true => string.to_string(),
            false => string.to_lowercase(),
        }; // match

        // Attempt to get matching keywords from `BTreeMap`:
        self.b_tree_map
            // Get matching keywords for starting with (partial) keyword string:
            .range(string.to_string()..)
            // `range` returns a key-value pair. We're autocompleting the key,
            // so discard the value:
            .map(|(key, _value)| key)
            // Only return `maximum_autocomplete_results` number of keywords:
            .take(self.maximum_autocomplete_results)
            // We did not specify an end bound for our `range` function (see
            // above.) `range` will return _every_ keyword greater than the
            // supplied keyword. The below `take_while` will effectively break
            // iteration when we reach a keyword that does not start with our
            // supplied (partial) keyword.
            .take_while(|keyword| keyword.starts_with(&string))
            // Collect all keyword autocompletions into a `Vec`:
            .collect()

    } // fn

} // impl