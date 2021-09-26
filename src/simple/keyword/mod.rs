//! The `keyword_autocomplete` and `keyword_search` methods work on strings that
//! are expected to contain only a single keyword (as opposed to strings
//! containing multiple keywords.) For small collections, these methods might be
//! a lighter-weight alternative to their big brothers.

mod autocomplete;
mod search;