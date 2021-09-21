// -----------------------------------------------------------------------------
//
/// To make a struct indexable, the programmer must implement the `Indexable`
/// trait for it. The trait returns a `Vec<String>` of all content that is to be
/// indexed.

pub trait Indexable {
    fn strings(&self) -> Vec<String>;
} // Indexable