use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;

// -----------------------------------------------------------------------------

fn exclude_keyword(
    keyword: &str,
    exclude_keywords: &Option<Vec<String>>
) -> bool {

    // Check to see if there's any keywords in the exclusion list:
    if let Some(exclude_keywords) = exclude_keywords {
        // If there are keywords to be excluded, scan the list to see if this
        // keyword is in it. If so, filter it out (true = filter, false = keep):
        exclude_keywords
            .iter()
            .any(|excluded| excluded.as_str() == keyword)
    } else {
        // If there are no keywords to be excluded, always allow the keyword
        // (true = filter, false = keep):
        false
    } // if

} // fn

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

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

        // If case sensitivity set, leave case intact. Otherwise, normalize the
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
                    chars >= self.minimum_keyword_length
                        && chars <= self.maximum_keyword_length
                }) // filter
                // Only keep the keyword if it's not in the exclusion list:
                .filter(|keyword|
                    !exclude_keyword(keyword, &self.exclude_keywords)
                ) // filter
                // Copy string from reference:
                .map(String::from)
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
            // Only keep the string if 1) we're using whole strings as keywords,
            // 2) it's shorter than the maximum, and 3) the keyword is not in
            // the exclusion list:
            if  use_string_as_keyword &&
                string.chars().count() <= maximum_string_length &&
                !exclude_keyword(&string, &self.exclude_keywords) {
                // Add field text / entire string to the keyword `Vec`:
                keywords.push(string);
            } // if
        } // if

        // Sort keywords & remove duplicates:
        keywords.sort_unstable();
        keywords.dedup();

        // Return keywords to caller:
        keywords

    } // fn

} // impl