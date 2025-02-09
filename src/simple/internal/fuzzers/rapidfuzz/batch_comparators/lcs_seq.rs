//! Longest Common Subsequence

#![allow(clippy::inline_always)]

use rapidfuzz::distance::lcs_seq::{Args, BatchComparator};

/// This `struct` is used to access the Longest Common Subsequence algorithm, as 
/// implemented by the [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in 
/// a generic manner.
///
/// The Longest Common Subsequence (LCS) measures the similarity between two
/// sequences by identifying the longest sequence of elements (characters,
/// numbers, etc.) that are common to both sequences. Importantly, the elements
/// in the common subsequence do not need to appear consecutively in the
/// original sequences.
///
/// It’s useful in applications where the order of elements is significant, but
/// their exact positions may vary. Common use cases involve:
///
/// * **Bioinformatics**: Commonly used in Bioinformatics for comparing genetic
///   sequences where identifying shared genes or regions, even if not
///   contiguous, is important.
///
/// * **Version Control Systems**: Tracking changes between different versions
///   of a document or codebase.
///
/// * **Plagiarism Detection**: Identifying similarities between texts even when
///   the wording is rearranged or some content is added or removed.
pub struct LcsSeq(BatchComparator<char>);

/// This `BatchComparator` trait implementation is used to access the
/// Longest Common Subsequence algorithm, as implemented by the
/// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate, in a generic manner.
impl crate::simple::internal::fuzzers::rapidfuzz::BatchComparator for LcsSeq {
    /// Instantiates a new batch comparator.
    #[must_use]
    #[inline(always)]
    fn new(one: &str) -> Self {
        Self(BatchComparator::new(one.chars()))
    } // fn

    /// Calculates normalized similarity.
    #[must_use]
    #[inline(always)]
    fn normalized_similarity(
        &self,
        many: &str,
        score_cutoff: f64
    ) -> Option<f64> {
        self.0.normalized_similarity_with_args(
            many.chars(),
            &Args::default().score_cutoff(score_cutoff)
        )
    } // fn
} // impl