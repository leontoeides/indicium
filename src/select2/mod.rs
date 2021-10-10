//! Server-side support for the popular `Select2` jQuery plug-in. Select2 gives
//! you a customizable HTML select box with support for searching, tagging,
//! remote data sets, infinite scrolling, and many other highly used options.

// Directories:
pub mod groupable;
pub mod selectable;

// Methods & structs:
pub mod pagination;
pub mod record;
pub mod request;
pub mod search_select2;

// -----------------------------------------------------------------------------

pub use crate::select2::pagination::Pagination;
pub use crate::select2::record::Record;
pub use crate::select2::request::Request;