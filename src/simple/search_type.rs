// -----------------------------------------------------------------------------
//
/// Indicium `simple` search provides three types of search. The search type
/// that should be used really depends on your use-case: the nature of the data,
/// the intent of the user, and the size of your data set. I would suggest
/// trying & testing both `And` and `Or` searches to see which one works better
/// for you. See variant descriptions for more information.
///
/// Support for `and` and `or` keywords inside the search string is not
/// currently planned for the `simple` search engine. The intent is to have a
/// relatively simple search engine.
///
/// For more information on the setting the search type in a `SearchIndex` type
/// see: [`SearchIndexBuilder`] or [`SearchIndex::new()`].
///
/// [`SearchIndexBuilder`]: struct.SearchIndexBuilder.html
/// [`SearchIndex::new()`]: struct.SearchIndex.html#method.new

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SearchType {
    /// The search string is expected to only contain a single keyword. Results
    /// are returned in lexographic order. This is the lightest and fastest
    /// search type. It is good for compact interfaces, where records are very
    /// simple, and data-sets are quite small.
    ///
    /// Probably best suited as a form widget.
    Keyword,
    /// This search method accepts multiple keywords in the search string. The
    /// logical conjuction for multiple keywords is `And`. For example, a search
    /// of `this that` will only return records containing keywords both `this`
    /// **and** `that`. In other words, _all_ keywords must be present in a
    /// record for it to be returned as a result.
    ///
    /// The results are returned in lexographic order. This conjuction uses less
    /// CPU resources than `Or`.
    ///
    /// The `And` search feels more like "use my keywords to filter out the
    /// records I don't want." It's likely a better choice for large collections
    /// because it uses less CPU resouces than `Or`.
    ///
    /// Probably best suited as a filter widget.
    And,
    /// This search method accepts multiple keywords in the search string. The
    /// logical conjuction for multiple keywords is `Or`. For example, a search
    /// of `this that` will return records containing keywords `this` **or**
    /// `that`. In other words, _any_ keyword can be present in a record for it
    /// to be returned as a result.
    ///
    /// The results are returned in order of descending relevance. Records
    /// containing both keywords `this` and `that` will be the top results. This
    /// conjuction uses more CPU resources than `And` because the keyword hits
    /// must be tallied and sorted.
    ///
    /// If your collection contains less than 10,000 records, `Or` might be a
    /// good place to start. To me, `Or` effectively feels like "using these
    /// keywords, find a record I might want" which works well if there aren't
    /// too many records.
    ///
    /// Probably best suited for a search screen.
    Or,
} // SearchType