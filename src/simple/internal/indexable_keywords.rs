// Conditionally select hash map type based on feature flags:
#[cfg(feature = "gxhash")]
type HashSet<T> = std::collections::HashSet<T, gxhash::GxBuildHasher>;
#[cfg(all(feature = "ahash", not(feature = "gxhash")))]
use ahash::HashSet;
#[cfg(all(not(feature = "ahash"), not(feature = "gxhash")))]
use std::collections::HashSet;

// Static dependencies:
use crate::simple::internal::string_keywords::SplitContext;
use crate::simple::{Indexable, SearchIndex};
use kstring::KString;
use std::cmp::Ord;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {
    // -------------------------------------------------------------------------
    //
    /// An associated helper method that returns all keywords for the given
    /// `Indexable` record. This function also relies on the `string_keywords`
    /// helper method.

    pub(crate) fn indexable_keywords(&self, value: &dyn Indexable) -> HashSet<KString> {
        // The implemented trait method `strings()` will return the strings from
        // the record that are meant to be indexed:
        let strings = value.strings();

        // Store the individual keywords from these strings:
        strings
            // Iterate over each `String` field from the record:
            .into_iter()
            // Split each `String` into keywords according to the `SearchIndex`
            // settings. Note that `string_keywords` will allow "use entire
            // string as a keyword" if enabled in user settings. Flatten the
            // string's keywords into the `HashSet`:
            .flat_map(|string| self.string_keywords(&string, &SplitContext::Indexing))
            // Collect all keywords into a `HashSet`:
            .collect()
    } // fn
} // impl
