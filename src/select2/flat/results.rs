use crate::select2::flat::{FlatResponse, Selectable};
use crate::select2::{Pagination, Record, Request};
use std::clone::Clone;

// -----------------------------------------------------------------------------

impl Request {

    /// This function does not perform the `term` or `q` search for the client
    /// request. The search much be performed by the caller using the
    /// `search_select2` method. The search results are passed to this function
    /// to build the response.
    ///
    /// If no search is requested, the caller can pass the entire collection (in
    /// the form of a slice) to this function to be processed into the `Select2`
    /// format.
    ///
    /// Steps for processing a `Select2` request:
    /// 1. Convert the query-string received from the Select2 plug-in into a `Request` struct.
    /// 2. Search the index using the `search_select2` method, supplying it with the `Request` struct.
    /// 3. If desired, filter (and further process) the search results.
    /// 4. Look-up references to full records in collections using the keys returned from `search_select2` method in step #2.
    /// 5. **You are here.** Use the `Request::results` method to produce the `Response` struct.
    /// 6. Convert the `Response` struct into `JSON` and return it to the client.

    #[tracing::instrument(level = "trace", name = "Build Flat Response", skip(self, search_results_keys, search_results_values))]
    pub fn flat_results<K: Clone + Ord + ToString, S: Selectable>(
        &self,
        items_per_page: &Option<usize>,
        selected_record: &Option<String>,
        search_results_keys: &[K],
        search_results_values: &[S]
    ) -> FlatResponse {

        let search_results: Vec<(&K, &S)> = search_results_keys
            .iter()
            .zip(search_results_values.iter())
            .collect();

        // If the caller specifies a maximum number of items per page, then
        // consider pagination turned on:
        // self.request_type == Some("query_append".to_string())
        if let Some(items_per_page) = items_per_page {

            // Ensure that the `page` number is set correctly before processing:
            let page = match self.page {
                // If no page number specified, assume page 1:
                None => 1,
                // There is no page 0. Assume caller meant page 1:
                Some(0) => 1,
                // Otherwise continue with caller's page number:
                _ => self.page.unwrap(),
            }; // match

            // This function works on the resolved output of a search, or the
            // records dumped from a key-value store:
            let paginated_results: Vec<Record> = search_results
                // Iterate over each passed record:
                .iter()
                // Skip records so we start at beginning of specified `page`:
                .skip(items_per_page * (page - 1))
                // Only take a page's worth of records:
                .take(*items_per_page)
                // Convert internal `SelectableRecord` format to output `Record`
                // format:
                .map(|(key, value)|
                    value.record().to_record(&key.to_string())
                ) // map
                // Check if this record was specified as being selected:
                .map(|mut record| {
                    // Check if the `selected_record` was set...
                    if let Some(selected_record) = selected_record {
                        // ...was set. Update record with comparison result and
                        // return record:
                        record.selected = record.id == *selected_record;
                        record
                    } else {
                        // ...wasn't set, return record as-is:
                        record
                    } // if
                }) // map
                // Collect all Select2 records into a `Vec<Record>`:
                .collect();

            // Return Select2 `FlatResponse` to caller:
            FlatResponse {
                results: paginated_results,
                pagination: Pagination {
                    more: items_per_page * page < search_results.len(),
                }, // Pagination
            } // FlatResponse

        } else {

            // This function works on the resolved output of a search, or the
            // records dumped from a key-value store:
            let unpaginated_results = search_results
                // Iterate over each passed record:
                .iter()
                // Convert internal `SelectableRecord` format to output `Record`
                // format:
                .map(|(key, value)|
                    value.record().to_record(&key.to_string())
                ) // map
                // Check if this record was specified as being selected:
                .map(|mut record| {
                    // Check if the `selected_record` was set...
                    if let Some(selected_record) = selected_record {
                        // ...was set. Update record with comparison result and
                        // return record:
                        record.selected = record.id == *selected_record;
                        record
                    } else {
                        // ...wasn't set, return record as-is:
                        record
                    } // if
                }) // map
                // Collect all select2 records into a `Vec<Record>`:
                .collect();

            // Return Select2 `FlatResponse` to caller:
            FlatResponse {
                results: unpaginated_results,
                pagination: Pagination { more: false }
            } // FlatResponse

        } // if

    } // fn

} // impl