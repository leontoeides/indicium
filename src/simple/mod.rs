mod tests;

// -----------------------------------------------------------------------------

use regex::Regex;
use std::clone::Clone;
use std::cmp::Eq;
use std::cmp::PartialEq;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::{IntoIterator, Iterator};

// -----------------------------------------------------------------------------
//
/// To make a struct indexable, the programmer must implement the
/// `Indexable` trait for it. The trait will return a `Vec<String>` of all
/// content that is to be indexed.

pub trait Indexable {
    fn strings(&self) -> Vec<String>;
} // Indexable

// -----------------------------------------------------------------------------
//
/// Structure that represents a search index.

#[derive(Debug)]
pub struct SearchIndex<K: Debug> {
    b_tree_map: BTreeMap<String, Vec<K>>,
    regex: Regex,
    case_sensitive: bool,
    minimum_keyword_length: usize,
    maximum_keyword_length: usize,
    maximum_autocomplete_results: usize,
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
        self.regex
            // `Regex` will split the `String` into smaller keyword strings:
            .split(string)
            // Iterate over each resulting keyword `String`:
            .into_iter()
            // Only keep the keyword if it's larger than the minimum length:
            .filter(|keyword| keyword.len() >= self.minimum_keyword_length)
            // Only keep the keyword if it's smaller than the maximum length:
            .filter(|keyword| keyword.len() <= self.maximum_keyword_length)
            // Collect all keywords into a `Vec`:
            .collect()
    } // fn

    // -------------------------------------------------------------------------
    //
    /// An associated helper function that returns all keywords for the given
    /// `Indexable`.

    fn struct_keywords(
        &self,
        value: &dyn Indexable,
    ) -> Vec<String> {
        // Process the value for the key-value pair:
        value
            // The implemented trait function will return several strings from
            // the struct that are to be indexed:
            .strings()
            // Iterate over each returned `String`:
            .iter()
            // Split search string into keywords, according to the rules:
            .map(|string| self.string_keywords(string))
            // Flatten the string's keywords:
            .flatten()
            // If case sensitivity set, leave case intact. Otherwise, convert
            // each keyword to lower case.
            .map(|string| match self.case_sensitive {
                true => string.to_string(),
                false => string.to_lowercase(),
            }) // map
            // Collect all keywords into a `Vec`:
            .collect()
    } // fn




























    // -------------------------------------------------------------------------
    //
    /// Makes a new, empty `SearchIndex`.

    pub fn new(
        regex: Regex,
        case_sensitive: bool,
        minimum_keyword_length: usize,
        maximum_keyword_length: usize,
        maximum_autocomplete_results: usize,
        maximum_search_results: usize,
    ) -> SearchIndex<K> {
        SearchIndex {
            b_tree_map: BTreeMap::new(),
            regex,
            case_sensitive,
            minimum_keyword_length,
            maximum_keyword_length,
            maximum_autocomplete_results,
            maximum_search_results,
        } // SearchIndex
    } // fn
























    // -------------------------------------------------------------------------
    //
    /// Clears the search index, removing all elements.

    pub fn clear(&mut self) {
        self.b_tree_map.clear()
    } // fn

    // -------------------------------------------------------------------------
    //
    /// Inserts a key-value pair into the search index.

    pub fn insert(&mut self, key: &K, value: &dyn Indexable) {
        let keywords = self.struct_keywords(value);
        keywords
            .iter()
            .for_each(|keyword|
                match self.b_tree_map.get_mut(keyword) {
                    Some(keys) => keys.push(key.clone()),
                    None => {
                        self.b_tree_map.insert(
                            keyword.to_string(),
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
        let keywords = self.struct_keywords(value);
        keywords
            .iter()
            .for_each(|keyword|
                if let Some(keys) = self.b_tree_map.get_mut(keyword) {
                    keys.retain(|value| value != key)
                } // if let
            ) // for_each
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
        self.remove(key, before);
        self.insert(key, after);
    } // fn























    // -------------------------------------------------------------------------
    //
    /// Return all matching _typeahead_ or _autocomplete_ keywords for the
    /// provided single keyword string.

    pub fn autocomplete_keyword(&self, string: &str) -> Vec<&String> {

        // If case sensitivity set, leave case intact. Otherwise, convert
        // keyword to lower case.
        let string = match self.case_sensitive {
            true => string.to_string(),
            false => string.to_lowercase(),
        }; // match

        // Attempt to get matching keywords from BTreeMap:
        self.b_tree_map
            // Get matching keywords for starting with keyword string:
            .range(string.to_string()..)
            // `range` returns a key-value pair. We're autocompleting the key,
            // so discard the value:
            .map(|(key, _value)| key)
            // There was no end bound for our `range` function above. Only
            // return keywords starting with with search keyword:
            .take_while(|keyword| keyword.starts_with(&string))
            // Only return `maximum_autocomplete_results` number of keywords:
            .take(self.maximum_autocomplete_results)
            // Collect all keyword autocompletions into a `Vec`:
            .collect()

    } // fn

    // -------------------------------------------------------------------------
    //
    /// Return all matching _typeahead_ or _autocomplete_ keywords for the
    /// provided search string. The search string may contain several keywords.
    /// The last keyword in the string will be autocompleted.

    pub fn autocomplete_string(&self, string: &str) -> Vec<String> {

        // Split search string into keywords, according to the rules:
        let mut keywords = self.string_keywords(string);

        // Pop the last keyword off collection. It's the keyword that we'll be
        // autocompleting:
        if let Some(last_keyword) = keywords.pop() {

            // Autocomplete the last keyword:
            let autocompletions = self.autocomplete_keyword(last_keyword);

            // Push a blank placeholder for the autocompleted last keyword onto
            // the end of the list:
            keywords.push("");

            // Build search strings from the last keyword autocompletions:
            autocompletions
                // Iterate over each returned autocompleted keyword:
                .iter()
                // Use the `keywords` and autocompleted last keyword to build
                // an search string:
                .map(|last_keyword| {
                    // Remove previous autocompleted last keyword from list:
                    keywords.pop();
                    // Add current autocompleted last keyword:
                    keywords.push(last_keyword);
                    // Join all keywords together using a space delimiter:
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

















    // -------------------------------------------------------------------------
    //
    /// Returns the keys resulting from the single keyword search.

    pub fn search_keyword(&self, keyword: &str) -> Option<Vec<&K>> {

        // If case insensitivity set, convert the keyword to lower case:
        let keyword = match self.case_sensitive {
            true => keyword.to_string(),
            false => keyword.to_lowercase(),
        }; // match

        // Attempt to get matching keys from BTreeMap:
        self.b_tree_map
            // Attempt to get matching keys for search keyword:
            .get(&keyword)
            // Iterate over all matching keys and only return
            // `maximum_search_results` number of keys:
            .map(|search_result| search_result
                .iter()
                .take(self.maximum_search_results)
                .collect()
            ) // map

    } // fn

    // -------------------------------------------------------------------------
    //
    /// Returns the keys resulting from the search string. The search string may
    /// contain several keywords.

    pub fn search_string(&self, string: &str) -> Vec<K> {

        // Split search string into keywords, according to the rules:
        let keywords = self.string_keywords(string);




        let mut search_results: HashMap<K, usize> = HashMap::new();

        // Search for each keyword:
        keywords
            .iter()
            .for_each(|keyword| {
                let keys = self.search_keyword(keyword).unwrap();
                keys
                    .iter()
                    .for_each(|key| match search_results.get_mut(key) {
                        Some(result_entry) => { *result_entry += 1 },
                        None => { search_results.insert((*key).clone(), 0); },
                    })
            });



        let mut search_results: Vec<(K, usize)> = search_results
            .iter()
            .map(|(key, value)| (key.clone(), *value))
            .collect();


        search_results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());


        search_results
            .iter()
            .map(|(key, _value)| key.clone())
            .collect()

        //let keys: Vec<(u8, &K)> = vec![];
        // weighted intersection - count hits by each keyword, sort descernding order


        //None

    } // fn















} // impl

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + Eq + Hash + PartialEq> Default for SearchIndex<K> {
    fn default() -> Self {
        Self::new(
            Regex::new(r"([ ,.]+)").expect("Invalid regex"),
            false,      // Case sensitive?
            3,          // Minimum keyword length.
            24,         // Maximum keyword length.
            5,          // Maximum number of auto-complete results.
            20          // Maximum number of search results.
        ) // SearchIndex
    } // fn
} // impl