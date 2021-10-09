// -----------------------------------------------------------------------------

use crate::select2::{Pagination, Record, Request, SelectableRecord};
use crate::simple::{SearchIndex, SearchType};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::string::ToString;

// -----------------------------------------------------------------------------
//
/// To make a struct Select2-ready with support for grouped data, the programmer
/// must implement the `Groupable` trait for it. The trait returns a
/// `GroupableRecord` with all content needed to make it usable with the
/// `select2.org` Javascript plugin.

pub trait Groupable {
    fn record(&self) -> SelectableRecord;
    /// When options are to be generated in `<optgroup>` sections, options
    /// should be nested under the `children` key of each group object. The
    /// label for the group should be specified as the `text` property on the
    /// group's corresponding data object.
    fn group(&self) -> String;
} // Groupable

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Group {
    /// The label for the group should be specified as the `text` property on
    /// the group's corresponding data object.
    pub text: String,
    /// When options are to be generated in `<optgroup>` sections, options
    /// should be nested under the `children` key of each group object.
    pub children: Vec<Record>,
} // Group

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct GroupedResults {
    /// This format consists of a JSON object containing an array of objects
    /// keyed by the `results` key.
    pub results: Vec<Group>,
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
//
/// This function will not perform the `term` or `q` search in the query. Any
/// requested search much be performed by the caller, and the search results
/// can be processed into `select2` format using this function.
///
/// If no search is requested, the caller can pass the collection (in the form
/// of a slice) to this function to be processed into `select2` format.

pub fn results<K: Clone + Debug + Display + Eq + Hash + PartialEq, G: Groupable>(
    request: &Request,
    items_per_page: &Option<usize>,
    selected_record: &Option<String>,
    search_results_keys: &[K],
    search_results_values: &[G]
) -> GroupedResults {


    // Zip keys and values together:

    let search_results: Vec<(&K, &G)> = search_results_keys
        .iter()
        .zip(search_results_values.iter())
        .collect();



    // Observe pagination:

    // If the caller specifies a maximum number of items per page, then consider
    // pagination turned on:
    // request.request_type == Some("query_append".to_string())
    let groupable_results: (bool, Vec<&(&K, &G)>) = if let Some(items_per_page) = items_per_page {

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
        let paginated_results: Vec<&(&K, &G)> = search_results
            // Iterate over each passed record:
            .iter()
            // Skip records so we start at beginning of specified `page`:
            .skip(items_per_page * (page - 1))
            // Only take a page's worth of records:
            .take(*items_per_page)
            // Collect all Select2 records into a `Vec<GroupableRecord>`:
            .collect();

        // Return pagination status and results to outer scope:
        (items_per_page * page < search_results.len(), paginated_results)

    } else {

        // This function works on the resolved output of a search, or the
        // records dumped from a key-value store:
        let unpaginated_results: Vec<&(&K, &G)> = search_results
            // Iterate over each passed record:
            .iter()
            // Collect all select2 records into a `Vec<GroupableRecord>`:
            .collect();

        // Return pagination status and results to outer scope:
        (false, unpaginated_results)

    }; // if









    // This `HashMap` is used to organize the records into their groups. The
    // `key` represents the group, and the `value` represents the `SelectableRecord`:
    let mut grouped_results: HashMap<String, Vec<(&K, SelectableRecord)>> = HashMap::new();

    // Iterate over the results and insert them into their respective groups:
    groupable_results.1
        // Iterate over the results records:
        .iter()
        // For each record in the results:
        .for_each(|(key, value)| match grouped_results.get_mut(&value.group()) {
            // If its group exists in hash map, add record to the group:
            Some(group) => { group.push((key, value.record())) },
            // If group does not exist, initialize with group with this record:
            None => { grouped_results.insert(value.group(), vec![(key, value.record())]); },
        }); // for_each

    let mut grouped_results: Vec<Group> = grouped_results
        .iter()
        .map(|(group, records)| Group {
            text: group.to_string(),
            children:
                records
                .iter()
                .map(|(key, value)| value.to_record(&key.to_string()))
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
                .collect::<Vec<Record>>(),
        }) // map
        .collect();





    // The hash map would have removed any ordering of groups. Sort groups (but
    // not the children of the groups):
    grouped_results.sort_by(|a, b| a.text.partial_cmp(&b.text).unwrap());






    GroupedResults {
        results: grouped_results,
        pagination: Pagination {
            more: groupable_results.0,
        }, // Pagination
    } // GroupedResults






} // fn