// Conditionally select hash map type based on feature flags:
#[cfg(feature = "gxhash")]
type HashSet<T> = std::collections::HashSet<T, gxhash::GxBuildHasher>;

#[cfg(feature = "ahash")]
use ahash::HashSet;

#[cfg(feature = "rustc-hash")]
use rustc_hash::FxHashSet as HashSet;

#[cfg(all(not(feature = "ahash"), not(feature = "gxhash"), not(feature = "rustc-hash")))]
use std::collections::HashSet;

// Static dependencies:
use crate::simple::internal::string_keywords::SplitContext;

// -----------------------------------------------------------------------------

impl<K: Ord> crate::simple::SearchIndex<K> {
    /// An associated helper method that returns all keywords for the given
    /// `Indexable` record. This function also relies on the `string_keywords`
    /// helper method.
    #[inline]
    pub(crate) fn indexable_keywords(
        &self,
        value: &dyn crate::simple::Indexable
    ) -> HashSet<kstring::KString> {
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
