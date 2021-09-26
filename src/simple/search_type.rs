use serde::{Deserialize, Serialize};

// -----------------------------------------------------------------------------
//
/// Indicium `simple` search provides three types of searches.
///
/// ### Which Search Should I Choose?
///
/// It really depends on your use-case: the nature of the data, the intent of
/// the user, and the size of your data set. I would suggest trying & testing
/// both `And` and `Or` to see which one works better for you.
///
/// The `And` search feels more like "use my keywords to filter out the records
/// I don't want." It's also a better choice for large collections because it
/// uses less CPU resouces than `Or`.
///
/// If your collection contains less than 10,000 records, `Or` might be a good
/// place to start. To me, Or effectively feels like "using these keywords, find
/// the best the record I might want" which works well if there aren't too
/// records. It's also worth noting that this conjuction uses more CPU resources
/// because the results must be tallied and sorted in order of relevance.
///
/// Support for `and` and `or` keywords in the search string is not currently
/// planned because the intent is to have a relatively simple search engine.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum SearchType {
    /// The search string is expected to only contain a single keyword. This is
    /// the lightest and fastest type.
    Keyword,
    /// All keywords in the search string must be present in a record for it to
    /// be returned as a result.
    And,
    /// Any keyword in the search string can be present in a record for it to be
    /// returned as a result.
    Or,
} // SearchType