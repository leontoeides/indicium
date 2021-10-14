use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// A special keyword that will return (or "dump") all keys (or records) in
    /// the search index. It should be made so that it's difficult or impossible
    /// for a user inadvertently trigger this behaviour.
    ///
    /// This keyword is helpful for the `Select2` module, where returning all
    /// keys is the desirable behaviour when the search string is empty.
    ///
    /// The default value of `null` is probably fine, but it was made
    /// configurable just in case.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use indicium::simple::SearchIndex;
    /// # let mut search_index: SearchIndex<usize> = SearchIndex::default();
    /// assert_eq!(search_index.dump_keyword(), Some("\0".to_string()));
    /// ```

    #[tracing::instrument(level = "trace", name = "Get Dump Keyword", skip(self))]
    pub fn dump_keyword(&self) -> Option<String> {
        self.dump_keyword.to_owned()
    } // fn

} // impl