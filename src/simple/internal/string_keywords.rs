use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;

// -----------------------------------------------------------------------------
//
/// When a string is passed to the `string_keywords` function, the intended use
/// for the keywords changes how the keywords are split & processed. The results
/// of splitting a string for `Indexing` may differ from splitting a string for
/// `Searching`. (In particular when no split-pattern has been defined.)

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum SplitContext {
    /// The intended use for split keywords is for indexing:
    Indexing,
    /// The intended use for split keywords is to be used for searching or
    /// autocompletion:
    Searching,
}

// -----------------------------------------------------------------------------
//
/// Function will check if the provided keyword is in the list of excluded
/// keywords. If it is, function will return `true`. If there are no excluded
/// keywords, function will always return `false`.

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
    /// defined length restrictions.

    pub(crate) fn string_keywords(
        &self,
        string: &str,
        context: SplitContext,
    ) -> Vec<String> {

        // If case sensitivity set, leave case intact. Otherwise, normalize the
        // entire string to lower case:
        let string = match self.case_sensitive {
            true => string.to_string(),
            false => string.to_lowercase(),
        };

        // Split the the string into keywords:
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

        // Using the whole string as a keyword:
        //
        // * For searching: return the whole string as the search keyword if
        // no split pattern is defined (keyword splitting is turned off).
        //
        // * For indexing: if the option is enabled, store the field text /
        // entire string itself as a keyword. This feature is primarily for
        // autocompletion purposes.

        let chars = string.chars().count();

        // If we're searching, keep the whole string if there is no split
        // pattern defined. We'll search by the whole search string without
        // any keyword splitting:
        if  context == SplitContext::Searching &&
            self.split_pattern == None &&
            chars >= self.minimum_keyword_length {
                keywords = vec![string]
        // If we're indexing, only keep the whole string if it meets the keyword
        // criteria: 1) we're using whole strings as keywords, 2) it's shorter
        // than the maximum, and 3) the keyword is not in the exclusion list.
        } else if let Some(maximum_string_length) = self.maximum_string_length {
            if  context == SplitContext::Indexing &&
                chars >= self.minimum_keyword_length &&
                chars <= maximum_string_length &&
                !exclude_keyword(&string, &self.exclude_keywords) {
                    // Add field text / entire string to the keyword `Vec`:
                    keywords.push(string)
            } // if
        } // if

        // Return keywords to caller:
        keywords

    } // fn

} // impl