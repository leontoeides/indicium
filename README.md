# Indicium Search

🔎 A simple in-memory search for collections (Vec, HashMap, BTreeMap, etc) and
key-value stores. Features autocompletion.

There are many incredible search engines available for Rust. Many seem to
require compiling a separate server binary. I wanted something simple and
light-weight - an easy-to-use crate that could conveniently search structs and
collections within my own binary. So, I made `indicium`.

![alt text](https://www.arkiteq.ca/crates/indicium/banner.jpg "Indicium: A Simple In-Memory Search for Rust")

While `indicium` was made with web apps in mind, it is an in-memory search and
it does not scale indefinitely or to cloud size (i.e. Facebook or Google size).
Even in such an environment, it would still be a convenient way of searching
large lists (such as currencies, languages, countries, etc.) It's also great for
applications where there is an anticipated scale limit (i.e. searching a list of
company assets, list of users in a corporate intranet, etc.)

Indicium easily can handle millions of records without breaking a sweat thanks
to Rust's [BTreeMap](https://cglab.ca/~abeinges/blah/rust-btree-case/). This
crate is primarily limited by available memory. However, depending on the nature
your data-set and if there keywords that are repeated many times, performance
may begin to degrade at a point.

# What's New?

* `0.5.1`: Fixes for experimental `select2` feature.

* `0.5.0`: The `simple` search index now internally employs the
[KString](https://crates.io/crates/kstring) crate. This should help with
performance in certain cases. The `dump_keyword` and `profile` function
signatures were changed slightly, otherwise there's no impact to the caller.

* `0.5.0`: Performance improvements.

* `0.4.2`: Any type that implements
[ToString](https://doc.rust-lang.org/std/string/trait.ToString.html) (and
consequently any type that implements
[Display](https://doc.rust-lang.org/std/fmt/trait.Display.html))
now gets the
[Indexable](https://docs.rs/indicium/latest/indicium/simple/trait.Indexable.html)
implementation for free.

* `0.4.1`: Improved contextual fuzzy matching.

* `0.4.0`: Initial support for fuzzy searching. Fuzzy matching is applied to the
last (partial) keyword in the search string for _autocompletion_ and _live
search_ only. Keywords at the start or in the middle of the user's search string
will not be substituted.
	* Some changes for an upcoming `0.5.0` release are being considered.
	This release could have some changes that would allow `indicium` to
	provide feedback to the user, including which keywords have been
	substituted.

* `0.4.0`: Breaking changes:
	* Builder pattern is now passed owned values.
	* `K` key type requires `Hash` trait for `fuzzy` string search feature.
	* New `SearchIndex` default settings.

* `0.4.0`: **Any dependent software should see if (or how) the updated defaults
change search behaviour and tweak accordingly before adopting the 0.4.0
update.**

* `0.3.7`: An experimental feature is now disabled by default to reduce resource
consumption.

* `0.3.6`: Implemented `DerefMut` which gives access to the search index's
underlying `BTreeMap`. Implemented `clear()` which is a convenience method for
clearing the search index.

* `0.3.5`: Peformance improvements.

* `0.3.4`: Peformance improvements.

* `0.3.3`: Fix: `cargo test` failed. Sorry.

* `0.3.2`: Fix: issue with search indexes that do not use keyword splitting.

* `0.3.1`: Autocomplete no longer offers previously used keywords as options.

* `0.3.1`: Added `maximum_keys_per_keyword` getter method.

* `0.3.1`: Added `autocomplete_with` and `search_with` methods which allow
ad-hoc overrides of the `AutocompleteType`/`SearchType` and maximum results
parameters.

* `0.3.0`: Added new search type `SearchType::Live` which is for "search as you
type" interfaces. It is sort of a hybrid between `autocomplete` and
`SearchType::And`. It will search using an (incomplete) string and return keys
as the search results. Each resulting key can then be used to retrieve the full
record from its collection to be rendered & displayed to the user.

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
types in your `struct` (or other complex type) indexable by converting them to a
`String` and including them in the returned `Vec<String>`.

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
feature to your users as an ergonomic alternative to fuzzy matching.

## 5. Autocompletion

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