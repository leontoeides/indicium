//! Integration with the [rapidfuzz](https://crates.io/crates/rapidfuzz)
//! string similarity crate.

// -----------------------------------------------------------------------------
//
// Interfaces for the `rapidfuzz` crate integration.

pub mod autocomplete_global;
pub mod autocomplete_global_comparator;

pub mod autocomplete_context;
pub mod autocomplete_context_comparator;

pub mod keyword_global;
pub mod keyword_global_comparator;

// -----------------------------------------------------------------------------
//
// The `BatchComparator` trait allows `indicium` to treat the various distance
// and string similarity algorithms in the
// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate generically.

mod batch_comparator;

use crate::simple::internal::rapidfuzz::batch_comparator::BatchComparator;

// -----------------------------------------------------------------------------
//
// The `BatchComparator` implementations for the various string distance and
// string similarity algorithms in the `rapidfuzz` crate.

mod batch_comparators;

use crate::simple::internal::rapidfuzz::batch_comparators::damerau_levenshtein::DamerauLevenshtein;
use crate::simple::internal::rapidfuzz::batch_comparators::hamming::Hamming;
use crate::simple::internal::rapidfuzz::batch_comparators::indel::Indel;
use crate::simple::internal::rapidfuzz::batch_comparators::jaro::Jaro;
use crate::simple::internal::rapidfuzz::batch_comparators::jaro_winkler::JaroWinkler;
use crate::simple::internal::rapidfuzz::batch_comparators::lcs_seq::LcsSeq;
use crate::simple::internal::rapidfuzz::batch_comparators::levenshtein::Levenshtein;
use crate::simple::internal::rapidfuzz::batch_comparators::osa::Osa;
use crate::simple::internal::rapidfuzz::batch_comparators::postfix::Postfix;
use crate::simple::internal::rapidfuzz::batch_comparators::prefix::Prefix;