# Indicium

A simple search engine for collections and key-value stores. Includes
capability for autocomplete / typeahead.

# Quick Start Guide

For our **Quick Start Guide** example, we will be searching inside of the
following `struct`:

```rust
struct MyStruct {
    title: String,
    description: String,
}
```
## Implementing Indexable

To begin, we must make our struct indexable. We do this by implementing the
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

## Indexing a Collection

To index an existing collection, we can iterate over the collection. For each
record, insert it into the search index. This might look like something like
these two examples:

```rust
use indicium::simple::SearchIndex;

let mut search_index: SearchIndex<usize> =
    SearchIndex::default();

my_vec
    .iter()
    .enumerate()
    .for_each(|(index, element)|
        search_index.insert(&index, element)
    );
```

```rust
use indicium::simple::SearchIndex;

let mut search_index: SearchIndex<usize> =
    SearchIndex::default();

my_hashmap
    .iter()
    .for_each(|(key, value)|
        search_index.insert(&key, value)
    );
```

The above examples will index a previously populated `Vec` or `HashMap`.
However, the preferred method is to `insert` into the `SearchIndex` as you
insert into your collection (Vec, HashMap, etc.).

Once the index has been populated, you can use the `autocomplete` and `search`
functions.

## Autocompletion

The `autocomplete` function will provide several autocompletion options for the
last keyword in the supplied string. The results are returned in lexographic
order. Example usage:

```rust
let keywords: Vec<String> =
    search_index.autocomplete(&"huge ass".to_string());

assert_eq!(keywords, vec!["huge assassin", "huge assistance"]);
```

With a bit of imagination you could create a typeahead microservice for your web
application using a crate like `actix-web` or `rocket`.

## Searching

The `search` function will return the keys for search results. Each resulting
key can then be used to retrieve the corresponding record from its collection.
Search keywords must be an exact match. The results are returned in order of
descending relevance. Example usage:

```rust
let keys: Vec<u32> =
    search_index.search_keyword(&"Helicopter".to_string());

assert_eq!(keys, Some(vec![1]));
```

## The Keyword Methods

The `autocomplete_keyword` and `search_keyword` methods work on strings that are
expected to contain only a single keyword - as opposed to strings with multiple
keywords. For small collections this might be a lighter-weight alternative to
their big brothers.

The `autocomplete_keyword` function will return all indexed keywords that begin
with the single `String` provided by the caller. Example usage:

```rust
let keywords: Vec<String> =
	search_index.autocomplete_keyword(&"ass".to_string());

assert_eq!(keywords, vec!["assassin", "assistance"]);
```

The `search_keyword` function will return all keys for indexed structs that
exactly match the single `String` keyword provided by the caller. Search
keywords must be an exact match. Each resulting key can then be used to retrieve
the corresponding record from its collection. The results are returned in
undefined order. Example usage:

```rust
let keys: Vec<u32> =
	search_index.search_keyword(&"Helicopter".to_string());

assert_eq!(keys, Some(vec![&1]));
```
