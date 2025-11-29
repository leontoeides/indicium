#![allow(clippy::inline_always)]

use crate::simple::StrsimMetric;
use kstring::KString;
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
    /// When the user's last (partial) keyword that is meant to be autocompleted
    /// returns no matches, this can be used to find the best matches for
    /// substitution.
    ///
    /// All keywords in the search index will potentially be examined.
    ///
    /// # Input
    ///
    /// * `preceding_keywords` · If the search string was `He who knows nothing,
    ///   loves nothing` the “preceding” keywords would be `he who knows nothing
    ///   loves`.
    ///
    ///   This collection of keywords is used to prevent previously typed
    ///   keywords from being suggested. In this case, the system would _not_
    ///   suggest `nothing` as an autocomplete keyword since it's already
    ///   present in the search string.
    ///
    /// * `last_keyword` · If the search string was `He who knows nothing,
    ///   loves nothing` the “last” keyword would be `nothing`.
    ///
    ///   This keyword is used to search the search index. For example, this
    ///   could potentially return `nothingness`, and `nothings` as
    ///   autocompletion options if those words were present in the index.
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
    /// * This method expects the input to be normalized already, i.e. if the
    ///   search is meant to be case-insensitive then the inputs should be in
    ///   lowercase.
    ///
    /// * This method differs from `strsim_global_metric` in that this
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
    /// let keys: Vec<&usize> = search_index
    ///     .strsim_global(
    ///         vec!["william".into()].as_slice(),
    ///         "englund" // misspelling of "england"
    ///     )
    ///     .flat_map(|(keyword, key)| key)
    ///     .collect();
    ///
    /// assert_eq!(
    ///     keys,
    ///     vec![&0, &2]
    /// );
    /// ```
    #[inline(always)]
    pub(crate) fn strsim_global<'s>(
        &'s self,
        preceding_keywords: &[KString],
        last_keyword: &str,
    ) -> impl Iterator<Item = (&'s KString, &'s BTreeSet<K>)> {
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
        if let Some(index_range) = self.index_range(last_keyword) {
            match self.strsim_metric.as_ref() {
                Some(StrsimMetric::Jaro) => self
                    .strsim_global_metric::<Jaro>(
                        preceding_keywords,
                        last_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(StrsimMetric::JaroWinkler) => self
                    .strsim_global_metric::<JaroWinkler>(
                        preceding_keywords,
                        last_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(StrsimMetric::DamerauLevenshtein) => self
                    .strsim_global_metric::<DamerauLevenshtein>(
                        preceding_keywords,
                        last_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(StrsimMetric::Levenshtein) => self
                    .strsim_global_metric::<Levenshtein>(
                        preceding_keywords,
                        last_keyword,
                        &index_range,
                        &mut top_scores
                    ),

                Some(StrsimMetric::SorensenDice) => self
                    .strsim_global_metric::<SorensenDice>(
                        preceding_keywords,
                        last_keyword,
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
