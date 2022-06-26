mod indexable_keywords;
mod search;
mod search_and;
#[cfg(feature = "strsim")]
mod strsim;
pub(crate) mod top_scores;
pub(crate) mod string_keywords;

// -----------------------------------------------------------------------------

pub(crate) use crate::simple::internal::top_scores::TopScores;