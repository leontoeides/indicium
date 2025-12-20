// -----------------------------------------------------------------------------
//
/// This is used to select a string similarity metric implemented by Danny Guo's
/// [strsim](https://crates.io/crates/strsim) crate.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum StrsimMetric {
    /// Like optimal string alignment, but substrings can be edited an unlimited
    /// number of times, and the triangle inequality holds.
    DamerauLevenshtein = 0,

    /// Calculates the Jaro similarity between two sequences. The returned value
    /// is between 0.0 and 1.0 (higher value means more similar).
    Jaro = 1,

    /// Like Jaro but gives a boost to sequences that have a common prefix.
    JaroWinkler = 2,

    /// Calculates the minimum number of insertions, deletions, and
    /// substitutions required to change one string into the other.
    #[default] Levenshtein = 3,

    /// Calculates a Sørensen-Dice similarity distance using bigrams.
    /// See <http://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient>.
    SorensenDice = 4,
}