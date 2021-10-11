use crate::select2::Request;
use crate::simple::{SearchIndex, SearchType};
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<'a, K: 'a + Hash + Ord> SearchIndex<K> {

    /// Once the client's `Select2` query-string has been parsed into a
    /// `Request` struct, the struct may be passed to this search method. This
    /// method will return all search results for the client's query.
    ///
    /// Steps for processing a `Select2` request:
    /// 1. Convert the query-string received from the Select2 plug-in into a `Request` struct.
    /// 2. **You are here.** Search the index using the `search_select2` method, supplying it with the `Request` struct.
    /// 3. If desired, filter (and further process) the search results.
    /// 4. Look-up references to full records in collections using the keys returned from `search_select2` method in step #2.
    /// 5. Use the `Request::results` method to produce the `Response` struct.
    /// 6. Convert the `Response` struct into `JSON` and return it to the client.

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