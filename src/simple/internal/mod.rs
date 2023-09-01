mod indexable_keywords;
mod search;
mod search_and;
#[cfg(feature = "fuzzy")]
mod strsim;
pub(crate) mod string_keywords;
#[cfg(feature = "fuzzy")]
pub(crate) mod strsim_top_scores;

// -----------------------------------------------------------------------------

#[cfg(feature = "fuzzy")]
pub(crate) use crate::simple::internal::strsim_top_scores::StrsimTopScores as StrsimTopScores;