use crate::simple::conjunction::Conjunction;
use crate::simple::search_index::SearchIndex;
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + Eq + Hash + PartialEq> Default for SearchIndex<K> {
    fn default() -> Self {
        Self::new(
            // `And` was chosen as the default conjunction because it uses
            // fewer CPU resources. `Or` may be preferable for smaller search
            // indicies.
            Conjunction::And,    // Logical conjuction for each keyword's results.
            Some(vec![' ', '\n', '\r', '\t', ',', '.']), // Split characters.
            false,              // Case sensitive?
            1,                  // Minimum keyword length (in chars or codepoints.)
            24,                 // Maximum keyword length (in chars or codepoints.)
            Some(24),           // Maximum text length (in chars or codepoints.)
            5,                  // Maximum number of auto-complete options.
            20,                 // Maximum number of search results.
        ) // SearchIndex
    } // fn
} // impl