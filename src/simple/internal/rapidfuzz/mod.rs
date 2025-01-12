//! Integration with the [rapidfuzz](https://crates.io/crates/rapidfuzz)
//! string similarity crate.

mod batch_comparator;

use crate::simple::internal::rapidfuzz::batch_comparator::BatchComparator;

// -----------------------------------------------------------------------------

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

// -----------------------------------------------------------------------------

pub mod autocomplete_context_generic;
pub mod autocomplete_global_generic;
pub mod keyword_global_generic;
pub mod rapidfuzz_autocomplete;
pub mod rapidfuzz_context_autocomplete;
pub mod rapidfuzz_global_autocomplete;
pub mod rapidfuzz_keyword;