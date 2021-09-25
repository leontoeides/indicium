use crate::simple::conjunction::Conjunction;
use crate::simple::search_index::SearchIndex;
use std::cmp::Ord;
use std::fmt::Debug;

// -----------------------------------------------------------------------------

impl<K: Debug + Ord> SearchIndex<K> {

    // -------------------------------------------------------------------------
    //
    /// Return all matching _typeahead_ or _autocomplete_ keywords for the
    /// provided search string. The search string may contain several keywords.
    /// The last keyword in the string will be autocompleted.

    pub fn autocomplete(&self, string: &str) -> Vec<String> {

        match &self.conjunction {
            Conjunction::And => self.and_autocomplete(string),
            Conjunction::Or => self.or_autocomplete(string),
        } // match

    } // fn

} // impl