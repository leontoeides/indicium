use std::fmt::Debug;


// -----------------------------------------------------------------------------

struct TypeaheadRecord {
    /// The `prefetch` option is available for large record sets. For small
    /// data-sets, `prefetch` could be set to `None` for all records. For
    /// large data-sets, frequently referenced records could be set to `true`
    /// and the record will be included in the prefetch data-set. Unfrequently
    /// used records will only be available on queries to the server.
    prefetch: Option<bool>,
    /// This is the `String` title or name that represents this record.
    value: String,
}



// -----------------------------------------------------------------------------
//
/// To make a struct servable for the `typeahead.js` server, the programmer must
/// implement the `TypeaheadServable` trait for it. The trait returns a
/// `String` of they string that is to be returned.

pub trait TypeaheadServable {
    fn value(&self) -> TypeaheadRecord;
} // TypeaheadServable


// -----------------------------------------------------------------------------
//
/// Structure that represents a search index.

#[derive(Debug)]
pub struct TypeaheadServer {
    /// The search index data structure.
    vec: Vec<String>,
} // TypeaheadServer

