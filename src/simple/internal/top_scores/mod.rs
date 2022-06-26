use std::collections::{BTreeSet, HashMap};
use std::{clone::Clone, cmp::Ord, cmp::PartialOrd, fmt::Debug, hash::Hash};

// -----------------------------------------------------------------------------

#[derive(Debug, Default)]
pub(crate) struct TopScores<'a, K: Hash + Ord, S: Debug + PartialOrd> {
    pub(crate) top: HashMap<&'a str, (&'a BTreeSet<K>, S)>,
    pub(crate) bottom: Option<(&'a str, S)>,
    pub(crate) capacity: usize,
} // TopScores

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord, S: Clone + Debug + PartialOrd> TopScores<'a, K, S> {

    pub(crate) fn with_capacity(capacity: usize) -> TopScores<'a, K, S> {
        TopScores {
            top: HashMap::new(),
            bottom: None,
            capacity,
        } // TopScores
    } // fn

    pub(crate) fn insert(&mut self, keyword: &'a str, keys: &'a BTreeSet<K>, score: S) {
        if self.top.len() >= self.capacity {
            if self.bottom.is_none() { self.find_bottom() }
            if let Some(bottom) = &self.bottom {
                if score > bottom.1 {
                    self.top.insert(keyword, (keys, score));
                    self.remove_bottom();
                    self.find_bottom();
                } // if
            } // if
        } else {
            self.top.insert(keyword, (keys, score));
        } // if
    } // fn

    fn remove_bottom(&mut self) {
        if let Some(bottom) = &self.bottom { self.top.remove(bottom.0); }
    } // fn

    fn find_bottom(&mut self) {
        self.bottom = self.top
            .iter()
            .min_by(|(_a_keyword, a_score), (_b_keyword, b_score)|
                a_score.partial_cmp(b_score).unwrap()
            ) // min_by
            .map(|(keyword, (_keys, score))| (*keyword, score.clone()))
    } // fn

    pub(crate) fn keywords(self) -> Vec<&'a str> {
        self.top
            .into_iter()
            .map(|(keyword, _score)| keyword)
            .collect()
    } // if

} // impl