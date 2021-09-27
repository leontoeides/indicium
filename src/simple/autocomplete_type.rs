use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Indicium `simple` search provides three types of autocompletions. The
/// autocompletion type that should be used depends on the user interface and
/// the data-set.
///
/// The `Contextual` autocompletion provides the best user experience. However,
/// if your data-set is very large with highly repetitive keywords, it is
/// recommended to use the `Global` autocompletion instead. `Keyword` is the
/// most efficient autocompletion and is good for compact interfaces or where
/// records are very simple.
///
/// For more information about the setting the autocompletion type in the
/// `SearchIndex` type see: [`SearchIndexBuilder`].
///
/// [`SearchIndexBuilder`]: struct.SearchIndexBuilder.html

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum AutocompleteType {
    /// The search string is expected to only contain a single keyword. This is
    /// the lightest and fastest autocompletion type.
    Keyword,
    /// The final keyword in the search string will be autocompleted from all
    /// keywords in the search index.
    Global,
    /// The final keyword in the search string will be autocompleted by using
    /// the preceding keywords as a filter. This is the heaviest and slowest
    /// autocompletion type.
    Contextual,
} // AutocompleteType