mod and_search;
mod autocomplete;
mod indexable_keywords;
mod search;
mod string_keywords;

// -----------------------------------------------------------------------------

const MAXIMUM_INTERNAL_AUTOCOMPLETE_RESULTS: usize = 40_960;
const MAXIMUM_INTERNAL_SEARCH_RESULTS: usize = 40_960;