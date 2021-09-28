use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Indicium `simple` search provides three types of autocompletions. The
/// autocompletion type that should be used depends on the user interface and
/// the data-set. See variant descriptions for more information.
///
/// For more information on the setting the autocompletion type in a
/// `SearchIndex` type see: [`SearchIndexBuilder`] or [`SearchIndex.new()`].
///
/// [`SearchIndexBuilder`]: struct.SearchIndexBuilder.html
/// [`SearchIndex.new()`]: struct.SearchIndex.html#method.new

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum AutocompleteType {
    /// The search string is expected to only contain a single keyword. This is
    /// the lightest and fastest autocompletion type. It is good for compact
    /// interfaces or where records are very simple.
    Keyword,
    /// The final keyword in the search string will be autocompleted from all
    /// keywords in the search index. If your data-set is very large or has
    /// repetitive keywords, this is the recommended autocompletion type.
    Global,
    /// The final keyword in the search string will be autocompleted by using
    /// the preceding keywords as a filter. This is the heaviest and slowest
    /// autocompletion type but likely provides the best user experience.
    Contextual,
} // AutocompleteType