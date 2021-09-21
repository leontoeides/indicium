// -----------------------------------------------------------------------------

use crate::simple::search_index::SearchIndex;
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + Eq + Hash + PartialEq> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Makes a new, empty `SearchIndex`.

    pub fn new(
        split_pattern: Option<Vec<char>>,
        case_sensitive: bool,
        minimum_keyword_length: usize,
        maximum_keyword_length: usize,
        maximum_string_length: Option<usize>,
        maximum_autocomplete_results: usize,
        maximum_search_results: usize,
    ) -> SearchIndex<K> {
        SearchIndex {
            b_tree_map: BTreeMap::new(),
            split_pattern,
            case_sensitive,
            minimum_keyword_length,
            maximum_keyword_length,
            maximum_string_length,
            maximum_autocomplete_results,
            maximum_search_results,
        } // SearchIndex
    } // fn

} // impl