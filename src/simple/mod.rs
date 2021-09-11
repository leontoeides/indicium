mod tests;

// -----------------------------------------------------------------------------

use regex::Regex;
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------
//
/// To make a struct indexable, the programmer must implement the `Indexable`
/// trait for it. The trait returns a `Vec<String>` of all content that is to be
/// indexed.

pub trait Indexable {
    fn strings(&self) -> Vec<String>;
} // Indexable

// -----------------------------------------------------------------------------
//
/// Structure that represents a search index.

#[derive(Debug)]
pub struct SearchIndex<K: Debug> {
    /// The search index data structure.
    b_tree_map: BTreeMap<String, Vec<K>>,
    /// The `Regex` that splits strings into keywords.
    regex_split: Regex,
    /// Indicates whether the search index is case sensitive or not. If set to
    /// false (case insensitive), all keywords will be converted to lower case.
    case_sensitive: bool,
    /// Minimum keyword length (in chars or codepoints) to be indexed.
    minimum_keyword_length: usize,
    /// Maximum keyword length (in chars or codepoints) to be indexed.
    maximum_keyword_length: usize,
    /// Maximum string length (in chars or codepoints) to be indexed. If set,
    /// Indicium will also index the struct's entire strings for autocompletion
    /// purposes.
    maximum_string_length: Option<usize>,
    /// Maximum number of auto-complete options to return.
    maximum_autocomplete_results: usize,
    /// Maximum number of search results to return.
    maximum_search_results: usize,
} // SearchIndex

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + Eq + Hash + PartialEq> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// An associated helper function that splits a `&str` into keywords using a
    /// `Regex` expression. This function will also filter-out keywords that
    /// don't meet the defined length constraints.

    fn string_keywords<'a>(
        &self,
        string: &'a str,
    ) -> Vec<&'a str> {
        // Use the `Regex` expression to split the `String` into keywords and
        // filter the results:
        self.regex_split
            // `Regex` will split the `String` into smaller keyword strings:
            .split(string)
            // Iterate over each resulting keyword `String`:
            .into_iter()
            // Only keep the keyword if it's longer than the minimum length and
            // shorter than the maximum length:
            .filter(|keyword| {
                let chars = keyword.chars().count();
                chars >= self.minimum_keyword_length && chars <= self.maximum_keyword_length
            }) // filter
            // Collect all keywords into a `Vec`:
            .collect()
    } // fn

    // -------------------------------------------------------------------------
    //
    /// An associated helper function that returns all keywords for the given
    /// `Indexable`.

    fn indexable_keywords(
        &self,
        value: &dyn Indexable,
    ) -> Vec<String> {

        // The implemented trait method `strings` will return the strings from
        // the struct that are meant to be indexed:
        let strings = value.strings();

        // Store the individual keywords from these strings:
        let mut keywords: Vec<String> = strings
            // Iterate over each `String` field from the struct:
            .iter()
            // Split each `String` into keywords according to the `SearchIndex`
            // settings:
            .map(|string| self.string_keywords(string))
            // Flatten the string's keywords:
            .flatten()
            // If case sensitivity set, leave case intact. Otherwise, convert
            // each keyword to lower case:
            .map(|string| match self.case_sensitive {
                true => string.to_string(),
                false => string.to_lowercase(),
            }) // map
            // Collect all keywords into a `Vec`:
            .collect();

        // If the option is enabled, store the entire strings themselves for
        // autocompletion purposes:
        if let Some(maximum_string_length) = self.maximum_string_length {
            // Extend the keyword list:
            keywords.extend_from_slice(
                // With the whole strings:
                strings
                    // Iterate over each `String` field from the struct:
                    .iter()
                    // Only keep the strings it's shorter than the maximum:
                    .filter(|string| string.chars().count() <= maximum_string_length)
                    // If case sensitivity set, leave case intact. Otherwise,
                    // convert each string to lower case:
                    .map(|string| match self.case_sensitive {
                        true => string.to_string(),
                        false => string.to_lowercase(),
                    }) // map
                    // Collect all strings into a `Vec`:
                    .collect::<Vec<String>>()
                    // Return to `extend_from_slice` as a slice:
                    .as_slice()
            ) // extend
        } // if

        // Sort, de-duplicate, and the return keywords (and full strings) to
        // the caller:
        keywords.sort_unstable();
        keywords.dedup();
        keywords

    } // fn

    // -------------------------------------------------------------------------
    //
    /// Makes a new, empty `SearchIndex`.

    pub fn new(
        regex_split: Regex,
        case_sensitive: bool,
        minimum_keyword_length: usize,
        maximum_keyword_length: usize,
        maximum_string_length: Option<usize>,
        maximum_autocomplete_results: usize,
        maximum_search_results: usize,
    ) -> SearchIndex<K> {
        SearchIndex {
            b_tree_map: BTreeMap::new(),
            regex_split,
            case_sensitive,
            minimum_keyword_length,
            maximum_keyword_length,
            maximum_string_length,
            maximum_autocomplete_results,
            maximum_search_results,
        } // SearchIndex
    } // fn

    // -------------------------------------------------------------------------
    //
    /// Clears the search index, removing all elements.

    pub fn clear(&mut self) {
        // Clear `BTreeMap`:
        self.b_tree_map.clear()
    } // fn

    // -------------------------------------------------------------------------
    //
    /// Inserts a key-value pair into the search index.

    pub fn insert(&mut self, key: &K, value: &dyn Indexable) {

        // Get all keywords for the `Indexable` record:
        let keywords = self.indexable_keywords(value);

        // Iterate over the keywords:
        keywords
            .iter()
            // For each keyword, add this record's _key_ to the _keyword entry_:
            .for_each(|keyword|
                // Attempt to get mutuable reference to the _keyword entry_ in
                // the search index:
                match self.b_tree_map.get_mut(keyword) {
                    // If keyword found in search index, add _key reference_ for
                    // this record to _keyword entry_:
                    Some(keys) => keys.push(key.clone()),
                    // If keyword not found in search index, initialize _keyword
                    // entry_ with the _key reference_ for this record:
                    None => {
                        self.b_tree_map.insert(
                            keyword.clone(),
                            vec![key.clone()]
                        ); // insert
                    }, // None
                } // match
            ) // for_each

    } // fn

    // -------------------------------------------------------------------------
    //
    /// Removes a key-value pair from the search index.

    pub fn remove(&mut self, key: &K, value: &dyn Indexable) {

        // Get all keywords for the `Indexable` record:
        let keywords = self.indexable_keywords(value);

        // Iterate over the keywords:
        keywords
            .iter()
            // For each keyword, remove this record's _key_ from the _keyword
            // entry_:
            .for_each(|keyword| {
                // Attempt to get mutuable reference to the _keyword entry_ in
                // the search index:
                let is_empty = if let Some(keys) = self.b_tree_map.get_mut(keyword) {
                    // If keyword found in search index, remove the _key
                    // reference_ for this record from _keyword entry_:
                    keys.retain(|value| value != key);
                    // Return whether the _keyword entry_ is now empty or not:
                    keys.is_empty()
                } else {
                    // If keyword not found in search index, signal that we
                    // should **not** remove the _keyword entry_ because that
                    // would result in an error:
                    false
                }; // if
                // If the _keyword entry_ no longer contains any _key
                // references_, it is empty and we should remove the keyword
                // from the search index:
                if is_empty { self.b_tree_map.remove(keyword); }
            }) // for_each

    } // fn

    // -------------------------------------------------------------------------
    //
    /// Replaces (or updates) the value for a key-value pair in the search
    /// index.

    pub fn replace(
        &mut self,
        key: &K,
        before: &dyn Indexable,
        after: &dyn Indexable,
    ) {
        // Remove all references to the old record and its keywords:
        self.remove(key, before);
        // Index the updated record:
        self.insert(key, after);
    } // fn

    // -------------------------------------------------------------------------
    //
    /// Returns the keys resulting from the single keyword search.
    ///
    /// The provided string is expected to be only a single keyword. For
    /// multi-keyword support see the `search` method.

    pub fn keyword_search(&self, keyword: &str) -> Vec<&K> {

        // If case insensitivity set, convert the keyword to lower case:
        let keyword = match self.case_sensitive {
            true => keyword.to_string(),
            false => keyword.to_lowercase(),
        }; // match

        // Attempt to get matching keys for the search keyword from BTreeMap:
        if let Some(keys) = self.b_tree_map.get(&keyword) {

            // Attempt to get matching keys for search keyword:
            keys
                // Iterate over all matching keys and only return
                // `maximum_search_results` number of keys:
                .iter()
                // Only return `maximum_search_results` number of keys:
                .take(self.maximum_search_results)
                // Collect all resulting keys into a `Vec`:
                .collect()

        } else {

            // The search keyword did not result in any matches. Return an
            // empty `Vec`:

            vec![]

        } // if

    } // fn

    // -------------------------------------------------------------------------
    //
    /// Returns the keys resulting from the search string. The search string may
    /// contain several keywords.

    pub fn search(&self, string: &str) -> Vec<K> {

        // Split search `String` into keywords according to the `SearchIndex`
        // settings:
        let keywords = self.string_keywords(string);

        // This `HashMap` is used to count the number of hits for each resulting
        // key. This is so we can return search results in order of relevance:
        let mut search_results: HashMap<K, usize> = HashMap::new();

        // Get each keyword from our `BTreeMap`, record the resulting keys in
        // a our `HashMap`, and track the hit-count for each key:
        keywords
            // Iterate over the keywords supplied in the search string:
            .iter()
            // For each keyword in the search string:
            .for_each(|keyword|
                // Search for keyword in our `BTreeMap`:
                self.keyword_search(keyword)
                    // Iterate over the resulting keys (if any):
                    .iter()
                    // For each resulting key from the keyword search:
                    .for_each(|key| match search_results.get_mut(key) {
                        // Add "hit" to counter for an already existing key:
                        Some(result_entry) => { *result_entry += 1 },
                        // No record for this key, initialize to one hit:
                        None => { search_results.insert((*key).clone(), 1); },
                    }) // for_each
            ); // for_each

        // At this point, we have a list of keys in a `HashMap`. The hash map
        // value holds the number of times each key has been returned in the
        // above keywords search.
        //
        // We want to sort these keys by descending hit-count. We'll convert it
        // to a `Vec` so this can be done:

        let mut search_results: Vec<(K, usize)> = search_results
            // Iterate over keys in the hash map:
            .iter()
            // Convert the key-value pair into a tuple element:
            .map(|(key, value)| (key.clone(), *value))
            // Collect the tuple elements into a `Vec`:
            .collect();

        // Sort the tuple elements by hit-count descending:
        search_results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Return the search results to the user:
        search_results
            // Iterate over the tuple elements:
            .iter()
            // Only return `maximum_search_results` number of keys:
            .take(self.maximum_search_results)
            // Remove the hit-count from the tuple, returning only the key:
            .map(|(key, _value)| key.clone())
            // Collect the keys into a `Vec`:
            .collect()

    } // fn

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

    // -------------------------------------------------------------------------
    //
    /// Return all matching _typeahead_ or _autocomplete_ keywords for the
    /// provided search string. The search string may contain several keywords.
    /// The last keyword in the string will be autocompleted.

    pub fn autocomplete(&self, string: &str) -> Vec<String> {

        // Split search `String` into keywords according to the `SearchIndex`
        // settings:
        let mut keywords = self.string_keywords(string);

        // Pop the last keyword off the list. It's the keyword that we'll be
        // autocompleting:
        if let Some(last_keyword) = keywords.pop() {

            // Autocomplete the last keyword:
            let autocompletions = self.keyword_autocomplete(last_keyword);

            // Push a blank placeholder onto the end of the keyword list. We
            // will be putting our autocompletions for the last keyword into
            // this spot:
            keywords.push("");

            // Build autocompleted search strings from the autocompletions
            // derived from the last keyword:
            autocompletions
                // Iterate over each autocompleted last keyword:
                .iter()
                // Use the prepended `keywords` and autocompleted last keyword
                // to build an autocompleted search string:
                .map(|last_keyword| {
                    // Remove previous autocompleted last keyword from list:
                    keywords.pop();
                    // Add current autocompleted last keyword to end of list:
                    keywords.push(last_keyword);
                    // Join all keywords together into a single `String` using a
                    // space delimiter:
                    keywords.join(" ")
                })
                // Collect all string autocompletions into a `Vec`:
                .collect()

        } else {

            // The search string did not have a last keyword to autocomplete.
            // Return an empty `Vec`:
            vec![]

        } // if

    } // fn

} // impl

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + Eq + Hash + PartialEq> Default for SearchIndex<K> {
    fn default() -> Self {
        Self::new(
            Regex::new(r"([ ,.]+)").expect("Invalid regex"),
            false,      // Case sensitive?
            3,          // Minimum keyword length (in chars or codepoints.)
            24,         // Maximum keyword length (in chars or codepoints.)
            Some(24),   // Maximum text length (in chars or codepoints.)
            5,          // Maximum number of auto-complete options.
            100,        // Maximum number of search results.
        ) // SearchIndex
    } // fn
} // impl