#[test]
fn simple() {

    use crate::simple::{Indexable, SearchIndex};

    struct TestStruct {
        title: String,
        description: String,
    } // TestStruct

    impl Indexable for TestStruct {
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
            description: "Another word for help.".to_string(),
        }, // TestStruct
    ]; // vec!

    test_vec
        .iter()
        .enumerate()
        .for_each(|(index, element)| search_index.insert(&index, element));

    // Test `autocomplete_keyword` method:

    println!("Autocomplete keyword: {:#?}", search_index.autocomplete_keyword(&"ass".to_string()));
    assert_eq!(search_index.autocomplete_keyword(&"ass".to_string()), vec!["assistance"]);

    // Test `autocomplete_keyword` method:

    println!("Autocomplete keyword: {:#?}", search_index.autocomplete_keyword(&"hel".to_string()));
    assert_eq!(search_index.autocomplete_keyword(&"hel".to_string()), vec!["helicopter", "hell", "hello", "help"]);

    // Test `autocomplete` method:

    println!("Autocomplete string: {:#?}", search_index.autocomplete(&"hel hel hel".to_string()));
    assert_eq!(
        search_index.autocomplete(&"hel hel hel".to_string()),
        vec![
            "hel hel helicopter",
            "hel hel hell",
            "hel hel hello",
            "hel hel help",
        ]
    );

    // Test `search_keyword` method:

    println!("Search keyword: {:#?}", search_index.search_keyword(&"AsSisTanCe".to_string()));
    assert_eq!(search_index.search_keyword(&"AsSisTanCe".to_string()), vec![&3]);

    // Test `search_keyword` method:

    println!("Search string: {:#?}", search_index.search(&"Helicopter around".to_string()));
    assert_eq!(search_index.search(&"Helicopter around".to_string()), vec![1]);

    // Test `search_keyword` method:

    println!("Search string: {:#?}", search_index.search(&"Helicopter around help".to_string()));
    assert_eq!(search_index.search(&"Helicopter around help".to_string()), vec![1, 3]);

    // Test `remove` method:

    search_index.remove(&1, &TestStruct {
        title: "Helicopter".to_string(),
        description: "How I would like to get around.".to_string(),
    });

    println!("Search: {:#?}", search_index.search_keyword(&"Helicopter".to_string()));
    assert_eq!(search_index.search_keyword(&"Helicopter".to_string()), Vec::<&usize>::new());

    // Test `insert` method:

    search_index.insert(&1, &TestStruct {
        title: "Helicopter".to_string(),
        description: "How I would like to get around.".to_string(),
    });

    println!("Search: {:#?}", search_index.search_keyword(&"Helicopter".to_string()));
    assert_eq!(search_index.search_keyword(&"Helicopter".to_string()), vec![&1]);

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
    assert_eq!(search_index.search_keyword(&"Quadricopter".to_string()), vec![&1]);

} // fn