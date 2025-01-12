//! Server-side support for the popular
//! [Select2](https://select2.org/) jQuery plug-in. Select2 gives you a
//! customizable
//! [HTML \<select\> box](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select)
//! with support for searching, tagging, remote data sets, infinite scrolling,
//! and many other highly used options.
//!
//! **The documentation for Select2 server-side support is a bit wanting at
//! the moment. I will continue working on improving this.**
//!
//! Before you begin: implement the [`Selectable`] and/or [`Groupable`]
//! traits for your `struct`.
//!
//! [`Selectable`]: flat/trait.Selectable.html
//! [`Groupable`]: grouped/trait.Groupable.html
//!
//! What is the difference between `flat` response and `grouped` response? A
//! `grouped` response means that there is support for
//! [\<optgroup\>](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/optgroup)
//! in the response, while `flat` provides no `<optgroup>` support.
//!
//! Steps for processing a Select2 request:
//!
//! 1. Setup a route to your Select2 server in your Rust web framework. In your
//!    route function, convert the query-string received from the Select2
//!    plug-in into a [`Request`] struct. Your chosen web framework should
//!    provide some facilities for processing a query-string into a `struct`.
//!
//! 2. Your search index must already be initialized. Search the index using the
//!    [`search_select2`] method, supplying it with the `Request` struct you've
//!    built from the query-string. This search function will return keys as the
//!    search results.
//!
//! 3. If desired, filter (and further process) the search results.
//!
//! 4. Using the keys returned from the `search_select2` search, get the values
//!    from your collection. This crate does not know how to look up values from
//!    your collection, so you must get them.
//!
//! 5. Use either the [`flat_response`] or [`grouped_response`] method to
//!    produce a response for the Select2 plug-in, providing:
//!     * the `Request` struct _in step #1_,
//!     * the keys from `search_select2` _in step #2_,
//!     * and the values you got from your collection _in step #4_,
//!
//! [`flat_response`]: struct.Request.html#method.flat_response
//! [`grouped_response`]: struct.Request.html#method.grouped_response
//!
//! 6. Depending on whether flat or grouped output was selected, convert the
//!    [`FlatResults`] or [`GroupedResults`] struct into `JSON` and return it to
//!    the client.
//!
//! [`FlatResults`]: flat/struct.FlatResults.html
//! [`GroupedResults`]: grouped/struct.GroupedResults.html

// Directories:
pub mod flat;
pub mod grouped;

// Methods & structs:
pub mod search_select2;

// -----------------------------------------------------------------------------

use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------
//
/// Select2 supports pagination ("infinite scrolling") for remote data sources
/// out of the box.
///
/// To use pagination, you must tell Select2 to add any necessary pagination
/// parameters to the request by overriding the `ajax.data` setting. The current
/// page to be retrieved is stored in the `params.page` property.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Pagination {
    /// Select2 will expect a `pagination.more` value in the response. The value
    /// of `more` should be `true` or `false`, which tells Select2 whether or
    /// not there are more pages of results available for retrieval:
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

// -----------------------------------------------------------------------------
//
/// Your web application will receive a query-string from the Select2 plug-in
/// that need to be parsed into this `Request` struct.
///
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

impl<'a> Request {
    /// For some reason, `Select2` can send the user's search term in either
    /// the `term` field or in the `q` field. This convenience method checks
    /// both fields and returns the user's query term, if available.
    #[must_use]
    #[allow(clippy::option_if_let_else)]
    pub fn query_term(&'a self, dump_keyword: Option<&'a str>) -> Option<&'a str> {
        // Get query (search term) if any:
        match &self.q {
            Some(q) => Some(q.as_str()),
            None => self.term.as_ref().map_or(dump_keyword, |term| Some(term.as_str())), // None
        } // match
    } // fn
} // impl

// -----------------------------------------------------------------------------

impl Request {
    /// This convenience method will return the appropriate page number for
    /// pagination.
    #[must_use]
    #[allow(clippy::missing_panics_doc)] // `None` is handled, `unwrap` is safe
    pub fn page_number(&self) -> usize {
        // Ensure that the `page` number is set correctly before processing:
        match self.page {
            // If no page number specified, assume page 1:
            // There is no page 0. Assume caller meant page 1:
            Some(0) | None => 1,
            // Otherwise continue with caller's page number:
            _ => self.page.unwrap(),
        } // match
    } // fn
} // impl
