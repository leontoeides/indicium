// -----------------------------------------------------------------------------
//
/// Indicium `simple` search provides four types of search. The best search type
/// that should be used really depends on your use-case: the nature of the data,
/// the intent of the user, and the size of your data set.
///
/// Support for `and` and `or` keywords inside the search string is not
/// currently planned for the `simple` search engine since the intent is to have
/// a relatively simple implementation.
///
/// For more information on the setting the search type in a `SearchIndex` type
/// see: [`SearchIndexBuilder`] or [`SearchIndex::new()`].
///
/// [`SearchIndexBuilder`]: struct.SearchIndexBuilder.html
/// [`SearchIndex::new()`]: struct.SearchIndex.html#method.new

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SearchType {
    /// `Live` search allows for "search as you type." It is a hybridization
    /// of `autocomplete` and `search`. This method will effectively search
    /// all of the autocompletion options and return the search results to the
    /// caller.
    ///
    /// This search method accepts multiple keywords in the search string. The
    /// logical conjuction for multiple keywords is `And`. For example, a search
    /// of `this that` will only return records containing keywords both `this`
    /// **and** `that`. In other words, _all_ keywords must be present in a
    /// record for it to be returned as a result.
    ///
    /// You may change & control the ordering of your records by manually
    /// implementing the [Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html)
    /// trait for your `K` key.
    ///
    /// This conjuction uses the most CPU resources because the search results
    /// must be gathered for several autocompletion options.
    ///
    /// Modern Internet browsers often have a similar "type as you search"
    /// functionality in the address bar. _Spotify_ also has a cool
    /// implementation of a similar feature.
    Live,
    /// This search method accepts multiple keywords in the search string. The
    /// logical conjuction for multiple keywords is `And`. For example, a search
    /// of `this that` will only return records containing keywords both `this`
    /// **and** `that`. In other words, _all_ keywords must be present in a
    /// record for it to be returned as a result.
    ///
    /// You may change & control the ordering of your records by manually
    /// implementing the [Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html)
    /// trait for your `K` key.
    ///
    /// This conjuction uses less CPU resources than `Or`.
    ///
    /// The `And` search feels more like "use my keywords to filter out the
    /// records I don't want." It's likely a better choice for large collections
    /// because it uses less CPU resouces than `Or`.
    ///
    /// Probably best suited in a filter widget.
    And,
    /// This search method accepts multiple keywords in the search string. The
    /// logical conjuction for multiple keywords is `Or`. For example, a search
    /// of `this that` will return records containing keywords `this` **or**
    /// `that`. In other words, _any_ keyword can be present in a record for it
    /// to be returned as a result.
    ///
    /// The results are returned in order of descending relevance. Records
    /// containing both keywords `this` and `that` will be the top results.
    ///
    /// This conjuction uses more CPU resources than `And` because the keyword
    /// hits must be tallied and sorted.
    ///
    /// If your collection contains less than 10,000 records, `Or` might be a
    /// good place to start. To me, `Or` effectively feels like "using these
    /// keywords, find a record I might want" which works well if there aren't
    /// too many records.
    ///
    /// Probably best suited for a search results screen.
    Or,
    /// The search string is expected to only contain a single keyword. This is
    /// the lightest and fastest search type. It is good for compact interfaces,
    /// where records are very simple, and data-sets are quite small.
    ///
    /// You may change & control the ordering of your records by manually
    /// implementing the [Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html)
    /// trait for your `K` key.
    ///
    /// Probably best suited in a form widget.
    Keyword,
} // SearchType