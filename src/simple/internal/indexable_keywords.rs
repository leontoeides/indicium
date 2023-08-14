use crate::simple::internal::string_keywords::SplitContext;
use crate::simple::{Indexable, SearchIndex};
use std::cmp::Ord;
use std::collections::HashSet;

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
    ) -> HashSet<String> {

        // The implemented trait method `strings()` will return the strings from
        // the record that are meant to be indexed:
        let strings = value.strings();

        // Store the individual keywords from these strings:
        strings
            // Iterate over each `String` field from the record:
            .iter()
            // Split each `String` into keywords according to the `SearchIndex`
            // settings. Note that `string_keywords` will allow "use entire
            // string as a keyword" if enabled in user settings. Flatten the
            // string's keywords into the `HashSet`:
            .flat_map(|string| self.string_keywords(string, SplitContext::Indexing))
            // Collect all keywords into a `HashSet`:
            .collect()

    } // fn

} // impl