mod indexable_keywords;
mod search;
mod search_and;
#[cfg(feature = "fuzzy")]
mod strsim;
pub(crate) mod top_scores;
pub(crate) mod string_keywords;

// -----------------------------------------------------------------------------

pub(crate) use crate::simple::internal::top_scores::LowestScores;
pub(crate) use crate::simple::internal::top_scores::TopScores;