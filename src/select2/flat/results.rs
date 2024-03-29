use crate::select2::flat::{FlatResults, Selectable};
use crate::select2::{Pagination, Record, Request};
use std::io::{Error, ErrorKind};

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

    #[tracing::instrument(
        level = "trace",
        name = "build flat results",
        skip(self, search_results_keys, search_results_values)
    )]
    pub fn flat_response<K: Clone + Ord + ToString, S: Selectable>(
        &self,
        items_per_page: &Option<usize>,
        selected_record: &Option<String>,
        search_results_keys: &[&K],
        search_results_values: &[&S],
    ) -> Result<FlatResults, Error> {
        // Error checking. Ensure that there are the same number of keys and
        // values:

        if search_results_keys.len() != search_results_values.len() {
            let error_message = format!(
                "{} keys and {} values were supplied to `flat_response`. \
                the number of keys and values must be the same.",
                search_results_keys.len(),
                search_results_values.len(),
            ); // format!
            tracing::error!("{}", error_message);
            return Err(Error::new(ErrorKind::InvalidData, error_message));
        } else if search_results_keys.is_empty() {
            let error_message = "list of keys and values is empty. \
                returning empty response."
                .to_string();
            tracing::debug!("{}", error_message);
            return Ok(FlatResults::default());
        } // if

        // Observe pagination. If the caller specifies a maximum number of items
        // per page, then consider pagination turned on:

        // self.request_type == Some("query_append".to_string())
        if let Some(items_per_page) = items_per_page {
            // Paginated response:

            // Get the `page` number from the request:
            let page: usize = self.page_number();

            // This function works on the resolved output of a search, or the
            // records dumped from a key-value store:
            let paginated_results: Vec<Record> = search_results_keys
                // Iterate over each passed record:
                .into_iter()
                // Track the number of keys we've iterated over, so we can
                // look-up the corresponding values from the
                // `search_results_values` slice:
                .enumerate()
                // Skip records so we start at beginning of specified `page`:
                .skip(items_per_page * (page - 1))
                // Only take a page's worth of records:
                .take(*items_per_page)
                // Look-up the `Groupable` value from the enumeration or index:
                .map(|(index, key)| (*key, &search_results_values[index]))
                // Convert internal `SelectableRecord` format to output `Record`
                // format:
                .map(|(key, value)| value.record().to_record(&key.to_string())) // map
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

            // Return Select2 `FlatResults` to caller:
            Ok(FlatResults {
                results: paginated_results,
                pagination: Pagination {
                    more: items_per_page * page < search_results_keys.len(),
                }, // Pagination
            }) // FlatResults
        } else {
            // Unpaginated response:

            // This function works on the resolved output of a search, or the
            // records dumped from a key-value store:
            let unpaginated_results = search_results_keys
                // Iterate over each passed record:
                .into_iter()
                // Track the number of keys we've iterated over, so we can
                // look-up the corresponding values from the
                // `search_results_values` slice:
                .enumerate()
                // Look-up the `Groupable` value from the enumeration or index:
                .map(|(index, key)| (*key, &search_results_values[index]))
                // Convert internal `SelectableRecord` format to output `Record`
                // format:
                .map(|(key, value)| value.record().to_record(&key.to_string())) // map
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

            // Return Select2 `FlatResults` to caller:
            Ok(FlatResults {
                results: unpaginated_results,
                pagination: Pagination { more: false },
            }) // FlatResults
        } // if
    } // fn
} // impl
