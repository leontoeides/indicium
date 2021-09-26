use crate::simple::{SearchIndex, SearchType};
use std::cmp::Ord;

// -----------------------------------------------------------------------------

impl<K: Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Return all matching _typeahead_ or _autocomplete_ keywords for the
    /// provided search string. The search string may contain several keywords.
    /// The last keyword in the string will be autocompleted.

    pub fn autocomplete(&self, string: &str) -> Vec<String> {

        match &self.autocomplete_type {
            SearchType::Keyword => self.keyword_autocomplete(string).iter().cloned().cloned().collect(),
            SearchType::And => self.and_autocomplete(string),
            SearchType::Or => self.or_autocomplete(string),
        } // match

    } // fn

    // -------------------------------------------------------------------------
    //
    /// Return all matching _typeahead_ or _autocomplete_ keywords for the
    /// provided search string. The search string may contain several keywords.
    /// The last keyword in the string will be autocompleted.

    pub fn autocomplete_type(&self, autocomplete_type: &SearchType, string: &str) -> Vec<String> {

        match autocomplete_type {
            SearchType::Keyword => self.keyword_autocomplete(string).iter().cloned().cloned().collect(),
            SearchType::And => self.and_autocomplete(string),
            SearchType::Or => self.or_autocomplete(string),
        } // match

    } // fn

} // impl