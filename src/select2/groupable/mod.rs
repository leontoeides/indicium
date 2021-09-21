mod tests;

// -----------------------------------------------------------------------------

use crate::select2::{Pagination, Record, Request};
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
/// `GroupRecord` with all content needed to make it usable with the
/// `select2.org` Javascript plugin.

pub trait Groupable<K: Clone + Debug + Eq + Hash + PartialEq + ToString> {
    fn select2_grouped_record(&self) -> GroupRecord;
} // Groupable

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct GroupRecord {
    /// Select2 requires that the `id` property is used to uniquely identify the
    /// options that are displayed in the results list. If you use a property
    /// other than `id` (like `pk`) to uniquely identify an option, you need to
    /// map your old property to `id` before passing it to Select2.
    pub id: String,
    /// When options are to be generated in `<optgroup>` sections, options
    /// should be nested under the `children` key of each group object. The
    /// label for the group should be specified as the `text` property on the
    /// group's corresponding data object.
    pub group: String,
    /// Just like with the `id` property, Select2 requires that the text that
    /// should be displayed for an option is stored in the `text` property.
    pub text: String,
    /// You can also supply the `selected` properties for the options in this
    /// data structure.
    pub selected: bool,
    /// You can also supply the `disabled` properties for the options in this
    /// data structure.
    pub disabled: bool,
} // Group

// -----------------------------------------------------------------------------

impl std::convert::From<&GroupRecord> for Record {
    /// Converts a `GroupRecord` struct to a selectable `Record` struct.
    fn from(group_record: &GroupRecord) -> Self {
        Record {
            id: group_record.id.clone(),
            text: group_record.text.clone(),
            selected: group_record.selected,
            disabled: group_record.disabled,
        } // Record
    } // fn from
} // impl From

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
//
/// This function will not perform the `term` or `q` search in the query. Any
/// requested search much be performed by the caller, and the search results
/// can be processed into `select2` format using this function.
///
/// If no search is requested, the caller can pass the collection (in the form
/// of a slice) to this function to be processed into `select2` format.

pub fn results<K: Clone + Debug + Display + Eq + Hash + PartialEq, G: Groupable<K>>(
    request: &Request,
    items_per_page: &Option<usize>,
    records: &[G],
    selected_record: &Option<String>,
) -> GroupedResults {

    // If the caller specifies a maximum number of items per page, then consider
    // pagination turned on:
    // request.request_type == Some("query_append".to_string())
    let groupable_results = if let Some(items_per_page) = items_per_page {

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
        let paginated_results: Vec<GroupRecord> = records
            // Iterate over each passed record:
            .iter()
            // Skip records so we start at beginning of specified `page`:
            .skip(items_per_page * (page - 1))
            // Only take a page's worth of records:
            .take(*items_per_page)
            // Use `Selectable` trait method `select2_grouped_record` to convert
            // from user record to select2 grouped record:
            .map(|value| value.select2_grouped_record())
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
            // Collect all select2 records into a `Vec<GroupRecord>`:
            .collect();

        // Return pagination status and results to outer scope:
        (items_per_page * page < records.len(), paginated_results)

    } else {

        // This function works on the resolved output of a search, or the
        // records dumped from a key-value store:
        let unpaginated_results = records
            // Iterate over each passed record:
            .iter()
            // Use `Selectable` trait method `select2_grouped_record` to convert
            // from user record to select2 grouped record:
            .map(|record| record.select2_grouped_record())
            // Collect all select2 records into a `Vec<Record>`:
            .collect();

        // Return pagination status and results to outer scope:
        (false, unpaginated_results)

    }; // if

    // This `HashMap` is used to organize the records into their groups. The
    // `key` represents the group, and the `value` represents the `Record`:
    let mut grouped_results: HashMap<String, Vec<Record>> = HashMap::new();

    // Iterate over the results and insert them into their respective groups:
    groupable_results.1
        // Iterate over the results records:
        .iter()
        // For each record in the results:
        .for_each(|record| match grouped_results.get_mut(&record.group) {
            // If its group exists in hash map, add record to the group:
            Some(group) => { group.push(Record::from(record)) },
            // If group does not exist, initialize with group with this record:
            None => { grouped_results.insert(record.group.clone(), vec![Record::from(record)]); },
        }); // for_each

    let mut grouped_results: Vec<Group> = grouped_results
        .iter()
        .map(|(key, value)| Group {
            text: key.to_string(),
            children: value.clone(),
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