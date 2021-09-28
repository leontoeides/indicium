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
    /// `SearchIndex`. Partial keywords must be an exact match.
    ///
    /// Autocompletion behaviour can be changed by setting the
    /// [`AutocompleteType`] in the `SearchIndex`. See also:
    /// [`SearchIndexBuilder`] and [`SearchIndex::new()`].
    ///
    /// [`AutocompleteType`]: enum.AutocompleteType.html
    /// [`SearchIndexBuilder`]: struct.SearchIndexBuilder.html
    /// [`SearchIndex::new()`]: struct.SearchIndex.html#method.new

    pub fn autocomplete(&self, string: &str) -> Vec<String> {

        match &self.autocomplete_type {
            AutocompleteType::Context => self.autocomplete_context(string),
            AutocompleteType::Global => self.autocomplete_global(string),
            AutocompleteType::Keyword => self.autocomplete_keyword(string).iter().cloned().cloned().collect(),
        } // match

    } // fn

    // -------------------------------------------------------------------------
    //
    /// Returns matching autocompleted keywords for the provided search string.
    /// Partial keywords must be an exact match.
    ///
    /// Autocomplete behaviour can be changed by using different
    /// `AutocompleteType` variants as the first parameter for the method call.
    /// See [`AutocompleteType`] for more information on the different
    /// autocomplete types.
    ///
    /// [`AutocompleteType`]: enum.AutocompleteType.html

    pub fn autocomplete_type(&self, autocomplete_type: &AutocompleteType, string: &str) -> Vec<String> {

        match autocomplete_type {
            AutocompleteType::Context => self.autocomplete_context(string),
            AutocompleteType::Global => self.autocomplete_global(string),
            AutocompleteType::Keyword => self.autocomplete_keyword(string).iter().cloned().cloned().collect(),
        } // match

    } // fn

} // impl