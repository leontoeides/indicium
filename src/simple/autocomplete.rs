use crate::simple::{AutocompleteType, SearchIndex};
use std::cmp::Ord;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Returns all matching _autocomplete_ keywords for the provided search
    /// string. The search string may contain several keywords. The last keyword
    /// in the string will be autocompleted. This function will use the
    /// `AutocompleteType` setting stored in the `SearchIndex`.
    ///
    /// For more information about the different types of autocompletion see:
    /// [`AutocompleteType`].
    ///
    /// For more information about the setting the autocompletion type in the
    /// `SearchIndex` type see: [`SearchIndexBuilder`].
    ///
    /// [`AutocompleteType`]: enum.AutocompleteType.html
    /// [`SearchIndexBuilder`]: struct.SearchIndexBuilder.html

    pub fn autocomplete(&self, string: &str) -> Vec<String> {

        match &self.autocomplete_type {
            AutocompleteType::Keyword => self.keyword_autocomplete(string).iter().cloned().cloned().collect(),
            AutocompleteType::Contextual => self.and_autocomplete(string),
            AutocompleteType::Global => self.or_autocomplete(string),
        } // match

    } // fn

    // -------------------------------------------------------------------------
    //
    /// Returns all matching _autocomplete_ keywords for the provided search
    /// string. The search string may contain several keywords. The last keyword
    /// in the string will be autocompleted. This function allows the caller to
    /// override the `AutocompleteType` setting stored in the `SearchIndex`.
    ///
    /// For more information about the different types of autocompletion see:
    /// [`AutocompleteType`].
    ///
    /// For more information about the setting the autocompletion type in the
    /// `SearchIndex` type see: [`SearchIndexBuilder`].
    ///
    /// [`AutocompleteType`]: enum.AutocompleteType.html
    /// [`SearchIndexBuilder`]: struct.SearchIndexBuilder.html

    pub fn autocomplete_type(&self, autocomplete_type: &AutocompleteType, string: &str) -> Vec<String> {

        match autocomplete_type {
            AutocompleteType::Keyword => self.keyword_autocomplete(string).iter().cloned().cloned().collect(),
            AutocompleteType::Contextual => self.and_autocomplete(string),
            AutocompleteType::Global => self.or_autocomplete(string),
        } // match

    } // fn

} // impl