# indicium
![Crates.io Version](https://img.shields.io/crates/v/indicium)
![Crates.io MSRV](https://img.shields.io/crates/msrv/indicium)
![Crates.io License](https://img.shields.io/crates/l/indicium)
![Crates.io Total Downloads](https://img.shields.io/crates/d/indicium)

A simple in-memory search for collections (`Vec`, `HashMap`, `BTreeMap`, etc.)
and key-value stores. Features autocompletion and fuzzy matching.

There are many incredible search engines available for Rust. Many seem to
require compiling a separate server binary. I wanted something simple and
light-weight - an easy-to-use crate that could conveniently search structs and
collections within my own binary. So, I made `indicium`.

<img src="https://www.arkiteq.io/crates/indicium/banner.jpg" alt="Indicium: A Simple In-Memory Search for Rust" width="400"/>

While `indicium` was made with web apps in mind, it is an in-memory search and
it does not scale indefinitely or to cloud size (i.e. Facebook or Google size).
Even in such an environment, it would still be a convenient way of searching
large lists (such as currencies, languages, countries, etc.) It's also great for
applications where there is an anticipated scale limit (i.e. searching a list of
company assets, list of users in a corporate intranet, etc.)

Indicium easily can handle millions of records without breaking a sweat thanks
to Rust's [BTreeMap](https://cglab.ca/~abeinges/blah/rust-btree-case/). This
crate is primarily limited by available memory. However, depending on the nature
your data-set and if there are keywords that are repeated many times,
performance may begin to degrade at a point.

# Installation

Configure the dependencies in your project's `Cargo.toml` file:

```toml
[dependencies]
indicium = "0.6"
```

# Release Notes

* Release notes are available on
  [GitHub](https://github.com/leontoeides/indicium/releases).

* The full [change
  log](https://github.com/leontoeides/indicium/blob/master/CHANGELOG.md) is
  available on GitHub.

# Quick Start Guide

For our **Quick Start Guide** example, we will be searching inside of the
following `struct`:

```rust
struct MyStruct {
    title: String,
    year: u16,
    body: String,
}
```

## 1. Implementing Indexable

To begin, we must make our record indexable. We'll do this by implementing the
`Indexable` trait for our `struct`. The idea is to return a `String` for every
field that we would like to be indexed. Example:

```rust
use indicium::simple::Indexable;

impl Indexable for MyStruct {
    fn strings(&self) -> Vec<String> {
        vec![
            self.title.clone(),
            self.year.to_string(),
            self.body.clone(),
        ]
    }
}
```

Don't forget that you may make numbers, numeric identifiers, enums, and other
types in your `struct` (or other complex types) indexable by converting them to
a `String` and including them in the returned `Vec<String>`.

## 2. Indexing a Collection

To index an existing collection, we can iterate over the collection. For each
record, we will insert it into the search index. This should look something
like these two examples:

#### Vec

```rust
use indicium::simple::SearchIndex;

let my_vec: Vec<MyStruct> = Vec::new();

// In the case of a `Vec` collection, we use the index as our key. A
// `Vec` index is a `usize` type. Therefore we will instantiate
// `SearchIndex` as `SearchIndex<usize>`.

let mut search_index: SearchIndex<usize> = SearchIndex::default();

my_vec
    .iter()
    .enumerate()
    .for_each(|(index, element)|
        search_index.insert(&index, element)
    );
```

#### HashMap

```rust
use std::collections::HashMap;
use indicium::simple::SearchIndex;

let my_hash_map: HashMap<String, MyStruct> = HashMap::new();

// In the case of a `HashMap` collection, we use the hash map's key as
// the `SearchIndex` key. In our hypothetical example, we will use
// MyStruct's `title` as a the key which is a `String` type. Therefore
// we will instantiate `HashMap<K, V>` as HashMap<String, MyStruct> and
// `SearchIndex<K>` as `SearchIndex<String>`.

let mut search_index: SearchIndex<String> = SearchIndex::default();

my_hash_map
    .iter()
    .for_each(|(key, value)|
        search_index.insert(key, value)
    );
```

As long as the `Indexable` trait was implemented for your value type, the above
examples will index a previously populated `Vec` or `HashMap`. However, the
preferred method for large collections is to `insert` into the `SearchIndex` as
you insert into your collection (Vec, HashMap, etc.)

It's recommended to wrap your target collection (your `Vec`, `HashMap`, etc.)
and this `SearchIndex` together in a new `struct` type. Then, implement the
`insert`, `replace`, `remove`, etc. methods for this new `struct` type that will
update both the collection and search index. This will ensure that both your
collection and index are always synchronized.

Once the index has been populated, you can use the `search` and `autocomplete`
methods.

## 3. Searching

The `search` method will return keys as the search results. Each resulting
key can then be used to retrieve the full record from its collection.

Basic usage:

```rust
let mut search_index: SearchIndex<usize> = SearchIndex::default();

search_index.insert(&0, &"Harold Godwinson");
search_index.insert(&1, &"Edgar Ætheling");
search_index.insert(&2, &"William the Conqueror");
search_index.insert(&3, &"William Rufus");
search_index.insert(&4, &"Henry Beauclerc");

let resulting_keys: Vec<&usize> = search_index.search("William");

assert_eq!(resulting_keys, vec![&2, &3]);

// Demonstrating fuzzy matching:

let resulting_keys: Vec<&usize> = search_index.search("Harry");

assert_eq!(resulting_keys, vec![&0]);
```

Search only supports exact keyword matches. For `Live` searches, fuzzy matching
is only applied to the last keyword. Consider providing the `autocomplete`
feature to your users to help them build their search as they type.

## 4. Autocompletion

The `autocomplete` method will provide several autocompletion options for the
last keyword in the supplied string.

Basic usage:

```rust
let mut search_index: SearchIndex<usize> =
    SearchIndexBuilder::default()
        .autocomplete_type(&AutocompleteType::Global)
        .build();

search_index.insert(&0, &"apple");
search_index.insert(&1, &"ball");
search_index.insert(&3, &"bird");
search_index.insert(&4, &"birthday");
search_index.insert(&5, &"red");

let autocomplete_options: Vec<String> =
    search_index.autocomplete("a very big bi");

assert_eq!(
    autocomplete_options,
    vec!["a very big bird", "a very big birthday"]
);

// Demonstrating fuzzy matching:

let autocomplete_options: Vec<String> =
    search_index.autocomplete("a very big birf");

assert_eq!(
    autocomplete_options,
    vec!["a very big bird", "a very big birthday"]
);
```

# Bonus Points

## 5. Custom Result Ordering with `Ord`

`indicium` returns a list of matching keys (`K`) in the order defined by their `Ord` implementation.
This means you can use your own types for `K` that embed ranking signals like `popularity`,
`edge_weight`, or even a custom timestamp-based score.

By customizing `Ord`, you can make search results automatically favor more relevant or recent
entries without doing any post-sorting.

Here's how:

```rust
#[derive(Debug, Clone)]
pub struct MyKey {
    pub id: String,
    pub popularity: u32,
}

// Used for ordering search results

impl Ord for MyKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Descending order: more popular keys come first
        other.popularity.cmp(&self.popularity)
    }
}

impl PartialOrd for MyKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Used for equality & lookup

impl PartialEq for MyKey {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for MyKey {}

use std::hash::{Hash, Hasher};

impl Hash for MyKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
```

If you're using indicium to:

* Rank search hits by popularity
* Rank suggestions by recency
* Sort links by edge weight

...then this pattern will give you accurate result ordering for free at the key level.

Tip: Lookup Map Compatibility

Since you likely want to fetch a full record from a map or DB after search:

```rust
let hits = indicium.search("silver thread of dawn")?;

for key in hits {
    if let Some(record) = records.get(&key) {
        println!("Result: {:?}", record);
    }
}
```

Make sure your `Eq` and `Hash` impls exclude transient fields like `popularity` so you can use the
key for lookups.

# Crate Status

This crate is passively maintained. This crate does what it's expected to do and
does it pretty well, in my opinion. Frequent updates are not expected.