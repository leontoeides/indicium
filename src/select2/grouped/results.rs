use crate::select2::grouped::{Group, Groupable, GroupableRecord, GroupedResults};
use crate::select2::{Pagination, Record, Request};
use std::cmp::{Eq, PartialEq};
use std::collections::BTreeMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::io::{Error, ErrorKind};
use std::string::ToString;

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
        name = "build grouped results",
        skip(self, search_results_keys, search_results_values)
    )]
    pub fn grouped_response<
        K: Clone + Debug + Display + Eq + Hash + PartialEq + ToString,
        G: Groupable,
    >(
        &self,
        items_per_page: &Option<usize>,
        selected_record: &Option<String>,
        search_results_keys: &[&K],
        search_results_values: &[&G],
    ) -> Result<GroupedResults, Error> {
        // Error checking. Ensure that there are the same number of keys and
        // values:

        if search_results_keys.len() != search_results_values.len() {
            let error_message = format!(
                "{} keys and {} values were supplied to `grouped_response`. \
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
            return Ok(GroupedResults::default());
        } // if

        // Observe pagination. If the caller specifies a maximum number of items
        // per page, then consider pagination turned on:

        // self.request_type == Some("query_append".to_string())
        let groupable_results: (bool, Vec<(&K, &G)>) = if let Some(items_per_page) = items_per_page
        {
            // Paginated response:

            // Get the `page` number from the request:
            let page: usize = self.page_number();

            // This function works on the resolved output of a search, or the
            // records dumped from a key-value store:
            let paginated_results: Vec<(&K, &G)> = search_results_keys
                // Iterate over each key:
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
                .map(|(index, key)| (*key, search_results_values[index]))
                // Collect all Select2 records into a `Vec<GroupableRecord>`:
                .collect();

            // Return pagination status and results to outer scope:
            (
                items_per_page * page < search_results_keys.len(),
                paginated_results,
            )
        } else {
            // Unpaginated response:

            // This function works on the resolved output of a search, or the
            // records dumped from a key-value store:
            let unpaginated_results: Vec<(&K, &G)> = search_results_keys
                // Iterate over each key:
                .into_iter()
                // Track the number of keys we've iterated over, so we can
                // look-up the corresponding values from the
                // `search_results_values` slice:
                .enumerate()
                // Look-up the `Groupable` value from the enumeration or index:
                .map(|(index, key)| (*key, search_results_values[index]))
                // Collect all select2 records into a `Vec<GroupableRecord>`:
                .collect();

            // Return pagination status and results to outer scope:
            (false, unpaginated_results)
        }; // if

        // This `BTreeMap` is used to organize the records into their groups.
        // The `key` represents the group, and the `value` represents the
        // records in the group:

        let mut grouped_results: BTreeMap<String, Vec<Record>> = BTreeMap::new();

        // Iterate over the results and insert them into their respective
        // groups:
        groupable_results
            .1
            // Iterate over the results records:
            .into_iter()
            // For each record in the results:
            .for_each(|(key, value)| {
                // Convert the record from a `&G` into a `GroupableRecord`:
                let groupable_record: GroupableRecord = value.record();
                // Convert the `GroupableRecord` to a `Select2` `Record`:
                let mut record = groupable_record.to_record(key);
                // Check if the `selected_record` was set...
                if let Some(selected_record) = selected_record {
                    // ...was set. Update record with comparison result and
                    // return record:
                    record.selected = record.id == *selected_record;
                } // if
                  // Attempt to get mutuable reference to the group entry in
                  // the B-tree map:
                match grouped_results.get_mut(&groupable_record.group) {
                    // If group exists in hash map, add record to the group:
                    Some(group) => group.push(record),
                    // If group does not exist, initialize with this record:
                    None => {
                        grouped_results.insert(groupable_record.group.to_owned(), vec![record]);
                    } // None
                } // match
            }); // for_each

        // Convert `BTreeMap<String, Vec<Record>>` structure used to organize
        // records into `Vec<Group>` which will be returned as the response:

        let grouped_results: Vec<Group> = grouped_results
            .into_iter()
            .map(|(group, records)| Group {
                text: group.to_string(),
                children: records.to_owned(),
            }) // map
            .collect();

        // Return Select2 `GroupedResults` to caller:

        Ok(GroupedResults {
            results: grouped_results,
            pagination: Pagination {
                more: groupable_results.0,
            }, // Pagination
        }) // GroupedResults
    } // fn
} // impl
