mod tests;

// -----------------------------------------------------------------------------

use regex::Regex;
use std::clone::Clone;
use std::cmp::Ord;
use std::cmp::PartialEq;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::iter::IntoIterator;
use std::iter::Iterator;

// -----------------------------------------------------------------------------
//
/// To make a struct indexable, the programmer must implement the
/// `IndexableStruct` trait for it. This trait returns a `IndexComponents<K>`
/// struct for each record.

#[derive(Debug)]
pub struct IndexComponents<K: Debug> {
    pub key: K,
    pub strings: Vec<String>,
} // IndexComponents

// -----------------------------------------------------------------------------
//
/// To make a struct indexable, the programmer must implement the
/// `IndexableStruct` trait for it.

pub trait IndexableStruct<K: Debug> {
    fn components(&self) -> IndexComponents<K>;
} // IndexableStruct

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
//
/// A helper function that splits a string into keywords using a `Regex`
/// expression. This function will also filter-out keywords that don't meet the
/// defined length constraints.

fn string_keywords<'a>(
    regex: &'a Regex,
    text: &'a str,
    minimum_keyword_length: usize,
    maximum_keyword_length: usize,
) -> Vec<&'a str> {
    regex
        .split(text)
        .into_iter()
        .filter(|keyword| keyword.len() >= minimum_keyword_length)
        .filter(|keyword| keyword.len() <= maximum_keyword_length)
        .collect()
} // fn

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + PartialEq + Ord> SearchIndex<K> {

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

    /// Clears the search index, removing all elements.

    pub fn clear(&mut self) {
        self.b_tree_map.clear()
    } // fn

    /// Inserts a record into the search index.

    pub fn insert(&mut self, indexable_struct: &dyn IndexableStruct<K>) {
        let struct_components = indexable_struct.components();
        let keywords = SearchIndex::<K>::struct_keywords(
            &self.regex,
            &self.case_sensitive,
            &self.minimum_keyword_length,
            &self.maximum_keyword_length,
            indexable_struct
        ); // struct_keywords
        println!("Keywords found on insert: {:#?}", keywords);
        keywords
            .iter()
            .for_each(|keyword|
                match self.b_tree_map.get_mut(keyword) {
                    Some(keys) => {
                        keys.push(struct_components.key.clone())
                    }, // Some
                    None => {
                        self.b_tree_map.insert(
                            keyword.to_string(),
                            vec![struct_components.key.clone()]
                        ); // insert
                    }, // None
                } // match
            ) // for_each
    } // fn

    /// Removes a record from the search index.

    pub fn remove(&mut self, indexable_struct: &dyn IndexableStruct<K>) {
        let struct_components = indexable_struct.components();
        let keywords = SearchIndex::<K>::struct_keywords(
            &self.regex,
            &self.case_sensitive,
            &self.minimum_keyword_length,
            &self.maximum_keyword_length,
            indexable_struct
        ); // struct_keywords
        keywords
            .iter()
            .for_each(|keyword|
                if let Some(keys) = self.b_tree_map.get_mut(keyword) {
                    keys.retain(|value| value != &struct_components.key)
                } // if let
            ) // for_each
    } // fn

    /// Updates a record in the search index.

    pub fn replace(
        &mut self,
        before: &dyn IndexableStruct<K>,
        after: &dyn IndexableStruct<K>
    ) {
        self.remove(before);
        self.insert(after);
    } // fn

    /// Return all matching _typeahead_ or _autocomplete_ keywords for the
    /// provided search string.

    pub fn autocomplete(&self, string: &str) -> Vec<&String> {
        // If case insensitivity set, convert the search string to lower case:
        let string = match self.case_sensitive {
            true => string.to_string(),
            false => string.to_lowercase(),
        };
        // Attempt to get matching keywords from BTreeMap:
        self.b_tree_map
            // Attempt to get matching keywords for search string:
            .range(string.to_string()..)
            .map(|(key, _value)| key)
            // Only return keywords starting with with search string:
            .take_while(|keyword| keyword.starts_with(&string))
            // Only return `maximum_autocomplete_results` number of keywords:
            .take(self.maximum_autocomplete_results)
            .collect()
    } // fn

    /// Return all matching _search results_ for the provided search string.

    pub fn search(&self, string: &str) -> Option<Vec<&K>> {
        // If case insensitivity set, convert the search string to lower case:
        let string = match self.case_sensitive {
            true => string.to_string(),
            false => string.to_lowercase(),
        };
        // Attempt to get matching keys from BTreeMap:
        self.b_tree_map
            // Attempt to get matching keys for search string:
            .get(&string)
            // Iterate over all matching keys and only return
            // `maximum_search_results` number of keys:
            .map(|search_result| search_result
                .iter()
                .take(self.maximum_search_results)
                .collect()
            ) // map
    } // fn

    /// An associated helper function that returns all keywords for the given
    /// struct using a previously defined `Regex` expression.

    fn struct_keywords(
        regex: &Regex,
        case_sensitive: &bool,
        minimum_keyword_length: &usize,
        maximum_keyword_length: &usize,
        indexable_struct: &dyn IndexableStruct<K>
    ) -> Vec<String> {
        // Iterate over all of the strings returned for the record:
        indexable_struct.components().strings.iter()
            // Split each string into keywords:
            .map(|string| string_keywords(
                regex,
                string,
                *minimum_keyword_length,
                *maximum_keyword_length
            )) // map
            // Flatten the string's keywords:
            .flatten()
            // If case insensitivity set, convert each keyword to lower case:
            .map(|string| match case_sensitive {
                true => string.to_string(),
                false => string.to_lowercase(),
            }) // map
            // Collect all keywords into a Vec:
            .collect()
    } // fn

} // impl

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + PartialEq + Ord> Default for SearchIndex<K> {
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