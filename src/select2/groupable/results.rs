use crate::select2::groupable::{Group, Groupable, GroupableRecord, GroupedResults};
use crate::select2::{Pagination, Record, Request};
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::string::ToString;

// -----------------------------------------------------------------------------

impl Request {

    /// This function will not perform the `term` or `q` search in the query. Any
    /// requested search much be performed by the caller, and the search results
    /// can be processed into `select2` format using this function.
    ///
    /// If no search is requested, the caller can pass the collection (in the form
    /// of a slice) to this function to be processed into `select2` format.
    ///
    /// 1. Convert the query-string received from Select2 into a `Request` struct.
    /// 2. Search the index using the `search_select2` method and the `Request` struct.
    /// 3. If desired, filter the search results.
    /// 4. Look-up references to full records in collections from the keys returned from `search_select2` in step #2.
    /// 5. **You are here.** Use the `results` method to produce the `Results` struct.
    /// 6. Convert the `Results` struct into `JSON` and return it to the client.

    pub fn grouped_results<K: Clone + Debug + Display + Eq + Hash + PartialEq + ToString, G: Groupable>(
        &self,
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
        // `key` represents the group, and the `value` represents the
        // `GroupableRecord`:
        let mut grouped_results: HashMap<String, Vec<Record>> = HashMap::new();

        // Iterate over the results and insert them into their respective groups:
        groupable_results.1
            // Iterate over the results records:
            .iter()
            // For each record in the results:
            .for_each(|(key, value)| {

                let groupable_record: GroupableRecord = value.record();

                match grouped_results.get_mut(&groupable_record.group) {
                    // If its group exists in hash map, add record to the group:
                    Some(group) => { group.push(groupable_record.to_record(key)) },
                    // If group does not exist, initialize with group with this record:
                    None => { grouped_results.insert(groupable_record.group.to_owned(), vec![groupable_record.to_record(key)]); },
                } // match

            }); // for_each









        let mut grouped_results: Vec<Group> = grouped_results
            .iter()
            .map(|(group, records)| Group {
                text: group.to_string(),
                children:
                    records
                    .iter()
                    .cloned()
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

} // impl