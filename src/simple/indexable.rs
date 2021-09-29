// -----------------------------------------------------------------------------
//
/// To make a record indexable for Indicium, the `Indexable` trait must be
/// implemented for it. The trait returns a `Vec<String>` of all content that is
/// to be indexed.
///
/// ### Implementing Indexable
///
/// Before we can begin using Indicium, we must make our record indexable. We'll
/// do this by implementing the `Indexable` trait for our `struct`. The idea is
/// to return a `String` for every field that we would like to be indexed.
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
    fn strings(&self) -> Vec<String>;
} // Indexable