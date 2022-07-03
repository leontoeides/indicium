use crate::simple::search_index::SearchIndex;
use std::{cmp::Ord, hash::Hash};

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Scans the entire search index for the closest matching keyword using
    /// the configured string similarity metric. This feature relies on Danny
    /// Guo's [strsim](https://crates.io/crates/strsim) crate.
    ///
    /// When the user's search string contains a keyword that returns no
    /// matches, these `strsim_keyword_*` methods can be used to find the best
    /// match for substitution.

    pub fn strsim_keyword(
        &self,
        user_keyword: &str,
    ) -> Option<&String> {

        self.strsim_global_keyword(user_keyword)

    } // fn

} // impl