use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------



pub struct Select2Request {
    term: Option<String>,
    q: Option<String>,
    _type: Option<String>,
    page: Option<usize>,
} // Select2Record


pub struct Select2Record<K: Clone + Debug + Eq + Hash + PartialEq> {
    id: K,
    text: String,
    disabled: bool,
} // Select2Record

// -----------------------------------------------------------------------------
//
/// To make a struct indexable, the programmer must implement the `Selectable`
/// trait for it. The trait returns a `Select2Record` of all content needed to
/// make it usable with the `select2.js` plugin.

pub trait Selectable<K: Clone + Debug + Eq + Hash + PartialEq> {
    fn record(&self) -> Select2Record<K>;
} // Selectable




pub fn select2<K: Clone + Debug + Eq + Hash + PartialEq, I: Iterator>(
    request: Select2Request,
    iterator: I,
    selected_record: K,
    items_per_page: usize,
) -> Vec<Select2Record<K>> {

    iterator
        .skip(request.page * items_per_page)
        .take(items_per_page)



    vec![]

}