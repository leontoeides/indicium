use crate::simple::search_index::SearchIndex;
use std::clone::Clone;
use std::cmp::{Eq, PartialEq};
use std::fmt::Debug;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Clone + Debug + Eq + Hash + PartialEq> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// An associated helper function that splits a `&str` into keywords using a
    /// split pattern (`Vec` of `char`). This function will also filter-out
    /// keywords that don't meet the defined length constraints.

    pub(crate) fn string_keywords<'a>(
        &self,
        string: &'a str,
        allow_string_as_keyword: bool,
    ) -> Vec<&'a str> {

        // Split the the field text / string into keywords:
        let mut keywords = if let Some(split_pattern) = &self.split_pattern {
            // Use the split pattern (`Vec` of `char`) to split the `String` into
            // keywords and filter the results:
            string
                // Split the `String` into smaller strings / keywords on
                // specified characters:
                .split(split_pattern.as_slice())
                // Iterate over each resulting keyword:
                .into_iter()
                // Only keep the keyword if it's longer than the minimum length
                // and shorter than the maximum length:
                .filter(|keyword| {
                    let chars = keyword.chars().count();
                    chars >= self.minimum_keyword_length && chars <= self.maximum_keyword_length
                }) // filter
                // Collect all keywords into a `Vec`:
                .collect()
        } else {
            // Split pattern was set to `None`, so do not split the `String`
            // into keywords. Return an empty `Vec` instead:
            vec![]
        };

        // If the option is enabled, store the field text / entire string itself
        // as a keyword for autocompletion purposes:
        if let Some(maximum_string_length) = self.maximum_string_length {
            // Only keep the string if it's shorter than the maximum:
            if allow_string_as_keyword && string.chars().count() <= maximum_string_length {
                // Add field text / entire string to the keyword `Vec`:
                keywords.push(string);
            } // if
        } // if

        // Return keywords to caller:
        keywords

    } // fn

} // impl