use crate::simple::search_index::SearchIndex;
use std::{cmp::Ord, hash::Hash};

// -----------------------------------------------------------------------------

impl<K: Hash + Ord> SearchIndex<K> {
    // -------------------------------------------------------------------------
    //
    /// Scans the entire search index for the closest matching keyword using
    /// the configured string similarity metric. Ilia Schelokov's
    /// [eddie](https://crates.io/crates/eddie) crate.
    ///
    /// When the user's search string contains a keyword that returns no
    /// matches, these `eddie_keyword_*` methods can be used to find the best
    /// match for substitution.
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
    /// let keyword_substitution = search_index.eddie_keyword("Harry");
    ///
    /// assert_eq!(
    ///     keyword_substitution,
    ///     Some("harold")
    /// );
    /// ```

    #[must_use]
    pub fn eddie_keyword(&self, keyword: &str) -> Option<&str> {
        // If case sensitivity set, leave case intact. Otherwise, normalize
        // keyword to lower case:
        let keyword = if self.case_sensitive {
            keyword.to_string()
        } else {
            keyword.to_lowercase()
        }; // if

        // Call global keyword subtitution provider:
        self.eddie_global_keyword(&keyword)
            .map(kstring::KStringBase::as_str)
    } // fn
} // impl
