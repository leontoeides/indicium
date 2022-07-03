use crate::simple::search_index::SearchIndex;
use std::{cmp::Ord, collections::BTreeSet, hash::Hash};

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the configured string similarity metric. This feature relies on
    /// Danny Guo's [strsim](https://crates.io/crates/strsim) crate.
    ///
    /// When the user's last (partial) keyword that is meant to be autocompleted
    /// returns no matches, these `strsim_autocomplete_*` methods can be used to
    /// find the best match for substitution.

    pub fn strsim_autocomplete(
        &self,
        user_keyword: &str,
    ) -> Vec<(&String, &BTreeSet<K>)> {

        self.strsim_global_autocomplete(user_keyword)

    } // fn

} // impl