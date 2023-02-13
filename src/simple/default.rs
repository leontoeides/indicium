use crate::simple::{AutocompleteType, SearchIndex, SearchType, StrSimType};
use std::cmp::Ord;

// -----------------------------------------------------------------------------
//
/// Default values for a `SearchIndex`. These values can be overridden by using
/// `SearchIndex::new()` or `SearchIndexBuilder`.

impl<K: Ord> Default for SearchIndex<K> {
    fn default() -> Self {
        Self::new(
            SearchType::Live,               // Search type.
            AutocompleteType::Context,      // Autocompletion type.
            Some(StrSimType::Levenshtein),  // String similarity metric type.
            3,                              // String similarity match length.
            0.3,                            // String similarity minimum score.
            // Default split pattern:
            Some(vec![
                '\t',                       // Tab
                '\n',                       // Newline
                '\r',                       // Carriage return
                ' ',                        // Space
                '!',                        // Exclamation Mark
                '"',                        // Double quotation
                '&',                        // Ampersand
                '(',                        // Left Parenthesis
                ')',                        // Right Parenthesis
                '*',                        // Asterisk
                '+',                        // Plus Sign
                ',',                        // Comma
                '-',                        // Minus Sign
                '.',                        // Full Stop
                '/',                        // Solidus
                ':',                        // Colon
                ';',                        // Semicolon
                '<',                        // Less-Than Sign
                '=',                        // Equals Sign
                '>',                        // Greater-Than Sign
                '?',                        // Question Mark
                '[',                        // Left Square Bracket
                '\\',                       // Reverse Solidus
                ']',                        // Right Square Bracket
                '^',                        // Circumflex Accent
                '`',                        // Grave Accent
                '{',                        // Left Curly Bracket
                '|',                        // Vertical Line
                '}',                        // Right Curly Bracket
                '~',                        // Tilde
                ' ',                        // No-Break Space
                '¡',                        // Inverted Exclamation Mark
                '«',                        // Left-Pointing Double Angle Quotation Mark
                '»',                        // Right-Pointing Double Angle Quotation Mark
                '¿',                        // Inverted Question Mark
                '×',                        // Multiplication Sign
                '÷',                        // Division Sign
                'ˆ',                        // Modifier Letter Circumflex Accent
                '‘',                        // Left Single Quotation Mark
                '’',                        // Right Single Quotation Mark
                '“',                        // Left Double Quotation Mark
                '”',                        // Right Double Quotation Mark
                '„',                        // Double Low-9 Quotation Mark
                '‹',                        // Single Left-Pointing Angle Quotation Mark
                '›',                        // Single Right-Pointing Angle Quotation Mark
                '—',                        // Em Dash
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
                "en".to_string(),
                "et".to_string(),
                "la".to_string(),
                "le".to_string(),
                "les".to_string(),
                "ni".to_string(),
                "ou".to_string(),
                "par".to_string(),
                "pour".to_string(),
                "si".to_string(),
                "sur".to_string(),
                "un".to_string(),
                "une".to_string(),
                "à".to_string(),
                // Some Spanish:
                "asi".to_string(),
                "así".to_string(),
                "aun".to_string(),
                "aún".to_string(),
                "del".to_string(),
                "el".to_string(),
                "las".to_string(),
                "los".to_string(),
                "o".to_string(),
                "para".to_string(),
                "por".to_string(),
                "que".to_string(),
                "sí".to_string(),
                "si".to_string(),
                "tan".to_string(),
                "una".to_string(),
                "uno".to_string(),
                "vía".to_string(),
                "y".to_string(),
            ]),
            5,                              // Maximum number of auto-complete options.
            100,                            // Maximum number of search results.
            40_960,                         // Maximum keys per keyword.
            Some("\0".to_string()),         // Dump keyword.
        ) // SearchIndex
    } // fn
} // impl