mod selectable;
mod tests;

// -----------------------------------------------------------------------------

pub use crate::select2::selectable::Selectable;
pub use crate::select2::selectable::results as selectable_results;

// -----------------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------
//
/// Select2 will issue a request to the specified URL when the user opens the
/// control (unless there is a `minimumInputLength` set as a Select2 option),
/// and again every time the user types in the search box. By default, it will
/// send the following as query string parameters:

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Request {
    /// The current search term in the search box.
    pub term: Option<String>,
    /// Contains the same contents as `term`.
    pub q: Option<String>,
    /// A "request type". Will usually be `query`, but changes to `query_append`
    /// for paginated requests.
    #[serde(alias = "_type")]
    pub request_type: Option<String>,
    /// The current page number to request. Only sent for paginated (infinite
    /// scrolling) searches.
    pub page: Option<usize>,
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Pagination {
    pub more: bool,
} // Pagination

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
pub struct Record {
    /// Select2 requires that the `id` property is used to uniquely identify the
    /// options that are displayed in the results list. If you use a property
    /// other than `id` (like `pk`) to uniquely identify an option, you need to
    /// map your old property to `id` before passing it to Select2.
    pub id: String,
    /// Just like with the `id` property, Select2 requires that the text that
    /// should be displayed for an option is stored in the `text` property.
    pub text: String,
    /// You can also supply the `selected` properties for the options in this
    /// data structure.
    pub selected: bool,
    /// You can also supply the `disabled` properties for the options in this
    /// data structure.
    pub disabled: bool,
} // Record