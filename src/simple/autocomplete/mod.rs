mod context;
mod global;
mod keyword;

// -----------------------------------------------------------------------------

use crate::simple::{AutocompleteType, SearchIndex};
use std::cmp::Ord;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Returns matching autocompleted keywords for the provided search string.
    /// This function will use the `AutocompleteType` setting stored in the
    /// `SearchIndex`. Partial keywords must be an exact match. Results are
    /// returned in lexographic order.
    ///
    /// Autocompletion behaviour can be changed by setting the
    /// [`AutocompleteType`] in the `SearchIndex`. See also:
    /// [`SearchIndexBuilder`] and [`SearchIndex::new()`].
    ///
    /// [`AutocompleteType`]: enum.AutocompleteType.html
    /// [`SearchIndexBuilder`]: struct.SearchIndexBuilder.html
    /// [`SearchIndex::new()`]: struct.SearchIndex.html#method.new
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use indicium::simple::{AutocompleteType, Indexable, SearchIndex, SearchType};
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
    /// let autocomplete_options = search_index.autocomplete("Edgar last c");
    /// assert_eq!(autocomplete_options, vec!["edgar last cerdic".to_string()]);
    /// ```

    #[tracing::instrument(level = "trace", name = "Autocomplete", skip(self))]
    pub fn autocomplete(&self, string: &str) -> Vec<String> {

        let autocomplete_options: Vec<String> = match &self.autocomplete_type {
            AutocompleteType::Context => self.autocomplete_context(string),
            AutocompleteType::Global => self.autocomplete_global(string),
            AutocompleteType::Keyword => self.autocomplete_keyword(string).iter().cloned().cloned().collect(),
        }; // match

        // For debug builds:
        #[cfg(debug_assertions)]
        tracing::trace!(
            "{} autocomplete options for \"{}\".",
            autocomplete_options.len(),
            string,
        ); // trace!

        autocomplete_options

    } // fn

    // -------------------------------------------------------------------------
    //
    /// Returns matching autocompleted keywords for the provided search string.
    /// Partial keywords must be an exact match. Results are returned in
    /// lexographic order.
    ///
    /// Autocomplete behaviour can be changed by using different
    /// `AutocompleteType` variants as the first parameter for the method call.
    /// See [`AutocompleteType`] for more information on the different
    /// autocomplete types.
    ///
    /// [`AutocompleteType`]: enum.AutocompleteType.html
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// # use indicium::simple::{AutocompleteType, Indexable, SearchIndex, SearchType};
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
    /// let autocomplete_options =
    ///     search_index.autocomplete_type(&AutocompleteType::Global, "1100 e");
    ///
    /// assert_eq!(
    ///     autocomplete_options,
    ///     vec![
    ///         "1100 edgar".to_string(),
    ///         "1100 edgar ætheling".to_string(),
    ///         "1100 england".to_string()
    ///     ]
    /// );
    /// ```

    #[tracing::instrument(level = "trace", name = "Autocomplete", skip(self))]
    pub fn autocomplete_type(&self, autocomplete_type: &AutocompleteType, string: &str) -> Vec<String> {

        let autocomplete_options: Vec<String> = match autocomplete_type {
            AutocompleteType::Context => self.autocomplete_context(string),
            AutocompleteType::Global => self.autocomplete_global(string),
            AutocompleteType::Keyword => self.autocomplete_keyword(string).iter().cloned().cloned().collect(),
        }; // match

        // For debug builds:
        #[cfg(debug_assertions)]
        tracing::trace!(
            "{} autocomplete options for \"{}\".",
            autocomplete_options.len(),
            string,
        ); // trace!

        autocomplete_options

    } // fn

} // impl