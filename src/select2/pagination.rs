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
///
/// Select2 will expect a `pagination.more` value in the response. The value of
/// `more` should be `true` or `false`, which tells Select2 whether or not there
/// are more pages of results available for retrieval:

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Pagination {
    pub more: bool,
} // Pagination