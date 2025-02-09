#![allow(clippy::inline_always)]

use crate::simple::internal::fuzzers::rapidfuzz::{
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

// -----------------------------------------------------------------------------

impl<K: std::hash::Hash + Ord> crate::simple::search_index::SearchIndex<K> {
    /// Scans the entire search index for the closest matching keyword using
    /// the the specified string similarity metric from the
    /// [rapidfuzz](https://crates.io/crates/rapidfuzz) crate.
    ///
    /// When the user's search string contains a keyword that returns no
    /// matches, this method can be used to find the best match to be used as a
    /// substitution.
    ///
    /// All keywords in the search index will potentially be examined.
    ///
    /// # Input
    ///
    /// * `user_keyword` · This keyword is used to search the search index.
    ///
    ///   For example, if the user provided the misspelled word `nthing`, this
    ///   could potentially return `nothing` as an alternative keyword if it
    ///   was present in the index.
    ///
    ///   Note that this method expects the input to be normalized already, i.e.
    ///   if the search is meant to be case-insensitive then the input should be
    ///   in lowercase.
    ///
    /// # Output
    ///
    /// This method returns the single best matching alternative keyword.
    ///
    /// If no reasonable alternative keywords were found, a `None` will be
    /// returned.
    ///
    /// # Notes
    ///
    /// * This method differs from `rapidfuzz_substitute_comparator` in that
    ///   this method will perform some common setup, and dynamically dispatch
    ///   to the generic method indicated by the chosen string similarity metric
    ///   (`DamerauLevenshtein`, `Jaro`, `Osa`, etc.)
    ///
    /// # Basic Usage
    ///
    /// ```ignore
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
    /// let keyword_substitution = search_index.rapidfuzz_substitute("Harry");
    ///
    /// assert_eq!(
    ///     keyword_substitution,
    ///     Some("harold")
    /// );
    /// ```
    #[must_use]
    #[inline(always)]
    #[allow(clippy::option_if_let_else)]
    pub(crate) fn rapidfuzz_substitute(
        &self,
        user_keyword: &str
    ) -> Option<&str> {
        // If the search index is set to be case insensitive, normalize the
        // keyword to lower-case:
        let user_keyword = self.normalize(user_keyword);

        // This call to `index_range` builds a keyword index range to fuzzy
        // match against.
        //
        // This is used to restrict fuzzy-matching to only strings that match
        // the first _n_ characters in the user's keyword. This helps reduce
        // required compute.
        //
        // For example, if the `index_range` is "super" and the user's keyword
        // is "supersonic", only index keywords beginning with "super" will be
        // fuzzy compared against the user's keyword: "supersonic" against
        // "superalloy", "supersonic" against "supergiant" and so on...
        let best_match: Option<&str> = match self.index_range(&user_keyword) {
            // Attempt to find the closest match to the user's keyword. We'll
            // use the selected string similarity metric defined in the search
            // index:
            Some(index_range) => match self.rapidfuzz_metric.as_ref() {
                Some(RapidfuzzMetric::DamerauLevenshtein) =>
                    self.rapidfuzz_substitute_comparator::<DamerauLevenshtein>(
                        &user_keyword,
                        &index_range,
                    ),

                Some(RapidfuzzMetric::Hamming) =>
                    self.rapidfuzz_substitute_comparator::<Hamming>(
                        &user_keyword,
                        &index_range,
                    ),

                Some(RapidfuzzMetric::Indel) =>
                    self.rapidfuzz_substitute_comparator::<Indel>(
                        &user_keyword,
                        &index_range,
                    ),

                Some(RapidfuzzMetric::Jaro) =>
                    self.rapidfuzz_substitute_comparator::<Jaro>(
                        &user_keyword,
                        &index_range,
                    ),

                Some(RapidfuzzMetric::JaroWinkler) =>
                    self.rapidfuzz_substitute_comparator::<JaroWinkler>(
                        &user_keyword,
                        &index_range,
                    ),

                Some(RapidfuzzMetric::LcsSeq) =>
                    self.rapidfuzz_substitute_comparator::<LcsSeq>(
                        &user_keyword,
                        &index_range,
                    ),

                Some(RapidfuzzMetric::Levenshtein) =>
                    self.rapidfuzz_substitute_comparator::<Levenshtein>(
                        &user_keyword,
                        &index_range,
                    ),

                Some(RapidfuzzMetric::Osa) =>
                    self.rapidfuzz_substitute_comparator::<Osa>(
                        &user_keyword,
                        &index_range,
                    ),

                Some(RapidfuzzMetric::Postfix) =>
                    self.rapidfuzz_substitute_comparator::<Postfix>(
                        &user_keyword,
                        &index_range,
                    ),

                Some(RapidfuzzMetric::Prefix) =>
                    self.rapidfuzz_substitute_comparator::<Prefix>(
                        &user_keyword,
                        &index_range,
                    ),

                // If no string similarity metric was defined in the search
                // index, fuzzy string matching is effectively turned off.
                None => None,
            }, // Some(index_range)

            // If a `None` is returned by `index_range` then no fuzzy-matching
            // should be performed.
            None => None,
        }; // match

        // Return the closest matching keyword to the caller:
        best_match
    } // fn
} // impl
