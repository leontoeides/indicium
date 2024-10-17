//! # Indicium Search
//!
//! 🔎 A simple in-memory search for collections (Vec, `HashMap`, `BTreeMap`, etc)
//! and key-value stores. Features autocompletion.
//!
//! There are many incredible search engines available for Rust. Many seem to
//! require compiling a separate server binary. I wanted something simple, light
//! weight, easy to use, and that could conveniently search structs and
//! collections in my binary. So, I made `indicium`.
//!
//! ![alt text](https://www.arkiteq.io/crates/indicium/banner.jpg "Indicium Search - In-Memory Search for Rust Collections")
//!
//! # What's New?
//!
//! * Release notes are available on
//!   [GitHub](https://github.com/leontoeides/indicium/releases).
//!
//! * The full [change
//!   log](https://github.com/leontoeides/indicium/blob/master/CHANGELOG.md) is
//!   available on GitHub.
//!
//! # Quick Start Guide
//!
//! For our **Quick Start Guide** example, we will be searching inside of the
//! following `struct`:
//!
//! ```rust
//! struct MyStruct {
//!     title: String,
//!     year: u16,
//!     body: String,
//! }
//! ```
//!
//! ## 1. Implementing Indexable
//!
//! To begin, we must make our record indexable. We'll do this by implementing
//! the `Indexable` trait for our `struct`. The idea is to return a `String` for
//! every field that we would like to be indexed. Example:
//!
//! ```rust
//! # struct MyStruct {
//! #   title: String,
//! #   year: u16,
//! #   body: String,
//! # }
//! #
//! use indicium::simple::Indexable;
//!
//! impl Indexable for MyStruct {
//!     fn strings(&self) -> Vec<String> {
//!         vec![
//!             self.title.clone(),
//!             self.year.to_string(),
//!             self.body.clone(),
//!         ]
//!     }
//! }
//! ```
//!
//! Don't forget that you may make numbers, numeric identifiers, enums, and
//! other types indexable by converting them to a `String` and including them in
//! the returned `Vec<String>`.
//!
//! ## 2. Indexing a Collection
//!
//! To index an existing collection, we can iterate over the collection. For
//! each record, we will insert it into the search index. This should look
//! something like these two examples:
//!
//! #### Vec
//!
//! ```rust
//! # struct MyStruct {
//! #   title: String,
//! #   year: u16,
//! #   body: String,
//! # }
//! #
//! # use indicium::simple::Indexable;
//! #
//! # impl Indexable for MyStruct {
//! #   fn strings(&self) -> Vec<String> {
//! #       vec![
//! #           self.title.clone(),
//! #           self.year.to_string(),
//! #           self.body.clone(),
//! #       ]
//! #   }
//! # }
//! use indicium::simple::SearchIndex;
//!
//! let my_vec: Vec<MyStruct> = Vec::new();
//!
//! // In the case of a `Vec` collection, we use the index as our key. A
//! // `Vec` index is a `usize` type. Therefore we will instantiate
//! // `SearchIndex` as `SearchIndex<usize>`.
//!
//! let mut search_index: SearchIndex<usize> = SearchIndex::default();
//!
//! my_vec
//!     .iter()
//!     .enumerate()
//!     .for_each(|(index, element)|
//!         search_index.insert(&index, element)
//!     );
//! ```
//!
//! #### `HashMap`
//!
//! ```rust
//! # struct MyStruct {
//! #   title: String,
//! #   year: u16,
//! #   body: String,
//! # }
//! #
//! # use indicium::simple::Indexable;
//! #
//! # impl Indexable for MyStruct {
//! #   fn strings(&self) -> Vec<String> {
//! #       vec![
//! #           self.title.clone(),
//! #           self.year.to_string(),
//! #           self.body.clone(),
//! #       ]
//! #   }
//! # }
//! #
//! use std::collections::HashMap;
//! use indicium::simple::SearchIndex;
//!
//! let my_hash_map: HashMap<String, MyStruct> = HashMap::new();
//!
//! // In the case of a `HashMap` collection, we use the hash map's key as
//! // the `SearchIndex` key. In our hypothetical example, we will use
//! // MyStruct's `title` as a the key which is a `String` type. Therefore
//! // we will instantiate `HashMap<K, V>` as HashMap<String, MyStruct> and
//! // `SearchIndex<K>` as `SearchIndex<String>`.
//!
//! let mut search_index: SearchIndex<String> = SearchIndex::default();
//!
//! my_hash_map
//!     .iter()
//!     .for_each(|(key, value)|
//!         search_index.insert(key, value)
//!     );
//! ```
//!
//! As long as the `Indexable` trait was implemented for your value type, the
//! above examples will index a previously populated `Vec` or `HashMap`.
//! However, the preferred method for large collections is to `insert` into the
//! `SearchIndex` as you insert into your collection (Vec, `HashMap`, etc.)
//!
//! It's recommended to wrap your target collection (your `Vec`, `HashMap`,
//! etc.) and this `SearchIndex` together in a new `struct` type. Then,
//! implement the `insert`, `replace`, `remove`, etc. methods for this new
//! `struct` type that will update both the collection and search index. This
//! will ensure that both your collection and index are always synchronized.
//!
//! Once the index has been populated, you can use the `search` and
//! `autocomplete` methods.
//!
//! ## 3. Searching
//!
//! The `search` method will return keys as the search results. Each resulting
//! key can then be used to retrieve the full record from its collection.
//!
//! Basic usage:
//!
//! ```rust
//! # use indicium::simple::{Indexable, SearchIndex};
//! # use pretty_assertions::assert_eq;
//! #
//! let mut search_index: SearchIndex<usize> = SearchIndex::default();
//!
//! search_index.insert(&0, &"Harold Godwinson");
//! search_index.insert(&1, &"Edgar Ætheling");
//! search_index.insert(&2, &"William the Conqueror");
//! search_index.insert(&3, &"William Rufus");
//! search_index.insert(&4, &"Henry Beauclerc");
//!
//! let resulting_keys: Vec<&usize> = search_index.search("William");
//!
//! assert_eq!(resulting_keys, vec![&2, &3]);
//!
//! // Demonstrating fuzzy matching:
//!
//! # #[cfg(feature = "strsim")]
//! let resulting_keys: Vec<&usize> = search_index.search("Harry");
//!
//! # #[cfg(feature = "strsim")]
//! assert_eq!(resulting_keys, vec![&0]);
//! ```
//!
//! Search only supports exact keyword matches. For `Live` searches, fuzzy
//! matching is only applied to the last keyword. Consider providing the
//! `autocomplete` feature to your users as an ergonomic alternative to fuzzy
//! matching.
//!
//! ## 4. Autocompletion
//!
//! The `autocomplete` method will provide several autocompletion options for
//! the last keyword in the supplied string.
//!
//! Basic usage:
//!
//! ```rust
//! # use indicium::simple::{AutocompleteType, Indexable, SearchIndex, SearchIndexBuilder};
//! # use pretty_assertions::assert_eq;
//! #
//! let mut search_index: SearchIndex<usize> =
//!     SearchIndexBuilder::default()
//!         .autocomplete_type(AutocompleteType::Global)
//!         .exclude_keywords(None)
//!         .build();
//!
//! search_index.insert(&0, &"apple");
//! search_index.insert(&1, &"ball");
//! search_index.insert(&3, &"bird");
//! search_index.insert(&4, &"birthday");
//! search_index.insert(&5, &"red");
//!
//! let autocomplete_options: Vec<String> =
//!     search_index.autocomplete("a very big bi");
//!
//! assert_eq!(
//!     autocomplete_options,
//!     vec!["a very big bird", "a very big birthday"]
//! );
//!
//! // Demonstrating fuzzy matching:
//!
//! # #[cfg(feature = "strsim")]
//! let autocomplete_options: Vec<String> =
//!     search_index.autocomplete("a very big birf");
//!
//! # #[cfg(feature = "strsim")]
//! assert_eq!(
//!     autocomplete_options,
//!     vec!["a very big bird", "a very big birthday"]
//! );
//! ```

#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::empty_line_after_doc_comments,
    clippy::module_name_repetitions,
    clippy::too_long_first_doc_paragraph
)]
#![doc(html_favicon_url = "https://www.arkiteq.io/crates/indicium/icon.png")]
#![doc(html_logo_url = "https://www.arkiteq.io/crates/indicium/logo.png")]

#[cfg(feature = "simple")]
pub mod simple;

#[cfg(feature = "select2")]
pub mod select2;
