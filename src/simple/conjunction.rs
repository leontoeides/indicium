// -----------------------------------------------------------------------------
//
/// The Indicium `simple` search provides two types of searches: one with an
/// `And` conjuction, where _all_ keywords can be present in a record to be
/// returned as a result. Another with an `Or` conjuction, where _any_ keyword
/// must be present in a record to be returned as a result. This `enum` is used
/// to select the desired search type.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Conjunction {
    /// All keywords must be present in a record for it to be returned as a
    /// result.
    And,
    /// Any keyword must be present in a record for it to be returned as a
    /// result.
    Or,
} // Conjuction