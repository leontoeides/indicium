impl<K: Ord> crate::simple::SearchIndex<K> {
    /// Clears the search index, removing all elements.
    pub fn clear(&mut self) {
        self.b_tree_map.clear();
    } // fn
} // impl