use std::{collections::HashMap, hash::Hash};



pub struct HashVec<K, V> {
    keys: HashMap<K, usize>,
    values: Vec<V>,
}

impl<K, V> Default for HashVec<K, V> {
    fn default() -> Self {
        Self { keys: Default::default(), values: Default::default() }
    }
}

impl<K, V> HashVec<K, V>
    where K: Eq + Hash
{
    pub fn new(keys: Vec<K>, values: Vec<V>) -> Self {
        debug_assert_eq!(keys.len(), values.len());

        let keys = keys.into_iter()
            .enumerate()
            .map(|(idx, k)| (k, idx))
            .collect();

        Self {
            keys,
            values,
        }
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // ------------------------------------------------------------------------

    pub fn contains_key(&self, key: &K) -> bool {
        self.keys.contains_key(key)
    }

    pub fn idx_of(&self, key: &K) -> Option<usize> {
        self.keys.get(key).cloned()
    }

    pub fn value(&self, idx: usize) -> Option<&V> {
        self.values.get(idx)
    }

    pub fn value_from_key(&self, key: &K) -> Option<&V> {
        self.value(self.idx_of(key)?)
    }

    pub fn values(&self) -> &[V] {
        &self.values
    }

    // ------------------------------------------------------------------------

    pub fn push(&mut self, key: K, val: V) -> usize {
        debug_assert!(!self.contains_key(&key));

        let idx = self.len();
        self.keys.insert(key, idx);
        self.values.push(val);

        idx
    }

    pub fn push_or_get(&mut self, key: K, val: V) -> usize {
        if let Some(idx) = self.idx_of(&key) { return idx; }

        let idx = self.len();
        self.keys.insert(key, idx);
        self.values.push(val);

        idx
    }
}
