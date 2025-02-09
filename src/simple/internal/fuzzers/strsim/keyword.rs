#![allow(clippy::inline_always)]

use crate::simple::StrsimMetric;
use std::collections::BTreeSet;
use crate::simple::internal::fuzzers::strsim::{
    Jaro,
    JaroWinkler,
    DamerauLevenshtein,
    Levenshtein,
    SorensenDice,
};

// -----------------------------------------------------------------------------

impl<K: std::hash::Hash + Ord> crate::simple::search_index::SearchIndex<K> {
    /// Scans the entire search index for the closest matching _n_ keywords
    /// using the provided keyword and configured string similarity metric. This
    /// feature relies on the [strsim](https://crates.io/crates/strsim)
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
    /// * This method differs from `strsim_keyword_metric` in that this
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
    ///     .strsim_keyword("harry")
    ///     .map(|(keyword, _key)| keyword.as_str())
    ///     .collect();
    ///
    /// assert_eq!(
    ///     keywords,
    ///     vec!["harold"]
    /// );
    /// ```
    #[inline(always)]
    pub(crate) fn strsim_keyword<'s>(
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
        match self.index_range(user_keyword) {
            // Attempt to find the top matches for the user's (partial) keyword
            // using the selected string similarity metric defined in the
            // `SearchIndex`:
            Some(index_range) => match self.strsim_metric.as_ref() {
                Some(StrsimMetric::Jaro) => self
                    .strsim_keyword_metric::<Jaro>(
                        user_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(StrsimMetric::JaroWinkler) => self
                    .strsim_keyword_metric::<JaroWinkler>(
                        user_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(StrsimMetric::DamerauLevenshtein) => self
                    .strsim_keyword_metric::<DamerauLevenshtein>(
                        user_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(StrsimMetric::Levenshtein) => self
                    .strsim_keyword_metric::<Levenshtein>(
                        user_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(StrsimMetric::SorensenDice) => self
                    .strsim_keyword_metric::<SorensenDice>(
                        user_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                // If no string similarity metric was defined in the search
                // index, fuzzy string matching is effectively turned off.
                None => { /* Do nothing */ },
            }, // Some(index_range)

            // If a `None` is returned by `index_range` then no fuzzy-matching
            // should be performed.
            None => { /* Do nothing */ },
        }; // match

        // Return the top scoring keywords that could be used as autocomplete
        // options, and their keys, to the caller:
        top_scores.results()
    } // fn
} // impl
