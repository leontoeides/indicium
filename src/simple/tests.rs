#[test]
fn simple() {

    use crate::simple::{IndexableStruct, SearchIndex};

    struct TestStruct {
        title: String,
        description: String,
    } // TestStruct

    impl IndexableStruct for TestStruct {
        fn strings(&self) -> Vec<String> {
            vec![
                self.title.clone(),
                self.description.clone(),
            ] // vec!
        } // fn
    } // impl

    let mut search_index: SearchIndex<usize> = SearchIndex::default();

    let test_vec = vec![
        TestStruct {
            title: "Hello".to_string(),
            description: "What I say to people.".to_string(),
        }, // TestStruct
        TestStruct {
            title: "Helicopter".to_string(),
            description: "How I would like to get around.".to_string(),
        }, // TestStruct
        TestStruct {
            title: "Hell".to_string(),
            description: "A place I don't wanna be.".to_string(),
        }, // TestStruct
        TestStruct {
            title: "Assistance".to_string(),
            description: "Is another word for help.".to_string(),
        }, // TestStruct
    ]; // vec!

    test_vec
        .iter()
        .enumerate()
        .for_each(|(index, element)| search_index.insert(&index, element));

    // Test `autocomplete` method:

    println!("Autocomplete: {:#?}", search_index.autocomplete(&"ass".to_string()));
    assert_eq!(search_index.autocomplete(&"ass".to_string()), vec!["assistance"]);

    // Test `search` method:

    println!("Search: {:#?}", search_index.autocomplete(&"hel".to_string()));
    assert_eq!(search_index.autocomplete(&"hel".to_string()), vec!["helicopter", "hell", "hello", "help"]);

    // Test `search` method:

    println!("Search: {:#?}", search_index.search_keyword(&"AsSisTanCe".to_string()));
    assert_eq!(search_index.search_keyword(&"AsSisTanCe".to_string()), Some(vec![&3]));

    // Test `remove` method:

    search_index.remove(&1, &TestStruct {
        title: "Helicopter".to_string(),
        description: "How I would like to get around.".to_string(),
    });

    println!("Search: {:#?}", search_index.search_keyword(&"Helicopter".to_string()));
    assert_eq!(search_index.search_keyword(&"Helicopter".to_string()), Some(vec![]));

    // Test `insert` method:

    search_index.insert(&1, &TestStruct {
        title: "Helicopter".to_string(),
        description: "How I would like to get around.".to_string(),
    });

    println!("Search: {:#?}", search_index.search_keyword(&"Helicopter".to_string()));
    assert_eq!(search_index.search_keyword(&"Helicopter".to_string()), Some(vec![&1]));

    // Test `replace` method:

    search_index.replace(
        &1,
        &TestStruct {
            title: "Helicopter".to_string(),
            description: "How I would like to get around.".to_string(),
        },
        &TestStruct {
            title: "Quadricopter".to_string(),
            description: "How I would like to get around.".to_string(),
        },
    );

    println!("Search: {:#?}", search_index.search_keyword(&"Quadricopter".to_string()));
    assert_eq!(search_index.search_keyword(&"Quadricopter".to_string()), Some(vec![&1]));

} // fn