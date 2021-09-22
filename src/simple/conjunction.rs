// -----------------------------------------------------------------------------
//
/// The `indicium::simple` search engine provides two types of searches: one
/// with an `And` conjuction where all keywords must be present in the record.
/// Another with an `Or` conjuction where if any keyword must be present the
/// record will be returned.

pub enum Conjunction {
    And,
    Or,
} // Conjuction