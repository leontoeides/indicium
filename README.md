# Indicium

A simple search engine for collections and key-value stores with
autocomplete/typeahead.

# Quick Guide

For our **Quick Guide** example, we will be searching inside of the
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

Don't forget - you may make numbers, numeric identifiers, enums, and other types
indexable by converting them to a `String` and including them in the returned
`Vec<String>`.

## Indexing a Collection

To index an existing collection, we can iterate over the collection. For each
record, insert it into the search index. This might look like something like
these two examples:

```rust
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
let mut search_index: SearchIndex<usize> =
    SearchIndex::default();

my_hashmap
    .iter()
    .for_each(|(key, value)|
        search_index.insert(&key, value)
    );
```

The above examples will work for a previously populated `Vec` or `HashMap`.
However, the preferred method is to index your collection (Vec, HashMap, etc.)
as it is being populated.

## Searching

Once the index has been populated, you can use the `autocomplete` and `search`
functions.

The `autocomplete` function will provide several autocompletion options for the
last keyword in the supplied string. The results are returned in lexographic
order. Example usage:

```rust
let keywords: Vec<String> =
    search_index.autocomplete(&"huge ass".to_string());

assert_eq!(keywords, vec!["huge assassin", "huge assistance"]);
```

The `search` function will return the keys for found records. The results are
returned in order of descending relevance. Each resulting key can then be used
to retrieve the corresponding record from its collection. Example usage:

```rust
let indicies: Vec<u32> =
    search_index.search_keyword(&"Helicopter".to_string());

assert_eq!(indicies, Some(vec![&1]));
```

## The Keyword Methods

The `autocomplete_keyword` and `search_keyword` methods work on strings that are
expected to contain only a single keyword. For small collections this might be a
workable & lighter-weight alternative to using their big brothers.

The `autocomplete_keyword` function will return all indexed keywords that begin
with the single `String` provided by the caller. Example usage:

```rust
let keywords: Vec<String> = search_index.autocomplete_keyword(&"ass".to_string());

assert_eq!(keywords, vec!["assassin", "assistance"]);
```

The `search_keyword` function will return all keys for indexed structs that
exactly match the single `String` keyword provided by the caller. Each resulting
key can then be used to retrieve the corresponding record from its collection.
Example usage:

```rust
let indicies: Vec<u32> = search_index.search_keyword(&"Helicopter".to_string());

assert_eq!(indicies, Some(vec![&1]));
```
