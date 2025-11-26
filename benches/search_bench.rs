// benches/search_bench.rs with Criterion
use criterion::{criterion_group, criterion_main, Criterion};
use indicium::simple::{Indexable, SearchIndex};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct MyKey(String);
struct MyValue(Vec<String>);

impl Indexable for MyValue {
    fn strings(&self) -> Vec<String> {
        self.0.clone()
    }
}

fn indicium_benchmark(c: &mut Criterion) {
    let mut search_index: SearchIndex<MyKey> = SearchIndex::default();

    for i in 0..10_000 {
        let key = MyKey(format!("record_{:05}", i));
        let tokens = MyValue(vec![
            format!("thread_{}", i),
            "silver".to_string(),
            "dawn".to_string(),
        ]);
        search_index.insert(&key, &tokens);
    }

    c.bench_function("search 'silver thread of dawn'", |b| {
        b.iter(|| {
            let _ = search_index.search("silver thread of dawn");
        });
    });
}

criterion_group!(benches, indicium_benchmark);
criterion_main!(benches);