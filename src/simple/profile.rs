use crate::simple::search_index::SearchIndex;
use std::clone::Clone;
use std::cmp::Ord;

// -----------------------------------------------------------------------------

impl<K: Clone + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// **This method is only available in debug builds.**
    ///
    /// Extremely high repetitions of single keywords can reduce the performance
    /// of the search index. `profile` allows you to see the most repeated
    /// keywords in your search index. Using the output from this method, you
    /// can manually check for keywords that have high repetitions and that add
    /// little value (such as conjuctions, articles, and prepositions) and put
    /// them into your keyword exclusion list.
    ///
    /// See also: the [`exclude_keywords`] method for the builder pattern.
    ///
    /// [`exclude_keywords`]: struct.SearchIndexBuilder.html#method.exclude_keywords

    #[tracing::instrument(level = "trace", name = "search index profile", skip(self))]
    pub fn profile(&self, count: usize) -> impl Iterator<Item = (&String, usize)> {

        // Get a list of all keywords and the number of attached keys for each
        // keyword. For example: keyword "supercalifragilisticexpialidocious"
        // has 28 keys (or records) attached to it:
        let mut keywords: Vec<(&String, usize)> = self.b_tree_map
            // Iterate over every entry (representing a keyword) in the search
            // index:
            .iter()
            // Map `(String, BTreeSet<K>)` to `(String, usize)` by getting the
            // length of the `BTreeSet`.
            .map(|(key, value)| (key, value.len()))
            // Collect the keyword and key count into a `Vec`:
            .collect();

        // Sort keywords by number of attached keys (i.e. associated records),
        // in descending order:
        keywords.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Return only `count` number of records to the caller:
        keywords
            .into_iter()
            .take(count)

    } // fn

} // impl