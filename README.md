# Indicium

A simple search engine for collections and key-value stores with
typeahead/autocomplete.

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

To begin we must make our struct indexable. We do this by implementing the
`Indexable` trait for our `struct`:

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

The idea is to return a `String` for every field that we would like to be
indexed. Once this trait is implemented, the struct can be indexed by
`indicium`.

Don't forget: you may make numbers, numeric identifiers, enums, and other types
indexable by converting them to a `String` and including them in the returned
`Vec<String>`!

## Indexing a Collection

To index an existing collection, we can iterate over the collection. For each
record, insert it into the index. This might look like something like these two
examples:

```rust
let mut search_index: SearchIndex<usize> = SearchIndex::default();

my_vec
    .iter()
    .enumerate()
    .for_each(|(index, element)| search_index.insert(&index, element));
```

```rust
let mut search_index: SearchIndex<usize> = SearchIndex::default();

my_hashmap
    .iter()
    .for_each(|(key, value)| search_index.insert(&key, value));
```

The above examples will work for a previously populated `Vec` or `HashMap`.
However, the preferred method is to index your collection (Vec, HashMap, etc.)
as it is being populated.

## Search

Once the index has been populated, you can use the `autocomplete` and `search`
functions.

The `autocomplete` function will autocomplete / typeahead the last keyword in
the string. This function will return several strings with different options
for the autocompleting the last keyword. The results are returned in lexographic
order:

```rust
let keywords: Vec<String> = search_index.autocomplete(&"ass".to_string());

assert_eq!(keywords, vec!["assassin", "assistance"]);
```

The `search` function will return all keys for indexed structs that exactly match the single `String` keyword provided by the caller:

```rust
let indicies: Vec<u32> = search_index.search_keyword(&"Helicopter".to_string());

assert_eq!(indicies, Some(vec![&1]));
```

Note: the `autocomplete_keyword` and `search_keyword` functions work on strings
that are expected to contain only a single keyword. For small collections, this
might be a workable & lighter-weight solution than using their big brothers.

The `autocomplete_keyword` function will return all indexed keywords that begin
with the single `String` provided by the caller:

```rust
let keywords: Vec<String> = search_index.autocomplete_keyword(&"ass".to_string());

assert_eq!(keywords, vec!["assassin", "assistance"]);
```

The `search_keyword` function will return all keys for indexed structs that
exactly match the single `String` keyword provided by the caller:

```rust
let indicies: Vec<u32> = search_index.search_keyword(&"Helicopter".to_string());

assert_eq!(indicies, Some(vec![&1]));
```