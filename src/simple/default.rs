use crate::simple::{AutocompleteType, SearchIndex, SearchType};
use std::cmp::Ord;

// -----------------------------------------------------------------------------
//
/// Default values for a `SearchIndex`. These values can be overridden by using
/// `SearchIndex::new()` or `SearchIndexBuilder`.

impl<K: Ord> Default for SearchIndex<K> {
    fn default() -> Self {
        Self::new(
            SearchType::Or,                 // Search type.
            AutocompleteType::Context,      // Autocompletion type.
            Some(vec![' ', '\n', '\r', '\t', ',', '.']), // Split characters.
            false,                          // Case sensitive?
            1,                              // Minimum keyword length (in chars or codepoints.)
            24,                             // Maximum keyword length (in chars or codepoints.)
            Some(24),                       // Maximum text length (in chars or codepoints.)
            5,                              // Maximum number of auto-complete options.
            100,                            // Maximum number of search results.
        ) // SearchIndex
    } // fn
} // impl