// -----------------------------------------------------------------------------
//
/// This is used to select a string similarity metric implemented by the 
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RapidfuzzMetric {
    /// The Damerau-Levenshtein distance measures the minimum number of
    /// operations required to transform one string into another, considering
    /// four types of elementary edits: `insertions`, `deletions`,
    /// `substitutions`, and `transpositions of adjacent characters`. A
    /// transposition involves swapping two adjacent characters. It does respect
    /// triangle inequality, and is thus a metric distance.
    ///
    /// It’s often used in applications where transpositions are common. An
    /// example for this would be typing errors involving adjacent characters.
    DamerauLevenshtein,

    /// The Hamming distance measures the similarity of two sequences of equal
    /// length. Specifically, it counts the minimum number of substitutions
    /// required to transform one string into the other.
    ///
    /// While regularly the Hamming distance only works with texts of equal
    /// length, this implementation provides an addition argument `pad` to
    /// decide whether texts of unequal length should be padded or return an
    /// error.
    Hamming,

    /// The Indel distance is a specialized version of the `Levenshtein`
    /// distance with only insertions and deletions. It can be calculated from
    /// the `Longest Common Subsequence`.
    ///
    /// Similar to LCS it’s commonly used in Bioinformatics applications like
    /// DNA sequence analysis, where insertions and deletions play a crucial
    /// role in understanding evolutionary relationships and genetic variations.
    Indel,

    /// The Jaro similarity is a measure of similarity between two strings,
    /// often used in the field of record linkage and string matching. It’s
    /// particularly effective in comparing short strings, such as names. The
    /// algorithm considers both the common characters and their order in the
    /// strings, as well as the number of transpositions needed to make the
    /// strings equal.
    Jaro,

    /// The Jaro-Winkler similarity extends the Jaro similarity to provide
    /// additional sensitivity to matching prefixes. It introduces a scaling
    /// mechanism that boosts the similarity score for strings with common
    /// prefixes.
    JaroWinkler,

    /// The Longest Common Subsequence (LCS) measures the similarity between two
    /// sequences by identifying the longest sequence of elements (characters,
    /// numbers, etc.) that are common to both sequences. Importantly, the
    /// elements in the common subsequence do not need to appear consecutively
    /// in the original sequences.
    ///
    /// It’s useful in applications where the order of elements is significant,
    /// but their exact positions may vary. Common use cases involve:
    ///
    /// * **Bioinformatics**: Commonly used in Bioinformatics for comparing
    ///   genetic sequences where identifying shared genes or regions, even if
    ///   not contiguous, is important.
    ///
    /// * **Version Control Systems**: Tracking changes between different
    ///   versions of a document or codebase.
    ///
    /// * **Plagiarism Detection**: Identifying similarities between texts even
    ///   when the wording is rearranged or some content is added or removed.
    LcsSeq,

    /// The Levenshtein distance measures the minimum number of operations
    /// required to transform one string into another, considering three types
    /// of elementary edits: `insertions`, `deletions` and `substitutions`. It
    /// does respect triangle inequality, and is thus a metric distance.
    ///
    /// It finds use in various applications such as text processing, DNA
    /// sequence analysis, and data cleaning.
    Levenshtein,

    /// The Optimal String Alignment distance (OSA) measures the minimum number
    /// of operations required to transform one string into another, considering
    /// four types of elementary edits: `insertions`, `deletions`,
    /// `substitutions`, and `transpositions`.
    ///
    /// While both the `Damerau-Levenshtein` and OSA distance include
    /// transpositions, they differ in the treatment of transpositions. OSA
    /// treats any transposition as a single operation, regardless of whether
    /// the transposed characters are adjacent or not. In contrast, the
    /// Damerau-Levenshtein distance specifically allows transpositions of
    /// adjacent characters.
    ///
    /// The handling of transpositions in the OSA distance is simpler, which
    /// makes it computationally less intensive.
    #[default] Osa,

    /// The Postfix similarity measures the length of the common postfix between
    /// two sequences.
    Postfix,

    /// The Prefix similarity measures the length of the common prefix between
    /// two sequences.
    Prefix,
} // RapidfuzzMetric