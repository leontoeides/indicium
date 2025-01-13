use beef::lean::Cow;

// -----------------------------------------------------------------------------

impl<K: Ord> crate::simple::SearchIndex<K> {
    /// Returns a normalized string according to the search index's case
    /// sensitivity settings.
    ///
    /// * If the search index case been set to be case sensitive, the string
    ///   will be returned as-is.
    ///
    /// * If the search index case been set to be case sensitive, the string
    ///   will be returned in lower-case form.
    pub(crate) fn normalize<'k>(
        &self,
        keyword: &'k str
    ) -> Cow<'k, str> {
        if self.case_sensitive {
            keyword.into()
        } else {
            keyword.to_lowercase().into()
        } // if
    } // fn
} // impl