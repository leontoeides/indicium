use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------
//
/// Your web application will receive query-strings from the Select2 plug-in
/// that need to be parsed into this `Request` struct.
///
/// Steps for processing a `Select2` request:
/// 1. **You are here.** Convert the query-string received from Select2 into a `Request` struct.
/// 2. Search the index using the `search_select2` method and the `Request` struct.
/// 3. If desired, filter the search results.
/// 4. Look-up references to full records in collections from the keys returned from `search_select2` in step #2.
/// 5. Use the `results` method to produce the `Results` struct.
/// 6. Convert the `Results` struct into `JSON` and return it to the client.
///
/// Select2 will issue a request to the specified URL when the user opens the
/// control (unless there is a `minimumInputLength` set as a Select2 option),
/// and again every time the user types in the search box. By default, it will
/// send the following as query string parameters:

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Request {
    /// The current search term in the search box.
    pub term: Option<String>,
    /// Contains the same contents as `term`.
    pub q: Option<String>,
    /// A "request type". Will usually be `query`, but changes to `query_append`
    /// for paginated requests.
    #[serde(alias = "_type")]
    pub request_type: Option<String>,
    /// The current page number to request. Only sent for paginated (infinite
    /// scrolling) searches.
    pub page: Option<usize>,
} // Record

// -----------------------------------------------------------------------------

impl Request {
    // For some reason, `Select2` can send the user's search term in either
    // the `term` field or in the `q` field. This convenience method checks both
    // fields and returns the user's query term, if available:
    pub fn query_term(&self) -> Option<&String> {
        // Get query (search term) if any:
        match &self.q {
            Some(_q) => self.q.as_ref(),
            None => match &self.term {
                Some(_term) => self.term.as_ref(),
                None => None,
            }, // None
        } // match
    } // fn
} // impl