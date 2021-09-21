// -----------------------------------------------------------------------------

use crate::simple::{indexable::Indexable, search_index::SearchIndex};
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + Eq + Hash + PartialEq> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// An associated helper function that returns all keywords for the given
    /// `Indexable` record.

    pub(crate) fn indexable_keywords(
        &self,
        value: &dyn Indexable,
    ) -> Vec<String> {

        // The implemented trait method `strings()` will return the strings from
        // the record that are meant to be indexed:
        let strings = value.strings();

        // Store the individual keywords from these strings:
        let mut keywords: Vec<String> = strings
            // Iterate over each `String` field from the record:
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

        // Sort, de-duplicate, and the return keywords (and full strings) to
        // the caller:
        keywords.sort_unstable();
        keywords.dedup();
        keywords

    } // fn

} // impl