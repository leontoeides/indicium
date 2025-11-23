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
use std::collections::BTreeSet;

// -----------------------------------------------------------------------------

impl<K: std::hash::Hash + Ord> crate::simple::search_index::SearchIndex<K> {
    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the provided keyword and configured string similarity metric. This
    /// feature relies on the [rapidfuzz](https://crates.io/crates/rapidfuzz)
    /// crate.
    ///
    /// When the user's keyword that is meant to be autocompleted returns no
    /// matches, this can be used to find the best matches for substitution.
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
    /// This method returns an iterator over the top _n_ autocompletion options.
    ///
    /// Each item the returned iterator is comprised of a keyword, and the
    /// records associated with each keyword.
    ///
    /// The number of autocompletion options are defined by the
    /// `maximum_autocomplete_options` option in the search index.
    ///
    /// If no keywords or reasonable matches are found, this method will return
    /// an empty iterator.
    ///
    /// # Notes
    ///
    /// * This method differs from `rapidfuzz_keyword_comparator` in that this
    ///   method will perform some common setup, and dynamically dispatch to the
    ///   generic method indicated by the chosen string similarity metric
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
    /// // Note: This method expects the input to be normalized (i.e. in
    /// // lowercase) already.
    ///
    /// let keywords: Vec<&str> = search_index
    ///     .rapidfuzz_keyword("harry")
    ///     .map(|(keyword, _key)| keyword.as_str())
    ///     .collect();
    ///
    /// assert_eq!(
    ///     keywords,
    ///     vec!["harold"]
    /// );
    /// ```
    #[inline(always)]
    pub(crate) fn rapidfuzz_keyword<'s>(
        &'s self,
        user_keyword: &str,
    ) -> impl Iterator<Item = (&'s kstring::KString, &'s BTreeSet<K>)> {
        // This structure will track the top scoring keywords:
        let mut top_scores =
            crate::simple::internal::fuzzers::FuzzyTopScores::<K, f64>::with_capacity(
                self.maximum_autocomplete_options
            );

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
        if let Some(index_range) = self.index_range(user_keyword) {
            match self.rapidfuzz_metric.as_ref() {
                Some(RapidfuzzMetric::DamerauLevenshtein) => self
                    .rapidfuzz_keyword_comparator::<DamerauLevenshtein>(
                        user_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(RapidfuzzMetric::Hamming) => self
                    .rapidfuzz_keyword_comparator::<Hamming>(
                        user_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(RapidfuzzMetric::Indel) => self
                    .rapidfuzz_keyword_comparator::<Indel>(
                        user_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(RapidfuzzMetric::Jaro) => self
                    .rapidfuzz_keyword_comparator::<Jaro>(
                        user_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(RapidfuzzMetric::JaroWinkler) => self
                    .rapidfuzz_keyword_comparator::<JaroWinkler>(
                        user_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(RapidfuzzMetric::LcsSeq) => self
                    .rapidfuzz_keyword_comparator::<LcsSeq>(
                        user_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(RapidfuzzMetric::Levenshtein) => self
                    .rapidfuzz_keyword_comparator::<Levenshtein>(
                        user_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(RapidfuzzMetric::Osa) => self
                    .rapidfuzz_keyword_comparator::<Osa>(
                        user_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(RapidfuzzMetric::Postfix) => self
                    .rapidfuzz_keyword_comparator::<Postfix>(
                        user_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(RapidfuzzMetric::Prefix) => self
                    .rapidfuzz_keyword_comparator::<Prefix>(
                        user_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                // If no string similarity metric was defined in the search
                // index, fuzzy string matching is effectively turned off.
                None => { /* Do nothing */ },
            } // match
        } // if

        // Return the top scoring keywords that could be used as autocomplete
        // options, and their keys, to the caller:
        top_scores.results()
    } // fn
} // impl
