use crate::select2::{Pagination, Record, Request, SelectableRecord};
use crate::simple::{SearchIndex, SearchType};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------
//
/// To make a struct Select2-ready, the programmer must implement the
/// `Selectable` trait for it. The trait returns a `Record` with the content
/// needed to make it usable with the Select2 Javascript plugin.

pub trait Selectable {
    fn record(&self) -> SelectableRecord;
} // Selectable

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Results {
    /// This format consists of a JSON object containing an array of objects
    /// keyed by the `results` key.
    pub results: Vec<Record>,
    /// The response object may also contain pagination data, if you would like
    /// to use the "infinite scroll" feature. This should be specified under the
    /// `pagination` key.
    pub pagination: Pagination,
} // Results

// -----------------------------------------------------------------------------

pub fn search<'a, K: Hash + Ord>(
    search_index: &'a SearchIndex<K>,
    request: &'a Request,
) -> Option<Vec<&'a K>> {

    // Get query (search term) if any:
    let query_term: Option<&String> = request.query_term();

    // Search index for query/term:
    query_term.as_ref().map(|query|
        search_index.search_with(
            &SearchType::Live,
            &search_index.maximum_keys_per_keyword(),
            query,
        ) // search_with
    ) // query_term

} // fn

// -----------------------------------------------------------------------------

pub fn results<K: Clone + Ord + ToString, S: Selectable>(
    request: &Request,
    items_per_page: &Option<usize>,
    selected_record: &Option<String>,
    search_results_keys: &[K],
    search_results_values: &[S]
) -> Results {

    let search_results: Vec<(&K, &S)> = search_results_keys
        .iter()
        .zip(search_results_values.iter())
        .collect();

    // If the caller specifies a maximum number of items per page, then consider
    // pagination turned on:
    // request.request_type == Some("query_append".to_string())
    if let Some(items_per_page) = items_per_page {

        // Ensure that the `page` number is set correctly before processing:
        let page = match request.page {
            // If no page number specified, assume page 1:
            None => 1,
            // There is no page 0. Assume caller meant page 1:
            Some(0) => 1,
            // Otherwise continue with caller's page number:
            _ => request.page.unwrap(),
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

        // Return Select2 `Results` to caller:
        Results {
            results: paginated_results,
            pagination: Pagination {
                more: items_per_page * page < search_results.len(),
            }, // Pagination
        } // Results

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

        // Return Select2 `Results` to caller:
        Results {
            results: unpaginated_results,
            pagination: Pagination { more: false }
        } // Results

    } // if

} // fn