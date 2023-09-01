mod indexable_keywords;
mod search;
mod search_and;
pub(crate) mod or_top_scores;
pub(crate) mod string_keywords;

#[cfg(feature = "fuzzy")]
mod strsim;
#[cfg(feature = "fuzzy")]
pub(crate) mod strsim_top_scores;

// -----------------------------------------------------------------------------

pub(crate) use crate::simple::internal::or_top_scores::OrTopScores;

#[cfg(feature = "fuzzy")]
pub(crate) use crate::simple::internal::strsim_top_scores::StrsimTopScores;