//! # Indicium Search
//!
//! A simple in-memory search for collections (Vec, HashMap, BTreeMap, etc) and
//! key-value stores. Features autocompletion.
//!
//! There are many incredible search engines available for Rust. Many seem to
//! require compiling a separate server binary. I wanted something simple, light
//! weight, and that could conveniently search structs and collections. So I
//! have made `indicium`.

#![doc(html_favicon_url = "https://www.arkiteq.ca/crates/indicium/icon.png")]
#![doc(html_logo_url = "https://www.arkiteq.ca/crates/indicium/logo.png")]

/// The simple Indicium search implementation. Fewer bells-and-whistles but
/// easier to use than the other options.
///
/// There will be more search implementations in future versions.
pub mod simple;
// Support for the popular `Select2` jQuery plug-in.
// pub mod select2;