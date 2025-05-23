use rand::rngs::ThreadRng;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use indicium::simple::{SearchIndex, SearchType};
use rand::Rng;

fn random_all_word(rng: &mut ThreadRng) -> String {
    "all".to_owned() + &rng.gen::<gabble::Gab>().to_string()
}

fn criterion_benchmark(c: &mut Criterion) {
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

    let chars: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
        'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w',
        'x', 'y', 'z'];

    let mut search_index: SearchIndex<usize> = SearchIndex::default();

    all_vec
        .iter()
        .enumerate()
        .for_each(|(index, element)| search_index.insert(&index, element));

    let mut rng: ThreadRng = rand::thread_rng();

    // Search for words contained in the `all_vec` set. Since these words
    // already exist, no autocompletion or fuzzy-matching will be performed:
    c.bench_function("all277_words_from_set", |b| b.iter(|| {
        let word = &all_vec[rng.gen_range(0..276)];
        search_index.search_type(&SearchType::Live, black_box(word));
    } ));

    // Search for `all` repeatedly. The word `all` doesn't exist in the set,
    // so this will repeatedly autocomplete the text "all". No fuzzy-matching
    // will be performed in this scenario:
    c.bench_function("all277_stub_autocompletion", |b| b.iter(|| {
        search_index.search_type(&SearchType::Live, black_box("all"));
    } ));

    // Search for words that start with "all" + a single random character at
    // the end. For example: `ally`, `allz`, `alla`, etc. Some words will be
    // autocompleted, and others will be fuzzy matched:
    c.bench_function("all277_random_partial_words", |b| b.iter(|| {
        let word = "all".to_string() + &String::from(chars[rng.gen_range(0..25)]);
        search_index.search_type(&SearchType::Live, black_box(&word));
    } ));

    // Search for words that start with "all" + a random pseudo-word at the
    // end. For example: `allilaincy`, `allgruimsab`, `allduieyaior`, etc.
    // Pretty much all words will be fuzzy matched:
    c.bench_function("all277_random_full_words", |b| b.iter(|| {
        let word = random_all_word(&mut rng);
        search_index.search_type(&SearchType::Live, black_box(&word));
    } ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);