mod tests;

// -----------------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::string::ToString;

// -----------------------------------------------------------------------------
//
/// Select2 will issue a request to the specified URL when the user opens the
/// control (unless there is a `minimumInputLength` set as a Select2 option),
/// and again every time the user types in the search box. By default, it will
/// send the following as query string parameters:

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    /// The current search term in the search box.
    term: Option<String>,
    /// Contains the same contents as `term`.
    q: Option<String>,
    /// A "request type". Will usually be `query`, but changes to `query_append`
    /// for paginated requests.
    #[serde(alias = "_type")]
    request_type: Option<String>,
    /// The current page number to request. Only sent for paginated (infinite
    /// scrolling) searches.
    page: Option<usize>,
} // Record

// -----------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub struct Record {
    /// Select2 requires that the `id` property is used to uniquely identify the
    /// options that are displayed in the results list. If you use a property
    /// other than `id` (like `pk`) to uniquely identify an option, you need to
    /// map your old property to `id` before passing it to Select2.
    id: String,
    /// Just like with the `id` property, Select2 requires that the text that
    /// should be displayed for an option is stored in the `text` property.
    text: String,
    /// You can also supply the `disabled` properties for the options in this
    /// data structure.
    disabled: bool,
} // Record

// -----------------------------------------------------------------------------
//
/// Select2 supports pagination ("infinite scrolling") for remote data sources
/// out of the box.
///
/// To use pagination, you must tell Select2 to add any necessary pagination
/// parameters to the request by overriding the `ajax.data` setting. The current
/// page to be retrieved is stored in the `params.page` property.
///
/// Select2 will expect a `pagination.more` value in the response. The value of
/// `more` should be `true` or `false`, which tells Select2 whether or not there
/// are more pages of results available for retrieval:

#[derive(Debug)]
pub struct Pagination {
    more: bool,
} // Pagination

// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Results {
    results: Vec<Record>,
    pagination: Pagination,
} // Results

// -----------------------------------------------------------------------------
//
/// To make a struct Select2-ready, the programmer must implement the
/// `Selectable` trait for it. The trait returns a `Record` with all content
/// needed to make it usable with the `select2.org` Javascript plugin.

pub trait Selectable<K: Clone + Debug + Eq + Hash + PartialEq + ToString> {
    fn select2_record(&self) -> Record;
} // Selectable












































/*
// -----------------------------------------------------------------------------

pub struct GroupRecord<K: Clone + Debug + Eq + Hash + PartialEq + ToString, G: Display + PartialEq> {
    /// Select2 requires that the `id` property is used to uniquely identify the
    /// options that are displayed in the results list. If you use a property
    /// other than `id` (like `pk`) to uniquely identify an option, you need to
    /// map your old property to `id` before passing it to Select2.
    id: K,
    /// When options are to be generated in `<optgroup>` sections, options
    /// should be nested under the `children` key of each group object. The
    /// label for the group should be specified as the `text` property on the
    /// group's corresponding data object.
    group: G,
    /// Just like with the `id` property, Select2 requires that the text that
    /// should be displayed for an option is stored in the `text` property.
    text: String,
    /// You can also supply the `disabled` properties for the options in this
    /// data structure.
    disabled: bool,
} // Group

// -----------------------------------------------------------------------------

pub struct Group<K: Clone + Debug + Eq + Hash + PartialEq + ToString, G: Display + PartialEq> {
    /// The label for the group should be specified as the `text` property on
    /// the group's corresponding data object.
    text: String,
    /// When options are to be generated in `<optgroup>` sections, options
    /// should be nested under the `children` key of each group object.
    children: GroupRecord<K, G>,
} // Group

// -----------------------------------------------------------------------------

pub struct GroupResults<K: Clone + Debug + Eq + Hash + PartialEq + ToString, G: Display + PartialEq> {
    results: Vec<Group<K, G>>,
    pagination: Pagination,
} // Results
*/













// -----------------------------------------------------------------------------

pub fn results<K: Clone + Debug + Display + Eq + Hash + PartialEq, S: Selectable<K>>(
    request: &Request,
    search_results: &[S],
    selected_record: &Option<K>,
    items_per_page: &Option<usize>,
) -> Vec<Record> {

    if let (Some(mut page), Some(items_per_page)) = (request.page, items_per_page) {

        // 0-based indexing:
        if page > 0 { page -= 1 };

        search_results
            .iter()
            .skip(items_per_page * page)
            .take(*items_per_page)
            .map(|record| record.select2_record())
            .collect()

    } else {

        search_results
            .iter()
            .map(|record| record.select2_record())
            .collect()

    } // if

} // fn