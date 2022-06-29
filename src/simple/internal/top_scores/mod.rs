use std::collections::{BTreeSet, HashMap};
use std::{clone::Clone, cmp::Ord, cmp::PartialOrd, hash::Hash};

// -----------------------------------------------------------------------------

#[derive(Default)]
pub(crate) struct TopScores<'a, K: Hash + Ord, S: PartialOrd> {
    /// The top _n_ scores.
    pub(crate) top: HashMap<&'a str, (&'a BTreeSet<K>, S)>,
    /// This fields tracks lowest of the top scores.
    pub(crate) bottom: Option<(&'a str, S)>,
    /// The number of top scores to track.
    pub(crate) capacity: usize,
} // TopScores

// -----------------------------------------------------------------------------

impl<'a, K: Hash + Ord, S: Clone + PartialOrd + std::fmt::Debug> TopScores<'a, K, S> {

    pub(crate) fn with_capacity(capacity: usize) -> TopScores<'a, K, S> {
        TopScores {
            top: HashMap::with_capacity(capacity),
            bottom: None,
            capacity,
        } // TopScores
    } // fn

    pub(crate) fn insert(&mut self, keyword: &'a str, keys: &'a BTreeSet<K>, score: S) {
        if self.top.len() >= self.capacity {
            if self.bottom.is_none() { self.find_bottom() }
            if let Some(bottom) = &self.bottom {
                if score > bottom.1 {
                    self.remove_bottom();
                    self.top.insert(keyword, (keys, score));
                    self.find_bottom();
                } // if
            } // if
        } else {
            self.top.insert(keyword, (keys, score));
        } // if
    } // fn

    pub(crate) fn remove_bottom(&mut self) {
        if let Some(bottom) = &self.bottom {
            self.top.remove(bottom.0);
        } // if
    } // fn

    pub(crate) fn find_bottom(&mut self) {
        self.bottom = self.top
            .iter()
            .min_by(|(_a_keyword, (_a_keys, a_score)), (_b_keyword, (_b_keys, b_score))|
                a_score.partial_cmp(b_score).unwrap()
            ) // min_by
            .map(|(keyword, (_keys, score))| (*keyword, score.clone()));
    } // fn

    pub(crate) fn keywords(self) -> Vec<&'a str> {
        let mut vec: Vec<(&str, S)> = self.top.into_iter().map(|(keyword, (_keys, score))| (keyword, score)).collect();
        vec.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        vec.into_iter().map(|(keyword, _score)| keyword).collect()
    } // if

} // impl