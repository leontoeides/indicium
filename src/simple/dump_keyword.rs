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
    ///
    /// The intended usage of `dump_keyword` is retrieve a full listing of all
    /// records registered in the search index. This is useful for populating
    /// the [Select2](https://select2.org/) jQuery plug-in, for fully populating
    /// a [\<datalist\>](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/datalist)
    /// element, and potentially many other things.
    ///
    /// ```rust
    /// # use indicium::simple::SearchIndex;
    /// #
    /// # let mut search_index: SearchIndex<usize> =
    /// #     SearchIndexBuilder::default()
    /// #         .autocomplete_type(&AutocompleteType::Global)
    /// #         .exclude_keywords(&None)
    /// #         .build();
    /// #
    /// # search_index.insert(&0, &MyType::from("apple"));
    /// # search_index.insert(&1, &MyType::from("ball"));
    /// # search_index.insert(&2, &MyType::from("bath"));
    /// # search_index.insert(&3, &MyType::from("bird"));
    /// # search_index.insert(&4, &MyType::from("birthday"));
    /// # search_index.insert(&5, &MyType::from("red"));
    /// # search_index.insert(&6, &MyType::from("truck"));
    /// #
    /// assert_eq!(
    ///     search_index.search(search_index.dump_keyword()).len(),
    ///     7
    /// );
    /// ```

    #[tracing::instrument(level = "trace", name = "Get Dump Keyword", skip(self))]
    pub fn dump_keyword(&self) -> Option<String> {
        self.dump_keyword.to_owned()
    } // fn

} // impl