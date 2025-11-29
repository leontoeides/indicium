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

    // Ensure that the default index splitting behaves as expected:

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

    // Ensure that the default search splitting behaves as expected:

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

    // Ensure that the default index splitting removes very short and very long
    // words:

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

    // Continue search tests:
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
    assert!(autocomplete_options.contains(&"1100 england".to_string()));

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
    /* #[cfg(feature = "eddie")]
    let similar_keyword = search_index.eddie_substitute(&"Willy".to_lowercase());
    #[cfg(feature = "rapidfuzz")]
    let similar_keyword = search_index.rapidfuzz_substitute(&"Willy".to_lowercase());
    #[cfg(feature = "strsim")]
    let similar_keyword = search_index.strsim_substitute(&"Willy".to_lowercase());
    #[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
    assert_eq!(similar_keyword, Some("william")); */

    // Test internal global fuzzy autocompletion interface:
    #[cfg(feature = "eddie")]
    let similar_autocompletions = search_index.eddie_global(&[], &"Normy".to_lowercase());
    #[cfg(feature = "rapidfuzz")]
    let similar_autocompletions = search_index.rapidfuzz_global(&[], &"Normy".to_lowercase());
    #[cfg(feature = "strsim")]
    let similar_autocompletions = search_index.strsim_global(&[], &"Normy".to_lowercase());
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
} // fn

