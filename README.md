# Indicium

A simple search engine for collections (Vec, HashMap, BTreeMap, etc) and
key-value stores. Features autocompletion.

There are many incredible search engines available for Rust. Many seem to
require compiling a separate server binary. I wanted something simple, light
weight, and that could conveniently search structs and collections. So I have
made `indicium`.

# Quick Start Guide

For our **Quick Start Guide** example, we will be searching inside of the
following `struct`:

```rust
struct MyStruct {
    title: String,
    description: String,
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
            self.description.clone(),
        ]
    }
}
```

Don't forget that you may make numbers, numeric identifiers, enums, and other
types indexable by converting them to a `String` and including them in the
returned `Vec<String>`.

## 2. Indexing a Collection

To index an existing collection, we can iterate over the collection. For each
record, we will insert it into the search index. This should look something
like these two examples:

#### Vec

```rust
use indicium::simple::SearchIndex;

let my_vec: Vec<MyStruct> = Vec::new();

// In the case of a `Vec` collection, we use the index as our key.  A
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

Once the index has been populated, you can use the `autocomplete` and `search`
functions.

## 3. Searching

The `search` function will return keys as the search results. Each resulting
key can then be used to retrieve the full record from its collection. Search
keywords must be an exact match.

The logical conjuction for multiple keywords is `or`. For example, a search of
`this that` will return records containing keywords `this` or `that`.

The results are returned in order of descending relevance. Records containing
both keywords `this` and `that` will be the top results.

Example usage:

```rust
let resulting_keys: Vec<usize> =
    search_index.keyword_search(&"helicopter".to_string());

assert_eq!(resulting_keys, Some(vec![1]));
```

Search only supports exact keyword matches and does not use fuzzy matching.
Consider providing the `autocomplete` feature to your users as an ergonomic
alternative to fuzzy matching.

## 4. Autocompletion

The `autocomplete` function will provide several autocompletion options for the
last partial keyword in the supplied string. The results are returned in
lexographic order. Example usage:

```rust
let autocomplete_options: Vec<String> =
    search_index.autocomplete(&"a very big bir".to_string());

assert_eq!(
	autocomplete_options,
	vec!["very big bird", "very big birthday"]
);
```

# The Keyword Methods

The `keyword_autocomplete` and `keyword_search` methods work on strings that are
expected to contain only a single keyword (as opposed to strings containing
multiple keywords.) For small collections, these methods might be a
lighter-weight alternative to their big brothers.

## Searching

The `keyword_search` function will return keys for records that match the
keyword provided by the caller. Each resulting key can then be used to retrieve
the full record from its collection. The search keyword must be an exact match.
The results are returned in pseudo-random order. Example usage:

```rust
let resulting_keys: Vec<usize> =
	search_index.keyword_search(&"helicopter".to_string());

assert_eq!(resulting_keys, Some(vec![&1]));
```

Search only supports exact keyword matches and does not use fuzzy matching.
Consider providing the `autocomplete` feature to your users as an ergonomic
alternative to fuzzy matching.

Note that, currently, the `maximum_search_results` setting is only observed by
the `search` method and not by this `keyword_search` method. This method will
potentially return thousands of key results, depending on the size of your
collection. This is because `search` relies on this function, and it needs a
full list of keys to properly rank the results by relevance.

## Autocompletion

The `keyword_autocomplete` function will return several keywords that begin with
the partial keyword provided by the caller. The results are returned in
lexographic order. Example usage:

```rust
let autocomplete_options: Vec<String> =
	search_index.keyword_autocomplete(&"ass".to_string());

assert_eq!(autocomplete_options, vec!["assassin", "assistance"]);
```
