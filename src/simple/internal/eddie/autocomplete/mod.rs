//! Fuzzy matching for autocompletion.

pub(crate) mod context_damerau_levenshtein;
pub(crate) mod context_jaro;
pub(crate) mod context_jaro_winkler;
pub(crate) mod context_levenshtein;
pub(crate) mod global_damerau_levenshtein;
pub(crate) mod global_jaro;
pub(crate) mod global_jaro_winkler;
pub(crate) mod global_levenshtein;