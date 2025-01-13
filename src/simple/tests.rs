#![allow(clippy::cognitive_complexity)]
#![allow(clippy::too_many_lines)]
#[test]
fn simple() {
    use crate::simple::internal::string_keywords::SplitContext;
    use crate::simple::{AutocompleteType, Indexable, SearchIndex, SearchType};
    use kstring::KString;
    use pretty_assertions::assert_eq;

    #[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    struct MyStruct {
        title: String,
        year: u16,
        body: String,
    }

    impl Indexable for MyStruct {
        fn strings(&self) -> Vec<String> {
            vec![self.title.clone(), self.year.to_string(), self.body.clone()]
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

    let string_keywords: Vec<KString> = search_index.string_keywords(
        "All is not lost, the unconquerable will, and study of revenge, \
        immortal hate, and the courage never to submit or yield.",
        &SplitContext::Indexing,
    );

    assert_eq!(
        string_keywords,
        [
            "all",
            "is",
            "not",
            "lost",
            "unconquerable",
            "will",
            "study",
            "revenge",
            "immortal",
            "hate",
            "courage",
            "never",
            "submit",
            "yield"
        ]
    );

    let string_keywords: Vec<KString> = search_index.string_keywords(
        "He prayeth best, who loveth best All things both great and small; For \
        the dear God who loveth us, He made and loveth all.",
        &SplitContext::Searching,
    );

    assert_eq!(
        string_keywords,
        [
            "he", "prayeth", "best", "who", "loveth", "best", "all", "things", "both", "great",
            "small", "dear", "god", "who", "loveth", "us", "he", "made", "loveth", "all"
        ]
    );

    let string_keywords: Vec<KString> = search_index.string_keywords(
        "Digby was a floccinaucinihilipilificator at heart—which is an \
        eight-dollar word meaning a joker who does not believe in anything he \
        can't bite.",
        &SplitContext::Indexing,
    );

    assert_eq!(
        string_keywords,
        [
            "digby", "was", "heart", "which", "is", "eight", "dollar", "word", "meaning", "joker",
            "who", "does", "not", "believe", "anything", "he", "can't", "bite"
        ]
    );

    my_vec
        .iter()
        .enumerate()
        .for_each(|(index, element)| search_index.insert(&index, element));

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
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    let search_results = search_index.search_type(&SearchType::Live, "1066 Harry");
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    assert_eq!(search_results, vec![&0]);

    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Keyword, "E");
    assert_eq!(
        autocomplete_options,
        vec![
            "edgar".to_string(),
            "edgar ætheling".to_string(),
            "england".to_string()
        ]
    );

    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Global, "1100 e");
    assert_eq!(
        autocomplete_options,
        vec![
            "1100 edgar".to_string(),
            "1100 edgar ætheling".to_string(),
            "1100 england".to_string()
        ]
    );

    // Test fuzzy-matching for global autocompletion:
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    let autocomplete_options =
        search_index.autocomplete_type(&AutocompleteType::Global, "1100 Englelund");
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    assert_eq!(autocomplete_options, vec!["1100 england".to_string()]);

    // The only `w` keywords that `1087` should contain are `William` and
    // `William Rufus`. `Wessex` exists in the index but it is not related to
    // `1087`:
    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Context, "1087 W");
    assert_eq!(
        autocomplete_options,
        vec!["1087 william".to_string(), "1087 william rufus".to_string()]
    );

    // Test fuzzy-matching for context autocompletion:
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    let autocomplete_options =
        search_index.autocomplete_type(&AutocompleteType::Context, "1087 Willy");
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    assert_eq!(
        autocomplete_options,
        vec!["1087 william".to_string(), "1087 william rufus".to_string()]
    );

    // Ensure that `Context` autocomplete works with an empty search string /
    // single keyword. Context autocomplete works in two parts - an `And` search
    // for the preceding keywords, and an autocomplete for the last keyword:
    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Context, "108");
    assert_eq!(autocomplete_options, vec!["1087".to_string()]);

    // Test internal global fuzzy keyword search interface:
    #[cfg(feature = "eddie")]
    let similar_keyword = search_index.eddie_global_keyword(&"Willy".to_lowercase());
    #[cfg(feature = "rapidfuzz")]
    let similar_keyword = search_index.rapidfuzz_keyword_global(&"Willy".to_lowercase());
    #[cfg(feature = "strsim")]
    let similar_keyword = search_index.strsim_global_keyword(&"Willy".to_lowercase());
    #[cfg(any(feature = "eddie", feature = "strsim"))]
    assert_eq!(similar_keyword, Some(&KString::from_ref("william")));
    #[cfg(feature = "rapidfuzz")]
    assert_eq!(similar_keyword, Some("william"));

    // Test internal global fuzzy autocompletion interface:
    #[cfg(feature = "eddie")]
    let similar_autocompletions = search_index.eddie_global_autocomplete(&"Normy".to_lowercase());
    #[cfg(feature = "rapidfuzz")]
    let similar_autocompletions = search_index.rapidfuzz_autocomplete_global(&"Normy".to_lowercase());
    #[cfg(feature = "strsim")]
    let similar_autocompletions = search_index.strsim_global_autocomplete(&"Normy".to_lowercase());
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    let similar_autocompletions_vec: Vec<&KString> = similar_autocompletions
        .into_iter()
        .map(|(keyword, _keys)| keyword)
        .collect();
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    assert_eq!(similar_autocompletions_vec, vec![&"norman".to_string()]);

    // Test `Indexable` trait implementation for `ToString` generics:

    let my_vec: Vec<&str> = vec![
        "Vopnafjarðarhreppur",                   // 0
        "Weapon Fjord Municipality",             // 1
        "Annerveenschekanaal",                   // 2
        "Channel through the peat of Annen",     // 3
        "Cadibarrawirracanna",                   // 4
        "The stars were dancing",                // 5
        "Newtownmountkennedy",                   // 6
        "A new town near Mt. Kennedy",           // 7
        "Cottonshopeburnfoot",                   // 8
        "The end of the Cottonshope Burn",       // 9
        "Nyugotszenterzsébet",                   // 10
        "Western St. Elizabeth",                 // 11
        "Balatonszentgyörgy",                    // 12
        "St. George by Balaton",                 // 13
        "Kirkjubæjarklaustur",                   // 14
        "Church farm monastery",                 // 15
        "Jászalsószentgyörgy",                   // 16
        "Lower St. George in Jászság",           // 17
        "Krammerjachtensluis",                   // 18
        "Lock on the river Krammer of the hunt", // 19
    ]; // vec!

    let mut search_index: SearchIndex<usize> = SearchIndex::default();

    my_vec
        .iter()
        .enumerate()
        .for_each(|(index, element)| search_index.insert(&index, element));

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
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    let search_results = search_index.search_type(&SearchType::Live, "rivers");
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    assert_eq!(search_results, vec![&19]);

    // Fuzzy matching:
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    let search_results = search_index.search_type(&SearchType::Live, "peat of Annan");
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    assert_eq!(search_results, vec![&3]);

    // Keyword autocomplete:
    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Keyword, "Chan");
    assert_eq!(autocomplete_options, vec!["channel".to_string()]);

    // Global autocomplete:
    let autocomplete_options = search_index.autocomplete_type(&AutocompleteType::Global, "Lo");
    assert_eq!(
        autocomplete_options,
        vec!["lock".to_string(), "lower".to_string()]
    );

    // Context autocomplete:
    let autocomplete_options =
        search_index.autocomplete_type(&AutocompleteType::Context, "Krammer Lo");
    assert_eq!(autocomplete_options, vec!["krammer lock".to_string()]);

    // Fuzzy matching context autocomplete:
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    let autocomplete_options =
        search_index.autocomplete_type(&AutocompleteType::Context, "stars are dancers");
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    assert_eq!(autocomplete_options, vec!["stars are dancing".to_string()]);

    // Test UTF-8:
    let mut search_index = crate::simple::SearchIndex::<usize>::default();
    search_index.insert(&0, &"лол"); // lol in Cyrillic
    search_index.insert(&1, &"lol");
    search_index.insert(&2, &"lol лол");
    search_index.insert(&3, &"лол lol");
    let search_results = search_index.search("лол");
    assert_eq!(search_results, vec![&0, &2, &3]);

    // The "all" test. Looks at 277 words that start with the prefix "all":

    let all_vec: Vec<&str> = vec![
        "allanite",         // 1
        "allanites",        // 2
        "allantoic",        // 3
        "allantoid",        // 4
        "allantoides",      // 5
        "allantoids",       // 6
        "allantoin",        // 7
        "allantoins",       // 8
        "allantois",        // 9
        "allargando",       // 10
        "allay",            // 11
        "allayed",          // 12
        "allayer",          // 13
        "allayers",         // 14
        "allaying",         // 15
        "allays",           // 16
        "allee",            // 17
        "allees",           // 18
        "allegation",       // 19
        "allegations",      // 20
        "allege",           // 21
        "alleged",          // 22
        "allegedly",        // 23
        "alleger",          // 24
        "allegers",         // 25
        "alleges",          // 26
        "allegiance",       // 27
        "allegiances",      // 28
        "allegiant",        // 29
        "allegiants",       // 30
        "alleging",         // 31
        "allegoric",        // 32
        "allegorical",      // 33
        "allegorically",    // 34
        "allegoricalness",  // 35
        "allegories",       // 36
        "allegorise",       // 37
        "allegorised",      // 38
        "allegorises",      // 39
        "allegorising",     // 40
        "allegorist",       // 41
        "allegorists",      // 42
        "allegorization",   // 43
        "allegorizations",  // 44
        "allegorize",       // 45
        "allegorized",      // 46
        "allegorizer",      // 47
        "allegorizers",     // 48
        "allegorizes",      // 49
        "allegorizing",     // 50
        "allegory",         // 51
        "allegretto",       // 52
        "allegrettos",      // 53
        "allegro",          // 54
        "allegros",         // 55
        "allele",           // 56
        "alleles",          // 57
        "allelic",          // 58
        "allelism",         // 59
        "allelisms",        // 60
        "allelomorph",      // 61
        "allelomorphic",    // 62
        "allelomorphism",   // 63
        "allelomorphisms",  // 64
        "allelomorphs",     // 65
        "allelopathic",     // 66
        "allelopathies",    // 67
        "allelopathy",      // 68
        "alleluia",         // 69
        "alleluias",        // 70
        "allemande",        // 71
        "allemandes",       // 72
        "allergen",         // 73
        "allergenic",       // 74
        "allergenicities",  // 75
        "allergenicity",    // 76
        "allergens",        // 77
        "allergic",         // 78
        "allergies",        // 79
        "allergin",         // 80
        "allergins",        // 81
        "allergist",        // 82
        "allergists",       // 83
        "allergy",          // 84
        "allethrin",        // 85
        "allethrins",       // 86
        "alleviant",        // 87
        "alleviants",       // 88
        "alleviate",        // 89
        "alleviated",       // 90
        "alleviates",       // 91
        "alleviating",      // 92
        "alleviation",      // 93
        "alleviations",     // 94
        "alleviator",       // 95
        "alleviators",      // 96
        "alley",            // 97
        "alleys",           // 98
        "alleyway",         // 99
        "alleyways",        // 100
        "allheal",          // 101
        "allheals",         // 102
        "alliable",         // 103
        "alliaceous",       // 104
        "alliance",         // 105
        "alliances",        // 106
        "allicin",          // 107
        "allicins",         // 108
        "allied",           // 109
        "allies",           // 110
        "alligator",        // 111
        "alligators",       // 112
        "alliterate",       // 113
        "alliterated",      // 114
        "alliterates",      // 115
        "alliterating",     // 116
        "alliteration",     // 117
        "alliterations",    // 118
        "alliterative",     // 119
        "alliteratively",   // 120
        "allium",           // 121
        "alliums",          // 122
        "alloantibodies",   // 123
        "alloantibody",     // 124
        "alloantigen",      // 125
        "alloantigens",     // 126
        "allobar",          // 127
        "allobars",         // 128
        "allocable",        // 129
        "allocatable",      // 130
        "allocate",         // 131
        "allocated",        // 132
        "allocates",        // 133
        "allocating",       // 134
        "allocation",       // 135
        "allocations",      // 136
        "allocator",        // 137
        "allocators",       // 138
        "allocution",       // 139
        "allocutions",      // 140
        "allod",            // 141
        "allodia",          // 142
        "allodial",         // 143
        "allodium",         // 144
        "allods",           // 145
        "allogamies",       // 146
        "allogamous",       // 147
        "allogamy",         // 148
        "allogeneic",       // 149
        "allogenic",        // 150
        "allograft",        // 151
        "allografted",      // 152
        "allografting",     // 153
        "allografts",       // 154
        "allograph",        // 155
        "allographic",      // 156
        "allographs",       // 157
        "allometric",       // 158
        "allometries",      // 159
        "allometry",        // 160
        "allomorph",        // 161
        "allomorphic",      // 162
        "allomorphism",     // 163
        "allomorphisms",    // 164
        "allomorphs",       // 165
        "allonge",          // 166
        "allonges",         // 167
        "allonym",          // 168
        "allonyms",         // 169
        "allopath",         // 170
        "allopathies",      // 171
        "allopaths",        // 172
        "allopathy",        // 173
        "allopatric",       // 174
        "allopatrically",   // 175
        "allopatries",      // 176
        "allopatry",        // 177
        "allophane",        // 178
        "allophanes",       // 179
        "allophone",        // 180
        "allophones",       // 181
        "allophonic",       // 182
        "alloplasm",        // 183
        "alloplasms",       // 184
        "allopolyploid",    // 185
        "allopolyploids",   // 186
        "allopolyploidy",   // 187
        "allopurinol",      // 188
        "allopurinols",     // 189
        "allosaur",         // 190
        "allosaurs",        // 191
        "allosaurus",       // 192
        "allosauruses",     // 193
        "allosteric",       // 194
        "allosterically",   // 195
        "allosteries",      // 196
        "allostery",        // 197
        "allot",            // 198
        "allotetraploid",   // 199
        "allotetraploids",  // 200
        "allotetraploidy",  // 201
        "allotment",        // 202
        "allotments",       // 203
        "allotrope",        // 204
        "allotropes",       // 205
        "allotropic",       // 206
        "allotropies",      // 207
        "allotropy",        // 208
        "allots",           // 209
        "allotted",         // 210
        "allottee",         // 211
        "allottees",        // 212
        "allotter",         // 213
        "allotters",        // 214
        "allotting",        // 215
        "allotype",         // 216
        "allotypes",        // 217
        "allotypic",        // 218
        "allotypically",    // 219
        "allotypies",       // 220
        "allotypy",         // 221
        "allover",          // 222
        "allovers",         // 223
        "allow",            // 224
        "allowable",        // 225
        "allowables",       // 226
        "allowably",        // 227
        "allowance",        // 228
        "allowanced",       // 229
        "allowances",       // 230
        "allowancing",      // 231
        "allowed",          // 232
        "allowedly",        // 233
        "allowing",         // 234
        "allows",           // 235
        "alloxan",          // 236
        "alloxans",         // 237
        "alloy",            // 238
        "alloyed",          // 239
        "alloying",         // 240
        "alloys",           // 241
        "alls",             // 242
        "allseed",          // 243
        "allseeds",         // 244
        "allspice",         // 245
        "allspices",        // 246
        "allude",           // 247
        "alluded",          // 248
        "alludes",          // 249
        "alluding",         // 250
        "allure",           // 251
        "allured",          // 252
        "allurement",       // 253
        "allurements",      // 254
        "allurer",          // 255
        "allurers",         // 256
        "allures",          // 257
        "alluring",         // 258
        "alluringly",       // 259
        "allusion",         // 260
        "allusions",        // 261
        "allusive",         // 262
        "allusively",       // 263
        "allusiveness",     // 264
        "allusivenesses",   // 265
        "alluvia",          // 266
        "alluvial",         // 267
        "alluvials",        // 268
        "alluvion",         // 269
        "alluvions",        // 270
        "alluvium",         // 271
        "alluviums",        // 272
        "ally",             // 273
        "allying",          // 274
        "allyl",            // 275
        "allylic",          // 276
        "allyls",           // 277
    ]; // vec!

    let mut search_index: SearchIndex<usize> = SearchIndex::default();

    all_vec
        .iter()
        .enumerate()
        .for_each(|(index, element)| search_index.insert(&index, element));

    // Fuzzy matching:
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    let search_results = search_index.search_type(&SearchType::Live, "ally");
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    let search_results: Vec<&str> = search_results.into_iter().map(|key| all_vec[*key]).collect();
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    assert_eq!(search_results, vec!["ally", "allying", "allyl", "allylic", "allyls"]);
} // fn