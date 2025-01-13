use crate::simple::search_index::SearchIndex;
use kstring::KString;

// -----------------------------------------------------------------------------
//
/// When a string is passed to the `string_keywords` function, the intended use
/// for the keywords changes how the keywords are split & processed. The results
/// of splitting a string for `Indexing` may differ from splitting a string for
/// `Searching`. (In particular when no split-pattern has been defined.)

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SplitContext {
    /// The intended use for split keywords is for indexing:
    Indexing = 0,
    /// The intended use for split keywords is to be used for searching or
    /// autocompletion:
    Searching = 1,
}

// -----------------------------------------------------------------------------
//
/// Function will check if the provided keyword is in the list of excluded
/// keywords. If it is, function will return `true`. If there are no excluded
/// keywords, function will always return `false`.

pub fn exclude_keyword(keyword: &str, exclude_keywords: Option<&Vec<KString>>) -> bool {
    // Check to see if there's any keywords in the exclusion list:
    exclude_keywords.as_ref().is_some_and(|exclude_keywords| {
        exclude_keywords
            .iter()
            .any(|excluded| excluded.as_str() == keyword)
    }) // is_some_and
} // fn

// -----------------------------------------------------------------------------

#[test]
fn test_exclude_keyword() {
    let excluded_keywords: Option<Vec<KString>> = Some(vec![
        "awake".into(),
        "arise".into(),
        "or".into(),
        "be".into(),
        "for".into(),
        "ever".into(),
        "fall’n".into(),
    ]); // vec!

    assert!(exclude_keyword("arise", excluded_keywords.as_ref()));

    assert!(!exclude_keyword("arose", excluded_keywords.as_ref()));
}

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {
    /// An associated helper method that splits a `&str` into keywords using a
    /// split pattern (`Vec<char>`).
    ///
    /// This method will also perform case conversion if necessary, filter-out
    /// keywords that don't meet the defined length restrictions, and remove
    /// excluded keywords.
    pub(crate) fn string_keywords(
        &self,
        string: &str,
        context: &SplitContext
    ) -> Vec<KString> {
        // If case sensitivity set, leave case intact. Otherwise, normalize the
        // entire string to lower case:
        let string = KString::from(self.normalize(string).into_owned());

        // Split the the string into keywords:
        let mut keywords: Vec<KString> =
            self.split_pattern
                .as_ref()
                .map_or_else(Vec::new, |split_pattern| {
                    string
                        // Split the `KString` into smaller strings / keywords on
                        // specified characters:
                        .split(split_pattern.as_slice())
                        // Only keep the keyword if it's longer than the minimum length
                        // and shorter than the maximum length:
                        .filter(|keyword| {
                            let chars = keyword.chars().count();
                            chars >= self.minimum_keyword_length
                                && chars <= self.maximum_keyword_length
                        }) // filter
                        // Only keep the keyword if it's not in the exclusion list:
                        .filter(|keyword| !exclude_keyword(keyword, self.exclude_keywords.as_ref())) // filter
                        // Copy string from reference:
                        .map(KString::from_ref)
                        // Collect all keywords into a `Vec`:
                        .collect()
                }); // map_or_else

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
        if context == &SplitContext::Searching
            && self.split_pattern.is_none()
            && chars >= self.minimum_keyword_length
        {
            // Set keywords to the entire string:
            keywords = vec![string];

        // If we're indexing, only keep the whole string if it meets the keyword
        // criteria: 1) we're using whole strings as keywords, 2) it's shorter
        // than the maximum, and 3) the keyword is not in the exclusion list.
        } else if let Some(maximum_string_length) = self.maximum_string_length {
            if context == &SplitContext::Indexing
                && chars >= self.minimum_keyword_length
                && chars <= maximum_string_length
                && !exclude_keyword(&string, self.exclude_keywords.as_ref())
            {
                // Add field text / entire string to the keyword `Vec`:
                keywords.push(string);
            } // if
        } // if

        // Return keywords to caller:
        keywords
    } // fn
} // impl
