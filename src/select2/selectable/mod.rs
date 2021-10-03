use crate::select2::{Pagination, Record, Request};
use crate::simple::SearchIndex;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

// -----------------------------------------------------------------------------
//
/// Select2 can render programmatically supplied data from an array or remote
/// data source (AJAX) as dropdown options. In order to accomplish this, Select2
/// expects a very specific data format. This format consists of a JSON object
/// containing an array of objects keyed by the `results` key.
///
/// Select2 requires that each object contain an `id` and a `text` property.
/// Additional parameters passed in with data objects will be included on the
/// data objects that Select2 exposes.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct SelectableRecord {
    /// Just like with the `id` property, Select2 requires that the text that
    /// should be displayed for an option is stored in the `text` property.
    pub text: String,
    /// You can also supply the `selected` properties for the options in this
    /// data structure.
    pub selected: bool,
    /// You can also supply the `disabled` properties for the options in this
    /// data structure.
    pub disabled: bool,
} // SelectableRecord

// -----------------------------------------------------------------------------
//
/// **The Select2 `Selectable` index**. This is the most important structure for
/// `Selectable` collections. See also `Groupable` collections.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SelectableIndex<'a, K: Ord> {
    /// Search index data structure.
    pub(crate) search_index: &'a SearchIndex<K>,
} // SelectableIndex

// -----------------------------------------------------------------------------
//
/// To make a struct Select2-ready, the programmer must implement the
/// `Selectable` trait for it. The trait returns a `Record` with all content
/// needed to make it usable with the Select2 Javascript plugin.

pub trait Selectable {
    fn select2_record(&self) -> SelectableRecord;
} // Selectable

// -----------------------------------------------------------------------------

fn selectable_record_to_select2_record(
    key: &str,
    value: &SelectableRecord,
) -> Record {
    Record {
        id: key.to_string(),
        text: value.text.clone(),
        selected: value.selected,
        disabled: value.disabled,
    } // Record
} // fn

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
/// can be processed into `Select2` format using this function.
///
/// If no search is requested, the caller can pass the collection (in the form
/// of a slice) to this function to be processed into `Select2` format.

impl<'a, K: Clone + Ord> SelectableIndex<'_, K> {

    pub fn new(search_index: &'a SearchIndex<K>) -> SelectableIndex<K> {
        SelectableIndex {
            search_index,
        } // SelectableIndex
    } // fn

    pub fn insert<K: FromStr + ToString, V: Selectable>(
        &mut self,
        key: &K,
        value: &V,
    ) {
        self.b_tree_map.insert(key.to_string(), value.select2_record());
    } // fn

    pub fn selectables(
        &self,
        request: &Request,
        items_per_page: &Option<usize>,
        selected_record: &Option<String>,
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
            let paginated_results: Vec<Record> = self.b_tree_map
                // Iterate over each passed record:
                .iter()
                // Skip records so we start at beginning of specified `page`:
                .skip(items_per_page * (page - 1))
                // Only take a page's worth of records:
                .take(*items_per_page)
                // Convert internal `SelectableRecord` format to output `Record`
                // format:
                .map(|(key, value)| selectable_record_to_select2_record(key, value))
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

            // Determine if there are more records to be displayed. This operation
            // is performed here (rather than in the `Results` instantiation) to
            // avoid a move of `paginated_results`:
            let more: bool = items_per_page * page < self.b_tree_map.len();

            // Return Select2 `Results` to caller:
            Results {
                results: paginated_results,
                pagination: Pagination {
                    more,
                },
            } // Results

        } else {

            // This function works on the resolved output of a search, or the
            // records dumped from a key-value store:
            let unpaginated_results = self.b_tree_map
                // Iterate over each passed record:
                .iter()
                // Convert internal `SelectableRecord` format to output `Record`
                // format:
                .map(|(key, value)| selectable_record_to_select2_record(key, value))
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

} // impl