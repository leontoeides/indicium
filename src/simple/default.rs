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
            Some(vec![                      // Split pattern.
                ' ', '\n', '\r', '\t', ',', '.', '(', ')', '<', '>'
            ]),
            false,                          // Case sensitive?
            1,                              // Minimum keyword length (in chars or codepoints.)
            24,                             // Maximum keyword length (in chars or codepoints.)
            Some(24),                       // Maximum text length (in chars or codepoints.)
            Some(vec![                      // Exclude keywords.
                "a".to_string(), "an".to_string(), "and".to_string(),
                "as".to_string(), "as".to_string(), "at".to_string(),
                "but".to_string(), "by".to_string(), "for".to_string(),
                "if".to_string(), "in".to_string(), "nor".to_string(),
                "of".to_string(), "off".to_string(), "on".to_string(),
                "or".to_string(), "per".to_string(), "so".to_string(),
                "the".to_string(), "to".to_string(), "up".to_string(),
                "via".to_string(), "yet".to_string(),
            ]),
            5,                              // Maximum number of auto-complete options.
            100,                            // Maximum number of search results.
        ) // SearchIndex
    } // fn
} // impl