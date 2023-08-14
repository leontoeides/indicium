

#[test]
fn simple() {

    use crate::simple::{AutocompleteType, Indexable, SearchIndex, SearchType};
    use crate::simple::internal::string_keywords::SplitContext;

    #[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct MyStruct {
        title: String,
        year: u16,
        body: String,
    }

    impl Indexable for MyStruct {
        fn strings(&self) -> Vec<String> {
            vec![
                self.title.clone(),
                self.year.to_string(),
                self.body.clone(),
            ]
        }
    }

    let my_vec = vec![
        MyStruct {
            title: "Harold Godwinson".to_string(),
            year: 1066,
            body: "Last crowned Anglo-Saxon king of England.".to_string(),
        },
        MyStruct {
            title: "Edgar Ætheling".to_string(),
            year: 1066,
            body: "Last male member of the royal house of Cerdic of Wessex.".to_string(),
        },
        MyStruct {
            title: "William the Conqueror".to_string(),
            year: 1066,
            body: "First Norman monarch of England.".to_string(),
        },
        MyStruct {
            title: "William Rufus".to_string(),
            year: 1087,
            body: "Third son of William the Conqueror.".to_string(),
        },
        MyStruct {
            title: "Henry Beauclerc".to_string(),
            year: 1100,
            body: "Fourth son of William the Conqueror.".to_string(),
        },
    ];

    let mut search_index: SearchIndex<usize> = SearchIndex::default();

    let string_keywords: Vec<String> = search_index.string_keywords(
        "All is not lost, the unconquerable will, and study of revenge, \
        immortal hate, and the courage never to submit or yield.",
        SplitContext::Indexing,
    );

    assert_eq!(string_keywords,
        [ "all", "is", "not", "lost", "unconquerable", "will", "study",
        "revenge", "immortal", "hate", "courage", "never", "submit", "yield" ]
    );

    let string_keywords: Vec<String> = search_index.string_keywords(
        "He prayeth best, who loveth best All things both great and small; For \
        the dear God who loveth us, He made and loveth all.",
        SplitContext::Searching,
    );

    assert_eq!(string_keywords,
        [ "he", "prayeth", "best", "who", "loveth", "best", "all", "things",
        "both", "great", "small", "dear", "god", "who", "loveth", "us", "he",
        "made", "loveth", "all" ]
    );

    let string_keywords: Vec<String> = search_index.string_keywords(
        "Digby was a floccinaucinihilipilificator at heart—which is an \
        eight-dollar word meaning a joker who does not believe in anything he \
        can't bite.",
        SplitContext::Indexing,
    );

    assert_eq!(string_keywords,
        [ "digby", "was", "heart", "which", "is", "eight", "dollar", "word",
        "meaning", "joker", "who", "does", "not", "believe", "anything", "he",
        "can't", "bite" ]
    );

    my_vec
        .iter()
        .enumerate()
        .for_each(|(index, element)|
            search_index.insert(&index, element)
        );

    let search_results = search_index.search("third william");
    assert_eq!(search_results, vec![&3]);

    let search_results = search_index.search_type(&SearchType::Keyword, "Wessex");
    assert_eq!(search_results, vec![&1]);

    // Search for `last` or `wessex`. `Edgar Ætheling` contains both keywords,
    // so he should be returned first. `Harold Godwinson` only contains `last`
    // so he should be returned last:
    let search_results = search_index.search_type(&SearchType::Or, "last Wessex");
    assert_eq!(search_results, vec![&1, &0]);

    let search_results = search_index.search_type(&SearchType::Or, "last England");
    assert_eq!(search_results, vec![&0, &1, &2]);

    let search_results = search_index.search_type(&SearchType::And, "Conqueror third");
    assert_eq!(search_results, vec![&3]);

    let search_results = search_index.search_type(&SearchType::Live, "Last m");
    assert_eq!(search_results, vec![&1]);

    // Ensure that fuzzy matching is working with live searches:
    let search_results = search_index.search_type(&SearchType::Live, "1066 Harry");
    assert_eq!(search_results, vec![&0]);

    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Keyword, "E");
    assert_eq!(autocomplete_options, vec!["edgar".to_string(), "edgar ætheling".to_string(), "england".to_string()]);

    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Global, "1100 e");
    assert_eq!(autocomplete_options, vec!["1100 edgar".to_string(), "1100 edgar ætheling".to_string(), "1100 england".to_string()]);

    // Test fuzzy-matching for global autocompletion:
    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Global, "1100 Englelund");
    assert_eq!(autocomplete_options, vec!["1100 england".to_string()]);

    // The only `w` keywords that `1087` should contain are `William` and
    // `William Rufus`. `Wessex` exists in the index but it is not related to
    // `1087`:
    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Context, "1087 W");
    assert_eq!(autocomplete_options, vec!["1087 william".to_string(), "1087 william rufus".to_string()]);

    // Test fuzzy-matching for context autocompletion:
    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Context, "1087 Willy");
    assert_eq!(autocomplete_options, vec!["1087 william".to_string(), "1087 william rufus".to_string()]);

    // Ensure that `Context` autocomplete works with an empty search string /
    // single keyword. Context autocomplete works in two parts - an `And` search
    // for the preceding keywords, and an autocomplete for the last keyword:
    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Context, "108");
    assert_eq!(autocomplete_options, vec!["1087".to_string()]);

    // Test internal global fuzzy keyword search interface:
    let similar_keyword = search_index.strsim_global_keyword(&"Willy".to_lowercase());
    assert_eq!(similar_keyword, Some(&"william".to_string()));

    // Test internal global fuzzy autocompletion interface:
    let similar_autocompletions = search_index.strsim_global_autocomplete(&"Normy".to_lowercase());
    let similar_autocompletions_vec: Vec<&String> = similar_autocompletions.into_iter().map(|(keyword, _keys)| keyword).collect();
    assert_eq!(similar_autocompletions_vec, vec![&"norman".to_string()]);

    // Test `Indexable` trait implementation for `ToString` generics:
    let my_vec: Vec<&str> = vec![
        "Vopnafjarðarhreppur",                      // 0
        "Weapon Fjord Municipality",                // 1
        "Annerveenschekanaal",                      // 2
        "Channel through the peat of Annen",        // 3
        "Cadibarrawirracanna",                      // 4
        "The stars were dancing",                   // 5
        "Newtownmountkennedy",                      // 6
        "A new town near Mt. Kennedy",              // 7
        "Cottonshopeburnfoot",                      // 8
        "The end of the Cottonshope Burn",          // 9
        "Nyugotszenterzsébet",                      // 10
        "Western St. Elizabeth",                    // 11
        "Balatonszentgyörgy",                       // 12
        "St. George by Balaton",                    // 13
        "Kirkjubæjarklaustur",                      // 14
        "Church farm monastery",                    // 15
        "Jászalsószentgyörgy",                      // 16
        "Lower St. George in Jászság",              // 17
        "Krammerjachtensluis",                      // 18
        "Lock on the river Krammer of the hunt",    // 19
    ]; // vec!

    let mut search_index: SearchIndex<usize> = SearchIndex::default();

    my_vec
        .iter()
        .enumerate()
        .for_each(|(index, element)|
            search_index.insert(&index, element)
        );

    // Keyword search:
    let search_results = search_index.search_type(&SearchType::Keyword, "Cottonshope");
    assert_eq!(search_results, vec![&9]);

    // Or search:
    let search_results = search_index.search_type(&SearchType::Or, "George Elizabeth");
    assert_eq!(search_results, vec![&11, &13, &17]);

    // And search:
    let search_results = search_index.search_type(&SearchType::And, "George Jászság");
    assert_eq!(search_results, vec![&17]);

    // Live search:
    let search_results = search_index.search_type(&SearchType::Live, "Geo");
    assert_eq!(search_results, vec![&13, &17]);

    // Fuzzy matching:
    let search_results = search_index.search_type(&SearchType::Live, "rivers");
    assert_eq!(search_results, vec![&19]);

    // Fuzzy matching:
    let search_results = search_index.search_type(&SearchType::Live, "peet of Annan");
    assert_eq!(search_results, vec![&3]);

    // Keyword autocomplete:
    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Keyword, "Chan");
    assert_eq!(autocomplete_options, vec!["channel".to_string()]);

    // Global autocomplete:
    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Global, "Lo");
    assert_eq!(autocomplete_options, vec!["lock".to_string(), "lower".to_string()]);

    // Context autocomplete:
    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Context, "Krammer Lo");
    assert_eq!(autocomplete_options, vec!["krammer lock".to_string()]);

    // Context autocomplete:
    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Context, "stars are dancers");
    assert_eq!(autocomplete_options, vec!["stars are dancing".to_string()]);

} // fn