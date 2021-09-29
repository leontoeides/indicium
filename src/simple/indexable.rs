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

pub trait Indexable {
    /// Returns a string for every field for a record that is to be indexed by
    /// Indicium Search.
    fn strings(&self) -> Vec<String>;
} // Indexable