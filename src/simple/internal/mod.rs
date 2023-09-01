mod indexable_keywords;
mod search;
mod search_and;
pub(crate) mod search_top_scores;
pub(crate) mod string_keywords;

#[cfg(feature = "fuzzy")]
mod strsim;
#[cfg(feature = "fuzzy")]
pub(crate) mod strsim_top_scores;

// -----------------------------------------------------------------------------

pub(crate) use crate::simple::internal::search_top_scores::SearchTopScores;

#[cfg(feature = "fuzzy")]
pub(crate) use crate::simple::internal::strsim_top_scores::StrsimTopScores;