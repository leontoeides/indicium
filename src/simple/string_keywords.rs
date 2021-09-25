use crate::simple::search_index::SearchIndex;
use std::fmt::Debug;

// -----------------------------------------------------------------------------

impl<K: Debug> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// An associated helper method that splits a `&str` into keywords using a
    /// split pattern (`Vec<char>`). This method will also perform case
    /// conversion if necessary, and filter-out keywords that don't meet the
    /// defined length constraints.

    pub(crate) fn string_keywords(
        &self,
        string: &str,
        use_string_as_keyword: bool,
    ) -> Vec<String> {

        // If case sensitivity set, leave case intact. Otherwise, convert the
        // entire string to lower case:
        let string = match self.case_sensitive {
            true => string.to_string(),
            false => string.to_lowercase(),
        };

        // Split the the field text / string into keywords:
        let mut keywords: Vec<String> = if let Some(split_pattern) = &self.split_pattern {
            // Use the split pattern (a `Vec<char>`) to split the `String` into
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
                // Copy string from reference:
                .map(|str_ref| String::from(str_ref))
                // Collect all keywords into a `Vec`:
                .collect()
        } else {
            // Split pattern was set to `None`, so do not split the `String`
            // into keywords. Return an empty `Vec` instead:
            Vec::new()
        };

        // If the option is enabled, store the field text / entire string itself
        // as a keyword. This feature is for autocompletion purposes:
        if let Some(maximum_string_length) = self.maximum_string_length {
            // Only keep the string if it's shorter than the maximum:
            if use_string_as_keyword && string.chars().count() <= maximum_string_length {
                // Add field text / entire string to the keyword `Vec`:
                keywords.push(String::from(string));
            } // if
        } // if

        // Return keywords to caller:
        keywords

    } // fn

} // impl