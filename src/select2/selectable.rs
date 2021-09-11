use crate::select2::Pagination;
use crate::select2::Record;
use crate::select2::Request;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::string::ToString;

// -----------------------------------------------------------------------------
//
/// To make a struct Select2-ready, the programmer must implement the
/// `Selectable` trait for it. The trait returns a `Record` with all content
/// needed to make it usable with the `select2.org` Javascript plugin.

pub trait Selectable<K: Clone + Debug + Eq + Hash + PartialEq + ToString> {
    fn select2_record(&self) -> Record;
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
//
/// This function will not perform the `term` or `q` search in the query. Any
/// requested search much be performed by the caller, and the search results
/// should be processed into `select2` format using this function.
///
/// If no search is requested, the caller can pass the collection (in the form
/// of a slice) to this function to be processed into `select2` format.

pub fn results<K: Clone + Debug + Display + Eq + Hash + PartialEq, S: Selectable<K>>(
    request: &Request,
    records: &[S],
    selected_record: &Option<String>,
    items_per_page: &Option<usize>,
) -> Results {

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
        let results: Vec<Record> = records
            // Iterate over each passed record:
            .iter()
            // Skip records so we start at beginning of specified `page`:
            .skip(items_per_page * (page - 1))
            // Only take a page's worth of records:
            .take(*items_per_page)
            // Use `Selectable` trait method `select2_record` to convert from
            // user record to select2 record:
            .map(|value| value.select2_record())
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

        // Determine if there are more records to be displayed. This operation
        // is performed here to avoid a move of the `results` `Vec`:
        let more: bool = items_per_page * page < results.len();

        // Return `select2` `Results` to caller:
        Results {
            results,
            pagination: Pagination {
                more,
            },
        } // Results

    } else {

        // This function works on the resolved output of a search, or the
        // records dumped from a key-value store:
        let results = records
            // Iterate over each passed record:
            .iter()
            // Use `Selectable` trait method `select2_record` to convert from
            // user record to select2 record:
            .map(|record| record.select2_record())
            // Collect all select2 records into a `Vec<Record>`:
            .collect();

        // Return `select2` `Results` to caller:
        Results {
            results,
            pagination: Pagination { more: false }
        } // Results

    } // if

} // fn