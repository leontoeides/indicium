// -----------------------------------------------------------------------------
//
/// To make a record indexable for Indicium Search, the `Indexable` trait must
/// be implemented for it. The trait returns a `Vec<String>` of all content that
/// is to be indexed.
///
/// To begin, we'll implement the `Indexable` trait for our `struct`. The idea
/// is to return a `String` for every field that we would like to be indexed.
///
/// Basic usage:
///
/// ```rust
/// # use indicium::simple::Indexable;
/// #
/// struct MyStruct {
///     title: String,
///     year: u16,
///     body: String,
/// }
///
/// impl Indexable for MyStruct {
///     fn strings(&self) -> Vec<String> {
///         vec![
///             self.title.clone(),
///             self.year.to_string(),
///             self.body.clone(),
///         ]
///     }
/// }
/// ```
///
/// Don't forget that you may make numbers, numeric identifiers, enums, and
/// other types indexable by converting them to a `String` and including them in
/// the returned `Vec<String>`.
///
/// #### Pro-Tip: Hash Tags
///
/// If you would like to categorize or classify your records, you could employ
/// [hash tags](https://en.wikipedia.org/wiki/Hashtag). The purpose of the
/// _hash tag_ is to prevent category name collisions with user-space keywords.
/// Of course, consider using another symbol if the hash symbol (`#`) is
/// anticipated to be in the user-space.
///
/// You could preprend the user's search query with hash tag(s). For example:
/// you could categorize cities as American by returning the `#US` keyword for
/// them from your `Indexable` trait. Then, if a user intends to search for
/// cities in United States, your application could quietly prepend the user's
/// query with `#US` to restrict the search to American cities.

pub trait Indexable {
    /// Returns a string for every field for a record that is to be indexed by
    /// Indicium Search.
    fn strings(&self) -> Vec<String>;
} // Indexable