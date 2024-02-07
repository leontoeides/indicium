// -----------------------------------------------------------------------------
//
/// This is used to select a string similarity metric implemented by Ilia
/// Schelokov's [eddie](https://crates.io/crates/eddie) crate.

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EddieMetric {
    /// See [the detailed description](https://en.wikipedia.org/wiki/Levenshtein_distance).
    #[default]
    Levenshtein,
    /// See [the detailed description](https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance).
    DamerauLevenshtein,
    /// See [the detailed description](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance#Jaro_Similarity).
    Jaro,
    /// Like Jaro similarity but gives a higher score to the strings that start
    /// with the same sequence of characters. See
    /// [the detailed description](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance#Jaro%E2%80%93Winkler_Similarity).
    JaroWinkler,
} // EddieMetric
