#![allow(clippy::inline_always)]

use crate::simple::EddieMetric;
use eddie::str::{
    Levenshtein,
    DamerauLevenshtein,
    Jaro,
    JaroWinkler,
};
use std::hash::Hash;

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> crate::simple::search_index::SearchIndex<K> {
    /// Scans the entire search index for the closest matching keyword using
    /// the the specified string similarity metric from the
    /// [eddie](https://crates.io/crates/eddie) crate.
    ///
    /// When the user's search string contains a keyword that returns no
    /// matches, this method can be used to find the best match for
    /// substitution.
    ///
    /// # Input
    ///
    /// * `keyword` · Keywords most similar to this specified user keyword
    ///   will be returned.
    ///
    /// # Output
    ///
    /// * This method will return `None` if no keywords could be found. Settings
    ///   such as `fuzzy_length` and `fuzzy_minimum_score` can affect the
    ///   outcome.
    ///
    /// # Notes
    ///
    /// * `global` means that all keywords in the search index will potentially
    ///   be examined.
    ///
    /// * This method differs from `eddie_keyword_global_comparator` in that
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
    /// let keyword_substitution = search_index.eddie_keyword_global("Harry");
    ///
    /// assert_eq!(
    ///     keyword_substitution,
    ///     Some("harold")
    /// );
    /// ```
    #[must_use]
    #[inline(always)]
    pub(crate) fn eddie_keyword_global(
        &self,
        keyword: &str
    ) -> Option<&str> {
        // If the search index is set to be case insensitive, normalize the
        // keyword to lower-case:
        let user_keyword = self.normalize(keyword);

        // Build an keyword index range to fuzzy match against. This is used to
        // restrict fuzzy-matching to the strings that match the first _n_
        // characters in the user's keyword. This helps reduce required compute.
        // If a `None` is returned then no fuzzy-matching should be performed:
        let index_range = self.index_range(&user_keyword)?;

        // If no string similarity metric was defined in the search index, fuzzy
        // string matching is effectively turned off. Return a `None` to the
        // caller:
        let eddie_metric = self.eddie_metric.as_ref()?;

        // Attempt to find the closest match to the user's keyword. We'll use
        // the selected string similarity metric defined in the search index:
        let global_keyword: Option<&kstring::KString> = match eddie_metric {
            EddieMetric::Levenshtein =>
                self.eddie_keyword_global_comparator::<Levenshtein>(
                    &index_range,
                    &user_keyword
                ),

            EddieMetric::DamerauLevenshtein =>
                self.eddie_keyword_global_comparator::<DamerauLevenshtein>(
                    &index_range,
                    &user_keyword
                ),

            EddieMetric::Jaro =>
                self.eddie_keyword_global_comparator::<Jaro>(
                    &index_range,
                    &user_keyword
                ),

            EddieMetric::JaroWinkler =>
                self.eddie_keyword_global_comparator::<JaroWinkler>(
                    &index_range,
                    &user_keyword
                ),
        }; // match

        // Call global keyword subtitution provider:
        global_keyword.map(kstring::KStringBase::as_str)
    } // fn
} // impl
