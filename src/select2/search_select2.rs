//! `Select2` interfaces to the `simple::SearchIndex`.

use crate::select2::Request;
use crate::simple::{SearchIndex, SearchType};
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<'a, K: 'a + Hash + Ord> SearchIndex<K> {

    /// Once the client's `Select2` query-string has been parsed into a
    /// `Request` struct, the struct may be passed to this search method. This
    /// method will return all search results for the client's query.

    #[tracing::instrument(level = "trace", name = "Select2 Search", skip(self))]
    pub fn search_select2(
        &'a self,
        request: &'a Request,
    ) -> Option<Vec<&'a K>> {

        // Get query (search term) if any:
        let query_term: Option<&String> = request.query_term();

        // Search index for query/term:
        query_term.as_ref().map(|query|
            self.search_with(
                &SearchType::Live,
                &self.maximum_keys_per_keyword(),
                query,
            ) // search_with
        ) // query_term

    } // fn

} // impl