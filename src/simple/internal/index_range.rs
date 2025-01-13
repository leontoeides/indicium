impl<K: Ord> crate::simple::SearchIndex<K> {
    /// Returns the index range for the provided keyword by using the
    /// `fuzzy_length` in the search index settings.
    ///
    /// # What is an index range?
    ///
    /// For fuzzy matching, Indicium can optionally require that the first _n_
    /// characters of the user's keyword match the keyword in the index.
    ///
    /// This is accomplished by using this `index_range` as a b-trie map `range`
    /// parameter, and using it as a `take_while` predicate.
    ///
    /// This can significantly reduce the computational power needed to find
    /// good fuzzy-matched alternatives to the user's keyword.
    ///
    /// The `index_range` is a stub that is used to ensure that the first _n_
    /// characters match. For example, if the string
    /// “[finglonger](https://futurama.fandom.com/wiki/Fing-Longer)” is passed,
    /// and the `fuzzy_length` was set to `4`, this method will return `fing`.
    ///
    /// This returned `fing` string is, in turn, used to ensure that the
    /// keywords in the search index start with `fing` before attempting to
    /// perform any string similarity or distance calculations on them.
    ///
    /// # Examples
    ///
    /// The index range is shown in the `Index Keyword Must Start With...`
    /// column in the below table.
    ///
    /// | Example | User Keyword                       | Length | Index Keyword Must Start With... |
    /// |---------|------------------------------------|--------|----------------------------------|
    /// | 1       | Supercalifragilisticexpialidocious | 2      | Su                               |
    /// | 2       | Antidisestablishmentarianism       | 4      | Anti                             |
    /// | 3       | Pseudopseudohypoparathyroidism     | 0      |                                  |
    ///
    /// * In example 1, since the length is set to `2`, the user's keyword will
    ///   only be fuzzy matched against keywords in the index beginning with
    ///   `su`.
    ///
    /// * In example 2, since the length is set to `4`, the user's keyword will
    ///   only be fuzzy matched against keywords in the index beginning with
    ///   `anti`.
    ///
    /// * In example 3, since the length is set to `0`, the user's keyword will
    ///   be fuzzy matched against every keyword in the index. This is OK (or
    ///   even desirable) if the search index isn't large, however, this will be
    ///   crippling slow on very large search indicies.
    pub(crate) fn index_range<'k>(
        &self,
        user_keyword: &'k str
    ) -> Option<beef::lean::Cow<'k, str>> {
        if self.fuzzy_length > 0 {
            // The user keyword must be longer than the match length to be
            // evaluated for fuzzy-matches:
            if user_keyword.len() >= self.fuzzy_length {
                // Get the byte index of the _n_th character:
                let byte_index: Option<usize> = user_keyword
                    .char_indices()
                    .take(self.fuzzy_length)
                    .map(|(idx, _ch)| idx)
                    .max();

                // Use the first _n_ characters of the user's keyword. These
                // first characters are used to find search index keywords to
                // fuzzy match against:
                byte_index.map(|byte_index| (&user_keyword[0..byte_index]).into())
            } else {
                // The user's keyword is too short. Do not perform any fuzzy
                // matching:
                None
            } // if
        } else {
            // The fuzzy length is 0, compare user's keyword against all search
            // index keywords:
            Some("".into())
        }
    } // fn
} // impl