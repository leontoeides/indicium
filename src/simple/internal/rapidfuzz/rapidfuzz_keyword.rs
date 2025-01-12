use crate::simple::internal::rapidfuzz::{
    DamerauLevenshtein,
    Hamming,
    Indel,
    Jaro,
    JaroWinkler,
    LcsSeq,
    Levenshtein,
    Osa,
    Postfix,
    Prefix,
};
use crate::simple::RapidfuzzMetric;
use kstring::KString;
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> crate::simple::search_index::SearchIndex<K> {
    /// Scans the entire search index for the closest matching keyword using
    /// the configured string similarity metric. This feature relies on the
    /// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate.
    ///
    /// When the user's search string contains a keyword that returns no
    /// matches, these `rapidfuzz_keyword_*` methods can be used to find the
    /// best match for substitution.
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use indicium::simple::{AutocompleteType, Indexable, SearchIndex, SearchType};
    /// # use pretty_assertions::assert_eq;
    /// #
    /// # struct MyStruct {
    /// #   title: String,
    /// #   year: u16,
    /// #   body: String,
    /// # }
    /// #
    /// # impl Indexable for MyStruct {
    /// #   fn strings(&self) -> Vec<String> {
    /// #       vec![
    /// #           self.title.clone(),
    /// #           self.year.to_string(),
    /// #           self.body.clone(),
    /// #       ]
    /// #   }
    /// # }
    /// #
    /// # let my_vec = vec![
    /// #   MyStruct {
    /// #       title: "Harold Godwinson".to_string(),
    /// #       year: 1066,
    /// #       body: "Last crowned Anglo-Saxon king of England.".to_string(),
    /// #   },
    /// #   MyStruct {
    /// #       title: "Edgar Ætheling".to_string(),
    /// #       year: 1066,
    /// #       body: "Last male member of the royal house of Cerdic of Wessex.".to_string(),
    /// #   },
    /// #   MyStruct {
    /// #       title: "William the Conqueror".to_string(),
    /// #       year: 1066,
    /// #       body: "First Norman monarch of England.".to_string(),
    /// #   },
    /// #   MyStruct {
    /// #       title: "William Rufus".to_string(),
    /// #       year: 1087,
    /// #       body: "Third son of William the Conqueror.".to_string(),
    /// #   },
    /// #   MyStruct {
    /// #       title: "Henry Beauclerc".to_string(),
    /// #       year: 1100,
    /// #       body: "Fourth son of William the Conqueror.".to_string(),
    /// #   },
    /// # ];
    /// #
    /// # let mut search_index: SearchIndex<usize> = SearchIndex::default();
    /// #
    /// # my_vec
    /// #   .iter()
    /// #   .enumerate()
    /// #   .for_each(|(index, element)|
    /// #       search_index.insert(&index, element)
    /// #   );
    /// #
    /// #
    /// let keyword_substitution = search_index.rapidfuzz_global_keyword("Harry");
    ///
    /// assert_eq!(
    ///     keyword_substitution,
    ///     Some("harold")
    /// );
    /// ```
    #[must_use]
    pub fn rapidfuzz_global_keyword(&self, keyword: &str) -> Option<&str> {
        // If case sensitivity set, leave case intact. Otherwise, normalize
        // keyword to lower case:
        let user_keyword = if self.case_sensitive {
            keyword.to_string()
        } else {
            keyword.to_lowercase()
        }; // if

        // Build an index keyword range to fuzzy match against.
        //
        // | Example | User Keyword                       | Length | Index Keyword Must Start With... |
        // |---------|------------------------------------|--------|----------------------------------|
        // | 1       | Supercalifragilisticexpialidocious | 2      | Su                               |
        // | 2       | Antidisestablishmentarianism       | 4      | Anti                             |
        // | 3       | Pseudopseudohypoparathyroidism     | 0      |                                  |
        //
        // * In example 1, since the length is set to `2`, the user's keyword
        // will only be fuzzy matched against keywords in the index beginning
        // with `su`.
        //
        // * In example 2, since the length is set to `4`, the user's keyword
        // will only be fuzzy matched against keywords in the index beginning
        // with `anti`.
        //
        // * In example 3, since the length is set to `0`, the user's keyword
        // will be fuzzy matched against every keyword in the index. This is OK
        // (or even desirable) if the search index isn't large, however, this
        // will be crippling slow on very large search indicies.
        let index_range: &str = if self.fuzzy_length > 0 {
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
                match byte_index {
                    Some(byte_index) => &user_keyword[0..byte_index],
                    None => return None,
                } // match
            } else {
                // The user's keyword is too short. Do not perform any fuzzy
                // matching:
                return None;
            } // if
        } else {
            // The match length is 0, compare user's keyword against all search
            // index keywords:
            ""
        }; // if

        // Attempt to find the closest match for the user's keyword using the
        // selected string similarity metric defined in the `SearchIndex`:
        #[allow(clippy::option_if_let_else)] // I hate this lint
        let global_keyword: Option<&KString> =
            if let Some(rapidfuzz_metric) = &self.rapidfuzz_metric {
                match rapidfuzz_metric {
                    RapidfuzzMetric::DamerauLevenshtein =>
                        self.rapidfuzz_keyword_global_generic::<DamerauLevenshtein>(index_range, &user_keyword),

                    RapidfuzzMetric::Hamming =>
                        self.rapidfuzz_keyword_global_generic::<Hamming>(index_range, &user_keyword),

                    RapidfuzzMetric::Indel =>
                        self.rapidfuzz_keyword_global_generic::<Indel>(index_range, &user_keyword),

                    RapidfuzzMetric::Jaro =>
                        self.rapidfuzz_keyword_global_generic::<Jaro>(index_range, &user_keyword),

                    RapidfuzzMetric::JaroWinkler =>
                        self.rapidfuzz_keyword_global_generic::<JaroWinkler>(index_range, &user_keyword),

                    RapidfuzzMetric::LcsSeq =>
                        self.rapidfuzz_keyword_global_generic::<LcsSeq>(index_range, &user_keyword),

                    RapidfuzzMetric::Levenshtein =>
                        self.rapidfuzz_keyword_global_generic::<Levenshtein>(index_range, &user_keyword),

                    RapidfuzzMetric::Osa =>
                        self.rapidfuzz_keyword_global_generic::<Osa>(index_range, &user_keyword),

                    RapidfuzzMetric::Postfix =>
                        self.rapidfuzz_keyword_global_generic::<Postfix>(index_range, &user_keyword),

                    RapidfuzzMetric::Prefix =>
                        self.rapidfuzz_keyword_global_generic::<Prefix>(index_range, &user_keyword),
                } // match
            } else {
                // No string similarity metric was defined in the `SearchIndex`
                // settings. Fuzzy string matching effectively turned off.
                // Return a `None` to the caller:
                None
            }; // if

        // Call global keyword subtitution provider:
        global_keyword.map(kstring::KStringBase::as_str)
    } // fn
} // impl
