#[test]
fn simple() {

    use crate::simple::{IndexableStruct, IndexComponents, SearchIndex};

    #[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct TestStruct {
        id: u32,
        title: String,
        description: String,
    }

    impl IndexableStruct<u32> for TestStruct {
        fn components(&self) -> IndexComponents<u32> {
            IndexComponents {
                key: self.id,
                strings: vec![
                    self.id.to_string(),
                    self.title.clone(),
                    self.description.clone(),
                ],
            }
        }
    }

    let mut search_index: SearchIndex<u32> = SearchIndex::default();

    let test_vec = vec![
        TestStruct {
            id: 0,
            title: "Hello".to_string(),
            description: "What I say to people.".to_string(),
        },
        TestStruct {
            id: 1,
            title: "Helicopter".to_string(),
            description: "How I would like to get around.".to_string(),
        },
        TestStruct {
            id: 2,
            title: "Hell".to_string(),
            description: "A place I don't wanna be.".to_string(),
        },
        TestStruct {
            id: 3,
            title: "Assistance".to_string(),
            description: "Is another word for help.".to_string(),
        },
    ];

    test_vec.iter().for_each(|record| search_index.insert(record));

    // Test `autocomplete` method:

    println!("Autocomplete: {:#?}", search_index.autocomplete(&"ass".to_string()));
    assert_eq!(search_index.autocomplete(&"ass".to_string()), vec!["assistance"]);

    // Test `search` method:

    println!("Search: {:#?}", search_index.search(&"AsSisTanCe".to_string()));
    assert_eq!(search_index.search(&"AsSisTanCe".to_string()), Some(vec![&3]));

    // Test `remove` method:

    search_index.remove(&TestStruct {
        id: 1,
        title: "Helicopter".to_string(),
        description: "How I would like to get around.".to_string(),
    });

    println!("Search: {:#?}", search_index.search(&"Helicopter".to_string()));
    assert_eq!(search_index.search(&"Helicopter".to_string()), Some(vec![]));

    // Test `insert` method:

    search_index.insert(&TestStruct {
        id: 1,
        title: "Helicopter".to_string(),
        description: "How I would like to get around.".to_string(),
    });

    println!("Search: {:#?}", search_index.search(&"Helicopter".to_string()));
    assert_eq!(search_index.search(&"Helicopter".to_string()), Some(vec![&1]));

    // Test `replace` method:

    search_index.replace(
        &TestStruct {
            id: 1,
            title: "Helicopter".to_string(),
            description: "How I would like to get around.".to_string(),
        },
        &TestStruct {
            id: 4,
            title: "Quadricopter".to_string(),
            description: "How I would like to get around.".to_string(),
        },
    );

    println!("Search: {:#?}", search_index.search(&"Quadricopter".to_string()));
    assert_eq!(search_index.search(&"Quadricopter".to_string()), Some(vec![&4]));

} // fn