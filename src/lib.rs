//! # Indicium Search
//!
//! 🔎 A simple in-memory search for collections (Vec, HashMap, BTreeMap, etc)
//! and key-value stores. Features autocompletion.
//!
//! There are many incredible search engines available for Rust. Many seem to
//! require compiling a separate server binary. I wanted something simple, light
//! weight, easy to use, and that could conveniently search structs and
//! collections in my binary. So, I made `indicium`.
//!
//! # What's New?
//!
//! * `0.3.4`: Peformance improvements.
//!
//! * `0.3.3`: Fix: `cargo test` failed.
//!
//! * `0.3.2`: Fix: issue with search indexes that do not use keyword splitting.
//!
//! * `0.3.1`: Autocomplete no longer offers previously used keywords as
//! options.
//!
//! * `0.3.1`: Added `maximum_keys_per_keyword` getter method.
//!
//! * `0.3.1`: Added `autocomplete_with` and `search_with` methods which allow
//! ad-hoc overrides of the `AutocompleteType`/`SearchType` and maximum results
//! parameters.
//!
//! * `0.3.0`: Added new search type `SearchType::Live` which is for "search as
//! you type" interfaces. It is sort of a hybrid between `autocomplete` and
//! `SearchType::And`. It will search using an (incomplete) string and return
//! keys as the search results. Each resulting key can then be used to retrieve
//! the full record from its collection to be rendered & displayed to the user.
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
//! // In the case of a `Vec` collection, we use the index as our key.  A
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
//! #### HashMap
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
//! `SearchIndex` as you insert into your collection (Vec, HashMap, etc.)
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
//! #
//! # struct MyType { text: String }
//! #
//! # impl From<&str> for MyType {
//! #   fn from(string: &str) -> Self {
//! #       MyType { text: string.to_string() }
//! #   }
//! # }
//! #
//! # impl Indexable for MyType {
//! #   fn strings(&self) -> Vec<String> {
//! #       vec![self.text.clone()]
//! #   }
//! # }
//! #
//! let mut search_index: SearchIndex<usize> = SearchIndex::default();
//!
//! search_index.insert(&0, &MyType::from("Harold Godwinson"));
//! search_index.insert(&1, &MyType::from("Edgar Ætheling"));
//! search_index.insert(&2, &MyType::from("William the Conqueror"));
//! search_index.insert(&3, &MyType::from("William Rufus"));
//! search_index.insert(&4, &MyType::from("Henry Beauclerc"));
//!
//! let resulting_keys: Vec<&usize> = search_index.search("William");
//!
//! assert_eq!(resulting_keys, vec![&2, &3]);
//! ```
//!
//! Search only supports exact keyword matches and does not use fuzzy matching.
//! Consider providing the `autocomplete` feature to your users as an ergonomic
//! alternative to fuzzy matching.
//!
//! ## 5. Autocompletion
//!
//! The `autocomplete` method will provide several autocompletion options for
//! the last keyword in the supplied string.
//!
//! Basic usage:
//!
//! ```rust
//! # use indicium::simple::{AutocompleteType, Indexable, SearchIndex, SearchIndexBuilder};
//! #
//! # struct MyType { text: String }
//! #
//! # impl From<&str> for MyType {
//! #   fn from(string: &str) -> Self {
//! #       MyType { text: string.to_string() }
//! #   }
//! # }
//! #
//! # impl Indexable for MyType {
//! #   fn strings(&self) -> Vec<String> {
//! #       vec![self.text.clone()]
//! #   }
//! # }
//! #
//! let mut search_index: SearchIndex<usize> =
//!     SearchIndexBuilder::default()
//!         .autocomplete_type(&AutocompleteType::Global)
//!         .exclude_keywords(&None)
//!         .build();
//!
//! search_index.insert(&0, &MyType::from("apple"));
//! search_index.insert(&1, &MyType::from("ball"));
//! search_index.insert(&3, &MyType::from("bird"));
//! search_index.insert(&4, &MyType::from("birthday"));
//! search_index.insert(&5, &MyType::from("red"));
//!
//! let autocomplete_options: Vec<String> =
//!     search_index.autocomplete("a very big bi");
//!
//! assert_eq!(
//!     autocomplete_options,
//!     vec!["a very big bird", "a very big birthday"]
//! );
//! ```

#![doc(html_favicon_url = "https://www.arkiteq.ca/crates/indicium/icon.png")]
#![doc(html_logo_url = "https://www.arkiteq.ca/crates/indicium/logo.png")]

#[cfg(feature = "simple")]
pub mod simple;

#[cfg(feature = "select2")]
pub mod select2;