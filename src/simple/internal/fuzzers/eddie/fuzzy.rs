#![allow(clippy::inline_always)]

use crate::simple::search_index::SearchIndex;
use kstring::KString;
use std::{collections::BTreeSet, hash::Hash};

// -----------------------------------------------------------------------------
//
/// This `struct` is used to access the
/// [eddie](https://crates.io/crates/eddie) crate in a generic manner.
pub struct Eddie;

// -----------------------------------------------------------------------------
//
/// This `trait` implementation is used to access the
/// [eddie](https://crates.io/crates/eddie) crate in a generic manner.
impl<'s, K: Hash + Ord> crate::simple::internal::fuzzers::Fuzzy<'s, K> for Eddie {
    fn autocomplete_keyword(
        search_index: &'s crate::simple::search_index::SearchIndex<K>,
        last_autocomplete_options: &mut Vec<&'s str>,
        keyword: &str,
    ) {
        if last_autocomplete_options.is_empty() {
            // No autocomplete options were found for the user's last
            // (partial) keyword. Attempt to use fuzzy string search to find
            // other autocomplete options:
            let fuzzy_search_results: Vec<&str> = search_index
                .eddie_keyword(keyword)
                // The above method returns both the keyword and the keys. We're
                // searching for keywords, so discard the keys.
                .map(|(keyword, _keys)| keyword)
                // Convert `&KString` to `&str`:
                .map(kstring::KStringBase::as_str)
                // Collect all keyword autocompletions into a `Vec`:
                .collect();

            *last_autocomplete_options = fuzzy_search_results;
        } // if
    } // fn

    #[inline(always)]
    fn autocomplete_global(
        search_index: &'s SearchIndex<K>,
        preceding_keywords: &[KString],
        last_autocomplete_options: &mut Vec<&'s KString>,
        last_keyword: &str,
    ) {
        if last_autocomplete_options.is_empty() {
            // No search results were found for the user's last (partial)
            // keyword. Attempt to use fuzzy string search to find other
            // options:
            let fuzzy_search_results: Vec<&KString> =
                search_index.eddie_global(
                    preceding_keywords,
                    last_keyword
                )
                // The above method returns both the keyword and the keys. We're
                // searching for keywords, so discard the keys.
                .map(|(keyword, _keys)| keyword)
                // Collect all keyword autocompletions into a `Vec`:
                .collect();

            *last_autocomplete_options = fuzzy_search_results;
        } // if
    } // fn

    #[inline(always)]
    fn autocomplete_context(
        search_index: &'s SearchIndex<K>,
        preceding_keywords: &[KString],
        preceding_results: &BTreeSet<&'s K>,
        last_keyword: &str,
        last_autocomplete_options: &mut Vec<&'s KString>,
    ) {
        if last_autocomplete_options.is_empty() {
            // No search results were found for the user's last (partial)
            // keyword. Attempt to use fuzzy string search to find other
            // options:
            let fuzzy_search_results: Vec<&KString> =
                search_index.eddie_context(
                    preceding_keywords,
                    preceding_results,
                    last_keyword
                )
                // The above method returns both the keyword and the keys. We're
                // searching for keywords, so discard the keys.
                .map(|(keyword, _keys)| keyword)
                // Collect all keyword autocompletions into a `Vec`:
                .collect();

            *last_autocomplete_options = fuzzy_search_results;
        } // if
    } // fn

    #[inline(always)]
    fn live_search_keyword(
        search_index: &'s SearchIndex<K>,
        search_results: &mut BTreeSet<&'s K>,
        user_keyword: &str,
    ) {
        if search_results.is_empty() {
            // No search results were found for the user's last (partial)
            // keyword. Attempt to use fuzzy string search to find other
            // options:
            let fuzzy_search_results: BTreeSet<&K> = search_index
                .eddie_keyword(user_keyword)
                // The above method returns both the keyword and the keys. We're
                // searching for keys, so discard the keywords.
                .flat_map(|(_keyword, keys)| keys)
                // Only return `maximum_search_results` number of keys.
                //
                // Note that the above method returns
                // `maximum_autocomplete_options` items. However each item which
                // is represented by a single keyword and multiple keys. The
                // keys could expand into more than `maximum_search_results`
                // keys. This additional filter is required.
                .take(search_index.maximum_search_results)
                // Collect all fuzzy key results into a `BTreeSet`:
                .collect();

            *search_results = fuzzy_search_results;
        } // if
    } // fn

    #[inline(always)]
    fn live_search_context(
        search_index: &'s SearchIndex<K>,
        preceding_results: &BTreeSet<&'s K>,
        preceding_keywords: &[KString],
        last_results: &mut BTreeSet<&'s K>,
        last_keyword: &str,
    ) {
        if last_results.is_empty() {
            // No search results were found for the user's last (partial)
            // keyword. Attempt to use fuzzy string search to find other
            // options:
            let fuzzy_search_results: BTreeSet<&K> =
                search_index.eddie_context(
                    preceding_keywords,
                    preceding_results,
                    last_keyword
                )
                // The above method returns both the keyword and the keys. We're
                // searching for keys, so discard the keywords.
                .flat_map(|(_keyword, keys)| keys)
                // Only return `maximum_search_results` number of keys.
                //
                // Note that the above method returns
                // `maximum_autocomplete_options` items. However each item which
                // is represented by a single keyword and multiple keys. The
                // keys could expand into more than `maximum_search_results`
                // keys. This additional filter is required.
                .take(search_index.maximum_search_results)
                // Collect all fuzzy key results into a `BTreeSet`:
                .collect();

            *last_results = fuzzy_search_results;
        } // if
    } // fn
} // trait Fuzzy