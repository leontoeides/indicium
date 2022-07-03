// -----------------------------------------------------------------------------
//
/// Indicium `simple` search provides three types of autocomplete. The
/// autocompletion type that should be used depends on the user interface and
/// the data-set. See variant descriptions for more information.
///
/// You may change & control the ordering of the autocomplete options by
/// manually implementing the
/// [Ord](https://doc.rust-lang.org/std/cmp/trait.Ord.html) trait for your `K`
/// key.
///
/// For more information on the setting the autocompletion type in a
/// `SearchIndex` type see: [`SearchIndexBuilder`] or [`SearchIndex::new()`].
///
/// [`SearchIndexBuilder`]: struct.SearchIndexBuilder.html
/// [`SearchIndex::new()`]: struct.SearchIndex.html#method.new

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AutocompleteType {
    /// The search string may contain multiple keywords and the last (partial)
    /// keyword will be autocompleted. The last keyword in the search string
    /// will be autocompleted by using the preceding keywords as a filter. This
    /// effectively provides contextual autocompletion. This is the heaviest and
    /// slowest autocompletion type but probably provides the best user
    /// experience.
    Context,
    /// The search string may contain multiple keywords and the last (partial)
    /// keyword will be autocompleted. The last keyword in the search string
    /// will be autocompleted from all available keywords in the search index.
    /// If your data-set is very large or has repetitive keywords (see also: the
    /// [`profile`] utility method), this is the recommended autocomplete type.
    ///
    /// [`profile`]: struct.SearchIndex.html#method.profile
    Global,
    /// The search string is expected to only contain a single keyword. This is
    /// the lightest and fastest autocompletion type. It is good for compact
    /// interfaces or where records are very simple.
    Keyword,
} // AutocompleteType