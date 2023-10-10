# What's New?

* Release notes are available on
[GitHub](https://github.com/leontoeides/indicium/releases).

* `0.5.2`: New optional `ahash` feature which is turned on by default. When this
feature is enabled, this library will utilize
[ahash](https://lib.rs/crates/ahash) for faster `HashMap` and `HashSet` hashing
rather than the standard library's SipHash.

* `0.5.1`: Fixes compile failure for the experimental `select2` feature and when
not using `fuzzy` feature.

* `0.5.0`: The `simple` search index now internally employs the
[kstring](https://crates.io/crates/kstring) crate. This should help with
performance in certain cases. The `dump_keyword` and `profile` function
signatures were changed slightly, otherwise there's no impact to the caller.

* `0.5.0`: Performance improvements.

* `0.4.2`: Any type that implements
[ToString](https://doc.rust-lang.org/std/string/trait.ToString.html) (and
consequently any type that implements
[Display](https://doc.rust-lang.org/std/fmt/trait.Display.html))
now gets the
[Indexable](https://docs.rs/indicium/latest/indicium/simple/trait.Indexable.html)
implementation for free.

* `0.4.1`: Improved contextual fuzzy matching.

* `0.4.0`: Initial support for fuzzy searching. Fuzzy matching is applied to the
last (partial) keyword in the search string for _autocompletion_ and _live
search_ only. Keywords at the start or in the middle of the user's search string
will not be substituted.
	* Some changes for an upcoming `0.5.0` release are being considered.
	This release could have some changes that would allow `indicium` to
	provide feedback to the user, including which keywords have been
	substituted.

* `0.4.0`: Breaking changes:
	* Builder pattern is now passed owned values.
	* `K` key type requires `Hash` trait for `fuzzy` string search feature.
	* New `SearchIndex` default settings.

* `0.4.0`: **Any dependent software should see if (or how) the updated defaults
change search behaviour and tweak accordingly before adopting the 0.4.0
update.**

* `0.3.7`: An experimental feature is now disabled by default to reduce resource
consumption.

* `0.3.6`: Implemented `DerefMut` which gives access to the search index's
underlying `BTreeMap`. Implemented `clear()` which is a convenience method for
clearing the search index.

* `0.3.5`: Peformance improvements.

* `0.3.4`: Peformance improvements.

* `0.3.3`: Fix: `cargo test` failed. Sorry.

* `0.3.2`: Fix: issue with search indexes that do not use keyword splitting.

* `0.3.1`: Autocomplete no longer offers previously used keywords as options.

* `0.3.1`: Added `maximum_keys_per_keyword` getter method.

* `0.3.1`: Added `autocomplete_with` and `search_with` methods which allow
ad-hoc overrides of the `AutocompleteType`/`SearchType` and maximum results
parameters.

* `0.3.0`: Added new search type `SearchType::Live` which is for "search as you
type" interfaces. It is sort of a hybrid between `autocomplete` and
`SearchType::And`. It will search using an (incomplete) string and return keys
as the search results. Each resulting key can then be used to retrieve the full
record from its collection to be rendered & displayed to the user.