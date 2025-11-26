#[cfg(feature = "unicode-normalization")]
use unicode_normalization::UnicodeNormalization;

impl<K: Ord> crate::simple::SearchIndex<K> {
    /// Returns a normalized string according to the search index's settings.
    ///
    /// Normalization ensures consistent matching by canonicalizing Unicode
    /// representations and optionally folding case. This allows searches to
    /// match equivalent characters (like "ﬁ" and "fi") regardless of how
    /// they were encoded.
    ///
    /// When the `unicode-normalization` feature is enabled, NFKC normalization
    /// is applied to decompose compatibility characters into their canonical
    /// forms. When case insensitivity is also enabled, the string is
    /// additionally lowercased.
    ///
    /// * If the search index case been set to be case sensitive, the string
    ///   will be returned as-is.
    ///
    /// * If the search index case been set to be case insensitive, the string
    ///   will be returned in lower-case form.
    #[inline]
    pub(crate) fn normalize<'k>(
        &self,
        keyword: &'k str
    ) -> beef::lean::Cow<'k, str> {
        if self.case_sensitive {
            #[cfg(feature = "unicode-normalization")]
            let normalized = keyword.nfkc().collect::<String>().into();

            #[cfg(not(feature = "unicode-normalization"))]
            let normalized = keyword.into();

            normalized
        } else {
            #[cfg(feature = "unicode-normalization")]
            let normalized = keyword.nfkc().collect::<String>().to_lowercase().into();

            #[cfg(not(feature = "unicode-normalization"))]
            let normalized = keyword.to_lowercase().into();

            normalized
        } // if
    } // fn
} // impl