#[test]
fn simple_all() {
    use crate::simple::SearchIndex;
    #[cfg(any(feature = "strsim", feature = "eddie", feature = "rapidfuzz"))]
    use crate::simple::SearchType;
    #[cfg(any(feature = "strsim", feature = "eddie", feature = "rapidfuzz"))]
    use pretty_assertions::assert_eq;

    // The "all" test. Looks at 277 words that start with the prefix "all":

    let all_vec: Vec<&str> = vec![
        "allanite",         // 0
        "allanites",        // 1
        "allantoic",        // 2
        "allantoid",        // 3
        "allantoides",      // 4
        "allantoids",       // 5
        "allantoin",        // 6
        "allantoins",       // 7
        "allantois",        // 8
        "allargando",       // 9
        "allay",            // 10
        "allayed",          // 11
        "allayer",          // 12
        "allayers",         // 13
        "allaying",         // 14
        "allays",           // 15
        "allee",            // 16
        "allees",           // 17
        "allegation",       // 18
        "allegations",      // 19
        "allege",           // 20
        "alleged",          // 21
        "allegedly",        // 22
        "alleger",          // 23
        "allegers",         // 24
        "alleges",          // 25
        "allegiance",       // 26
        "allegiances",      // 27
        "allegiant",        // 28
        "allegiants",       // 29
        "alleging",         // 30
        "allegoric",        // 31
        "allegorical",      // 32
        "allegorically",    // 33
        "allegoricalness",  // 34
        "allegories",       // 35
        "allegorise",       // 36
        "allegorised",      // 37
        "allegorises",      // 38
        "allegorising",     // 39
        "allegorist",       // 40
        "allegorists",      // 41
        "allegorization",   // 42
        "allegorizations",  // 43
        "allegorize",       // 44
        "allegorized",      // 45
        "allegorizer",      // 46
        "allegorizers",     // 47
        "allegorizes",      // 48
        "allegorizing",     // 49
        "allegory",         // 50
        "allegretto",       // 51
        "allegrettos",      // 52
        "allegro",          // 53
        "allegros",         // 54
        "allele",           // 55
        "alleles",          // 56
        "allelic",          // 57
        "allelism",         // 58
        "allelisms",        // 59
        "allelomorph",      // 60
        "allelomorphic",    // 61
        "allelomorphism",   // 62
        "allelomorphisms",  // 63
        "allelomorphs",     // 64
        "allelopathic",     // 65
        "allelopathies",    // 66
        "allelopathy",      // 67
        "alleluia",         // 68
        "alleluias",        // 69
        "allemande",        // 70
        "allemandes",       // 71
        "allergen",         // 72
        "allergenic",       // 73
        "allergenicities",  // 74
        "allergenicity",    // 75
        "allergens",        // 76
        "allergic",         // 77
        "allergies",        // 78
        "allergin",         // 79
        "allergins",        // 80
        "allergist",        // 81
        "allergists",       // 82
        "allergy",          // 83
        "allethrin",        // 84
        "allethrins",       // 85
        "alleviant",        // 86
        "alleviants",       // 87
        "alleviate",        // 88
        "alleviated",       // 89
        "alleviates",       // 90
        "alleviating",      // 91
        "alleviation",      // 92
        "alleviations",     // 93
        "alleviator",       // 94
        "alleviators",      // 95
        "alley",            // 96
        "alleys",           // 97
        "alleyway",         // 98
        "alleyways",        // 99
        "allheal",          // 100
        "allheals",         // 101
        "alliable",         // 102
        "alliaceous",       // 103
        "alliance",         // 104
        "alliances",        // 105
        "allicin",          // 106
        "allicins",         // 107
        "allied",           // 108
        "allies",           // 109
        "alligator",        // 110
        "alligators",       // 111
        "alliterate",       // 112
        "alliterated",      // 113
        "alliterates",      // 114
        "alliterating",     // 115
        "alliteration",     // 116
        "alliterations",    // 117
        "alliterative",     // 118
        "alliteratively",   // 119
        "allium",           // 120
        "alliums",          // 121
        "alloantibodies",   // 122
        "alloantibody",     // 123
        "alloantigen",      // 124
        "alloantigens",     // 125
        "allobar",          // 126
        "allobars",         // 127
        "allocable",        // 128
        "allocatable",      // 129
        "allocate",         // 130
        "allocated",        // 131
        "allocates",        // 132
        "allocating",       // 133
        "allocation",       // 134
        "allocations",      // 135
        "allocator",        // 136
        "allocators",       // 137
        "allocution",       // 138
        "allocutions",      // 139
        "allod",            // 140
        "allodia",          // 141
        "allodial",         // 142
        "allodium",         // 143
        "allods",           // 144
        "allogamies",       // 145
        "allogamous",       // 146
        "allogamy",         // 147
        "allogeneic",       // 148
        "allogenic",        // 149
        "allograft",        // 150
        "allografted",      // 151
        "allografting",     // 152
        "allografts",       // 153
        "allograph",        // 154
        "allographic",      // 155
        "allographs",       // 156
        "allometric",       // 157
        "allometries",      // 158
        "allometry",        // 159
        "allomorph",        // 160
        "allomorphic",      // 161
        "allomorphism",     // 162
        "allomorphisms",    // 163
        "allomorphs",       // 164
        "allonge",          // 165
        "allonges",         // 166
        "allonym",          // 167
        "allonyms",         // 168
        "allopath",         // 169
        "allopathies",      // 170
        "allopaths",        // 171
        "allopathy",        // 172
        "allopatric",       // 173
        "allopatrically",   // 174
        "allopatries",      // 175
        "allopatry",        // 176
        "allophane",        // 177
        "allophanes",       // 178
        "allophone",        // 179
        "allophones",       // 180
        "allophonic",       // 181
        "alloplasm",        // 182
        "alloplasms",       // 183
        "allopolyploid",    // 184
        "allopolyploids",   // 185
        "allopolyploidy",   // 186
        "allopurinol",      // 187
        "allopurinols",     // 188
        "allosaur",         // 189
        "allosaurs",        // 190
        "allosaurus",       // 191
        "allosauruses",     // 192
        "allosteric",       // 193
        "allosterically",   // 194
        "allosteries",      // 195
        "allostery",        // 196
        "allot",            // 197
        "allotetraploid",   // 198
        "allotetraploids",  // 199
        "allotetraploidy",  // 200
        "allotment",        // 201
        "allotments",       // 202
        "allotrope",        // 203
        "allotropes",       // 204
        "allotropic",       // 205
        "allotropies",      // 206
        "allotropy",        // 207
        "allots",           // 208
        "allotted",         // 209
        "allottee",         // 210
        "allottees",        // 211
        "allotter",         // 212
        "allotters",        // 213
        "allotting",        // 214
        "allotype",         // 215
        "allotypes",        // 216
        "allotypic",        // 217
        "allotypically",    // 218
        "allotypies",       // 219
        "allotypy",         // 220
        "allover",          // 221
        "allovers",         // 222
        "allow",            // 223
        "allowable",        // 224
        "allowables",       // 225
        "allowably",        // 226
        "allowance",        // 227
        "allowanced",       // 228
        "allowances",       // 229
        "allowancing",      // 230
        "allowed",          // 231
        "allowedly",        // 232
        "allowing",         // 233
        "allows",           // 234
        "alloxan",          // 235
        "alloxans",         // 236
        "alloy",            // 237
        "alloyed",          // 238
        "alloying",         // 239
        "alloys",           // 240
        "alls",             // 241
        "allseed",          // 242
        "allseeds",         // 243
        "allspice",         // 244
        "allspices",        // 245
        "allude",           // 246
        "alluded",          // 247
        "alludes",          // 248
        "alluding",         // 249
        "allure",           // 250
        "allured",          // 251
        "allurement",       // 252
        "allurements",      // 253
        "allurer",          // 254
        "allurers",         // 255
        "allures",          // 256
        "alluring",         // 257
        "alluringly",       // 258
        "allusion",         // 259
        "allusions",        // 260
        "allusive",         // 261
        "allusively",       // 262
        "allusiveness",     // 263
        "allusivenesses",   // 264
        "alluvia",          // 265
        "alluvial",         // 266
        "alluvials",        // 267
        "alluvion",         // 268
        "alluvions",        // 269
        "alluvium",         // 270
        "alluviums",        // 271
        "ally",             // 272
        "allying",          // 273
        "allyl",            // 274
        "allylic",          // 275
        "allyls",           // 276
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

#[test]
#[cfg(feature = "unicode-normalization")]
fn unicode_normalization() {
    use crate::simple::{SearchIndex, SearchType};
    use pretty_assertions::assert_eq;

    let mut search_index: SearchIndex<usize> = SearchIndex::default();

    // Compatibility equivalents that NFKC should normalize:
    search_index.insert(&0, &"file");           // ASCII "fi"
    search_index.insert(&1, &"ﬁle");            // U+FB01 Latin small ligature fi
    search_index.insert(&2, &"ﬂood");           // U+FB02 Latin small ligature fl
    search_index.insert(&3, &"flood");          // ASCII "fl"
    search_index.insert(&4, &"①②③");           // Circled digits
    search_index.insert(&5, &"123");            // ASCII digits
    search_index.insert(&6, &"Ω resistor");     // U+2126 Ohm sign
    search_index.insert(&7, &"Ω resistor");     // U+03A9 Greek capital omega
    search_index.insert(&8, &"ｆｕｌｌｗｉｄｔｈ");  // Fullwidth ASCII
    search_index.insert(&9, &"fullwidth");      // ASCII

    // Ligature searches should find both forms:
    let results = search_index.search_type(&SearchType::Keyword, "file");
    assert_eq!(results, vec![&0, &1]);

    let results = search_index.search_type(&SearchType::Keyword, "ﬁle");
    assert_eq!(results, vec![&0, &1]);

    let results = search_index.search_type(&SearchType::Keyword, "flood");
    assert_eq!(results, vec![&2, &3]);

    // Ohm sign and Greek omega should match:
    let results = search_index.search_type(&SearchType::Keyword, "resistor");
    assert_eq!(results, vec![&6, &7]);

    // Fullwidth and ASCII should match:
    let results = search_index.search_type(&SearchType::Keyword, "fullwidth");
    assert_eq!(results, vec![&8, &9]);

    let results = search_index.search_type(&SearchType::Keyword, "ｆｕｌｌｗｉｄｔｈ");
    assert_eq!(results, vec![&8, &9]);
}

#[test]
#[cfg(feature = "unicode-normalization")]
fn unicode_normalization_case_insensitive() {
    use crate::simple::{SearchIndex, SearchType};
    use pretty_assertions::assert_eq;

    let mut search_index: SearchIndex<usize> = SearchIndex::default();

    // Case folding combined with normalization:
    search_index.insert(&0, &"ＦＩＬＥ");  // Fullwidth uppercase
    search_index.insert(&1, &"ﬁle");       // Ligature lowercase
    search_index.insert(&2, &"FILE");      // ASCII uppercase
    search_index.insert(&3, &"file");      // ASCII lowercase

    // All four should match regardless of search case:
    let results = search_index.search_type(&SearchType::Keyword, "file");
    assert_eq!(results, vec![&0, &1, &2, &3]);

    let results = search_index.search_type(&SearchType::Keyword, "FILE");
    assert_eq!(results, vec![&0, &1, &2, &3]);
}

#[test]
#[cfg(feature = "unicode-normalization")]
fn unicode_normalization_case_sensitive() {
    use crate::simple::{SearchIndex, SearchIndexBuilder, SearchType};
    use pretty_assertions::assert_eq;

    let mut search_index: SearchIndex<usize> = SearchIndexBuilder::default()
        .case_sensitive(true)
        .build();

    // Case folding combined with normalization:
    search_index.insert(&0, &"ＦＩＬＥ");  // Fullwidth uppercase
    search_index.insert(&1, &"ﬁle");       // Ligature lowercase
    search_index.insert(&2, &"FILE");      // ASCII uppercase
    search_index.insert(&3, &"file");      // ASCII lowercase

    // All four should match regardless of search case:
    let results = search_index.search_type(&SearchType::Keyword, "file");
    assert_eq!(results, vec![&1, &3]);

    let results = search_index.search_type(&SearchType::Keyword, "FILE");
    assert_eq!(results, vec![&0, &2]);
}