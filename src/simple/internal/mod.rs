mod indexable_keywords;
mod search;
mod search_and;
#[cfg(feature = "fuzzy")]
mod strsim;
pub(crate) mod string_keywords;
#[cfg(feature = "fuzzy")]
pub(crate) mod top_scores;

// -----------------------------------------------------------------------------

#[cfg(feature = "fuzzy")]
pub(crate) use crate::simple::internal::top_scores::TopScores;