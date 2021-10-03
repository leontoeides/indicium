use crate::simple::{Indexable, SearchIndex};
use std::cmp::Ord;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// An associated helper method that returns all keywords for the given
    /// `Indexable` record. This function also relies on the `string_keywords`
    /// helper method.

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
            // settings. `string_keywords` will allow "use entire string as a
            // keyword" if enabled in user settings:
            .map(|string| self.string_keywords(string, true))
            // Flatten the string's keywords:
            .flatten()
            // Collect all keywords into a `Vec`:
            .collect();

        // Sort keywords & remove duplicates:
        keywords.sort_unstable();
        keywords.dedup();

        // Return keywords to caller:
        keywords

    } // fn

} // impl