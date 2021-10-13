use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// A special keyword that will return or "dump" all keys (or records) in
    /// the search index. This is helpful for the `Select2` module, where it
    /// should be returning all records if the search string is empty.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use indicium::simple::SearchIndex;
    /// # let mut search_index: SearchIndex<usize> = SearchIndex::default();
    /// assert_eq!(search_index.dump_keyword(), "\0".to_string());
    /// ```

    #[tracing::instrument(level = "trace", name = "Get Dump Keyword", skip(self))]
    pub fn dump_keyword(&self) -> Option<String> {
        self.dump_keyword.to_owned()
    } // fn

} // impl