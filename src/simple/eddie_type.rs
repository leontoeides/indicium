// -----------------------------------------------------------------------------
//
/// This is used to select a string similarity metric implemented by the
/// Ilia Schelokov's [eddie](https://crates.io/crates/eddie) crate.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EddieType {
    /// See [the detailed description](https://en.wikipedia.org/wiki/Levenshtein_distance).
    Levenshtein,
    /// See [the detailed description](https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance).
    DamerauLevenshtein,
    /// See [the detailed description](https://en.wikipedia.org/wiki/Hamming_distance).
    Hamming,
    /// See [the detailed description](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance#Jaro_Similarity).
    Jaro,
    /// Like Jaro similarity but gives a higher score to the strings that start
    /// with the same sequence of characters. See
    /// [the detailed description](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance#Jaro%E2%80%93Winkler_Similarity).
    JaroWinkler,
} // EddieType