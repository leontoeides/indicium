# What's New?

* Release notes are available on
[GitHub](https://github.com/leontoeides/indicium/releases).

# 0.6.3

* Removed unnecessary `collect`s on some iterators. Thank you for pointing this
  out, `clippy`. This should provide a small performance improvement.

# 0.6.2

* Corrected a [panic on UTF-8
  searches](https://github.com/leontoeides/indicium/issues/2).

# 0.6.1

* Removed `eddie` as the default string similarity crate, for now, due to a
  potential `panic`.

# 0.6.0

* Fix for contextual fuzzy matching for `Live` interactive searches. In some
  cases `Live` search would return global results without properly observing the
  `maximum_search_results` setting. This has been fixed. This will improve
  performance and user experience.

* New, optional `eddie` feature which is turned on by default. When this feature
  is enabled, this library will utilize [Ilia Schelokov](https://github.com/thaumant)'s
  [eddie](https://lib.rs/crates/eddie) crate for
  [faster](https://github.com/thaumant/eddie/blob/master/benchmarks.md)
  UTF-8 string distance and string similarity calculations.

* New, optional `gxhash` feature. `ahash` is still the default hasher. When this
  feature is enabled, this library will utilize
  [Olivier Giniaux](https://github.com/ogxd)'s bleeding edge
  [gxhash](https://lib.rs/crates/gxhash) crate for faster `HashMap` and `HashSet`
  operations.

# 0.5.2

* New, optional `ahash` feature which is turned on by default. When this feature
  is enabled, this library will utilize [Tom Kaitchuck](https://crates.io/users/tkaitchuck)'s
  [ahash](https://lib.rs/crates/ahash) crate for faster `HashMap` and `HashSet`
  operations rather than using the standard library's SipHash.

# 0.5.1

* Fixes compile failure for the experimental `select2` feature and when not
  using `fuzzy` feature.

# 0.5.0

* The `simple` search index now internally employs
  [Ed Page](https://github.com/epage)'s [kstring](https://crates.io/crates/kstring)
  crate.

* The `dump_keyword` and `profile` function signatures were changed slightly,
  otherwise there's no expected impact to callers.

* Performance improvements.

# 0.4.2

* Any type that implements
  [ToString](https://doc.rust-lang.org/std/string/trait.ToString.html) (and
  consequently any type that implements
  [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html))
  now gets the
  [Indexable](https://docs.rs/indicium/latest/indicium/simple/trait.Indexable.html)
  implementation for free.

# 0.4.1

* Improved contextual fuzzy matching.

# 0.4.0

* Initial support for fuzzy searching. Fuzzy matching is applied to the last
  (partial) keyword in the search string for _autocompletion_ and _live search_
  only.

  Keywords at the start or in the middle of the user's search string will not be
  substituted. Indicium uses [Danny Guo](https://github.com/dguo)'s
  [strsim](https://crates.io/crates/strsim) crate for string similarity
  calculations.

* Breaking changes:
	* Builder pattern is now passed owned values.
	* `K` key type requires `Hash` trait for `fuzzy` string search feature.
	* New `SearchIndex` default settings.

* **Any dependent software should see if (or how) the updated defaults change
  search behaviour and tweak accordingly before adopting the 0.4.0 update.**

# 0.3.7

* An experimental feature is now disabled by default to reduce resource
  consumption.

# 0.3.6

* Implemented `DerefMut` which gives access to the search index's underlying
  `BTreeMap`. Implemented the `clear()` method for the seach index which is a
  convenience method for clearing the search index.

# 0.3.5

* Peformance improvements.

# 0.3.4

* Peformance improvements.

# 0.3.3

* Fix: `cargo test` failed. Sorry.

# 0.3.2

* Fix: issue with search indexes that do not use keyword splitting.

# 0.3.1

* Autocomplete no longer offers previously used keywords as options.

* Added `maximum_keys_per_keyword` getter method.

* Added `autocomplete_with` and `search_with` methods which allow ad-hoc
  overrides of the `AutocompleteType`/`SearchType` and maximum results
  parameters.

# 0.3.0

* Added new search type `SearchType::Live` which is for "search as you type"
  interfaces. It is sort of a hybrid between `autocomplete` and
  `SearchType::And`. It will search using an (incomplete) string and return keys
  as the search results. Each resulting key can then be used to retrieve the
  full record from its collection to be rendered & displayed to the user.