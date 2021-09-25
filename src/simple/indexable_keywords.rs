use crate::simple::{indexable::Indexable, search_index::SearchIndex};
use std::fmt::Debug;

// -----------------------------------------------------------------------------

impl<K: Debug> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// An associated helper method that returns all keywords for the given
    /// `Indexable` record. This function relies on the `string_keywords`
    /// method.

    pub(crate) fn indexable_keywords(
        &self,
        value: &dyn Indexable,
    ) -> Vec<String> {

        // The implemented trait method `strings()` will return the strings from
        // the record that are meant to be indexed:
        let strings = value.strings();

        // Store the individual keywords from these strings:
        strings
            // Iterate over each `String` field from the record:
            .iter()
            // Split each `String` into keywords according to the `SearchIndex`
            // settings. `string_keywords` will allow "use entire string as a
            // keyword" if enabled in user settings:
            .map(|string| self.string_keywords(string, true))
            // Flatten the string's keywords:
            .flatten()
            // Collect all keywords into a `Vec`:
            .collect()

    } // fn

} // impl