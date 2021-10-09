use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// If there are too many records attached to a single keyword, performance
    /// can begin to degrade. This setting limits the number of keys that may be
    /// attached to a keyword. This function returns the
    /// `maximum_keys_per_keyword` setting from the search index.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use indicium::simple::SearchIndex;
    /// # let mut search_index: SearchIndex<usize> = SearchIndex::default();
    /// assert_eq!(search_index.maximum_keys_per_keyword(), 40_960);
    /// ```

    #[tracing::instrument(level = "trace", name = "Search Index Maximum Keys per Keyword", skip(self))]
    pub fn maximum_keys_per_keyword(&self) -> usize {
        self.maximum_keys_per_keyword
    } // fn

} // impl