# indicium

A simple search engine for collections and key-value stores.

# Getting Started:

To begin you must make your structs indexable by implementing the
`IndexableStruct` trait.

For our **Getting Started** example, we will be using the following struct:

```rust
struct MyStruct {
    id: u32,
    title: String,
    description: String,
}
```

The following code demonstrates how to implement the `IndexableStruct` trait for
this struct:

```rust
use indicium::simple::IndexableStruct;

impl IndexableStruct for MyStruct {
    fn strings(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.title.clone(),
            self.description.clone(),
        ] // vec!
    } // fn
} // impl
```

The idea is to return a string for every field that you would like to be
indexed. Once this trait is implemented it can now be indexed by `indicium`.

To index your collection: iterate over the collection and for each record,
insert it into the index. It might look like something like this:

```rust
my_vec.iter().for_each(|record| search_index.insert(&record.id, record));
```

Once the index has been populated, you can use the `autocomplete` and `search`
functions.

The `autocomplete` function will return all indexed keywords that begin with the
`String` provided by the caller:

```rust
let keywords: Vec<String> = search_index.autocomplete(&"ass".to_string());

```

The `autocomplete` function will return all keys for indexed structs that
exactly match the `String` provided by the caller:

```rust
let keys: Vec<u32> = search_index.search(&"Helicopter".to_string());

```
