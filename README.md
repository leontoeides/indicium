# indicium

A simple search engine for collections and key-value stores.

# Getting Started:

For our **Getting Started** example, we will be searching inside of the
following `struct`:

```rust
struct MyStruct {
    id: u32,
    title: String,
    description: String,
}
```

To begin we must make our struct indexable. We do this by implementing the
`IndexableStruct` trait for our `struct`:

```rust
use indicium::simple::IndexableStruct;

impl IndexableStruct for MyStruct {
    fn strings(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.title.clone(),
            self.description.clone(),
        ]
    }
}
```

The idea is to return a string for every field that we would like to be indexed.
Once this trait is implemented it can now be indexed by `indicium`.

To index our collection we can iterate over the collection. For each record,
insert it into the index. It might look like something like this:

```rust
let mut search_index: SearchIndex<u32> = SearchIndex::default();

my_vec.iter().for_each(|record| search_index.insert(&record.id, record));
```

While this is one line of code looks very simple, the preferred method would be
to index your collection (Vec, HashMap, etc.) as it is being populated.

Once the index has been populated, you can use the `autocomplete` and `search`
functions.

The `autocomplete` function will return all indexed keywords that begin with the
`String` provided by the caller:

```rust
let keywords: Vec<String> = search_index.autocomplete(&"ass".to_string());

```

The `search` function will return all keys for indexed structs that exactly
match the `String` keyword provided by the caller:

```rust
let keys: Vec<u32> = search_index.search(&"Helicopter".to_string());

```
