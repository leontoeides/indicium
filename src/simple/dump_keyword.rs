use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Returns the special keyword that will return all keys (or records) in
    /// the search index.
    ///
    /// A _dump keyword_ should be selected so that it's difficult for a user
    /// inadvertently trigger this behaviour. It might be a good idea to turn
    /// this feature off for extremely large search indexes.
    ///
    /// This keyword is helpful for the [Select2](https://select2.org/) jQuery
    /// plug-in, where returning all keys is the desirable behaviour when the
    /// search string is empty. It can also be used for fully populating a
    /// [\<datalist\>](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/datalist)
    /// element, and potentially other things.
    ///
    /// The default value of `null` is probably fine, but it's configurable and
    /// it can be turned off altogether. For more information on the setting the
    /// `dump_keyword` for a `SearchIndex` type see: [`SearchIndexBuilder`].
    ///
    /// [`SearchIndexBuilder`]: struct.SearchIndexBuilder.html#method.dump_keyword
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use indicium::simple::SearchIndex;
    /// # let mut search_index: SearchIndex<usize> = SearchIndex::default();
    /// assert_eq!(search_index.dump_keyword(), Some("\0"));
    /// ```
    ///
    /// The intended usage of `dump_keyword` is retrieve a full listing of all
    /// records registered in the search index. Example usage:
    ///
    /// ```rust
    /// # use indicium::simple::{AutocompleteType, Indexable, SearchIndex, SearchIndexBuilder};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// # let mut search_index: SearchIndex<usize> =
    /// #     SearchIndexBuilder::default()
    /// #         .autocomplete_type(AutocompleteType::Global)
    /// #         .exclude_keywords(None)
    /// #         .build();
    /// #
    /// # struct MyType { text: String }
    /// #
    /// # impl From<&str> for MyType {
    /// #   fn from(string: &str) -> Self {
    /// #       MyType { text: string.to_string() }
    /// #   }
    /// # }
    /// #
    /// # impl Indexable for MyType {
    /// #   fn strings(&self) -> Vec<String> {
    /// #       vec![self.text.clone()]
    /// #   }
    /// # }
    /// #
    /// # search_index.insert(&0, &MyType::from("apple"));
    /// # search_index.insert(&1, &MyType::from("ball"));
    /// # search_index.insert(&2, &MyType::from("bath"));
    /// # search_index.insert(&3, &MyType::from("bird"));
    /// # search_index.insert(&4, &MyType::from("birthday"));
    /// # search_index.insert(&5, &MyType::from("red"));
    /// # search_index.insert(&6, &MyType::from("truck"));
    /// #
    /// if let Some(dump_keyword) = search_index.dump_keyword() {
    ///     assert_eq!(
    ///         search_index.search(&dump_keyword),
    ///         [&0, &1, &2, &3, &4, &5, &6]
    ///     );
    /// }
    /// ```

    pub fn dump_keyword(&self) -> Option<&str> {
        self.dump_keyword.as_ref().map(|kstring| kstring.as_str())
    } // fn

} // impl