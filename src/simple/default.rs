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
            // Default split pattern:
            Some(vec![
                '\t',                       // Tab
                '\n',                       // Newline
                '\r',                       // Carriage return
                ' ',                        // Space
                '!',
                '"',                        // Double quotation
                '&',
                '(',
                ')',
                '*',
                '+',
                ',',
                '-',
                '.',
                '/',
                ':',
                ';',
                '<',
                '=',
                '>',
                '?',
                '[',
                '\'',                       // Single quotation or apostrophe
                '\\',                       // Backslash
                ']',
                '^',
                '`',
                '{',
                '|',
                '}',
                '~',
            ]),
            false,                          // Case sensitive?
            1,                              // Minimum keyword length (in chars or codepoints.)
            24,                             // Maximum keyword length (in chars or codepoints.)
            Some(24),                       // Maximum text length (in chars or codepoints.)
            // Default keywords to be excluded:
            Some(vec![
                // Some English:
                "a".to_string(),
                "an".to_string(),
                "and".to_string(),
                "as".to_string(),
                "at".to_string(),
                "but".to_string(),
                "by".to_string(),
                "for".to_string(),
                "if".to_string(),
                "in".to_string(),
                "nor".to_string(),
                "of".to_string(),
                "off".to_string(),
                "on".to_string(),
                "or".to_string(),
                "per".to_string(),
                "so".to_string(),
                "the".to_string(),
                "to".to_string(),
                "up".to_string(),
                "via".to_string(),
                "yet".to_string(),
                // Some French:
                "de".to_string(),
                "et".to_string(),
                "la".to_string(),
                "le".to_string(),
                "les".to_string(),
                "ou".to_string(),
                // Some Spanish:
                "del".to_string(),
                "el".to_string(),
                "las".to_string(),
                "los".to_string(),
                "y".to_string(),
            ]),
            5,                              // Maximum number of auto-complete options.
            100,                            // Maximum number of search results.
        ) // SearchIndex
    } // fn
} // impl