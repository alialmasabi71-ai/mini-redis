#[allow(warnings)]

use std::collections::HashMap;
use std::hash::Hash;

use crate::error::{StoreError};
use crate::traits::Serializable;

#[derive(Debug, Default, Clone)]
pub struct Store<K, V>
where
    K: Eq + Hash + Clone,
    V: Serializable + Clone,
{
    inner: HashMap<K, V>,
}

impl<K, V> Store<K, V> where
    K: Eq + Hash + Clone,
    V: Serializable + Clone,
{
    pub fn new() -> Self {
        Store {
            inner: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: K, value: V) -> Result<Option<V>, StoreError> {
        value.validate().map_err(StoreError::ValidationFailed)?;
        Ok(self.inner.insert(key, value))
    }


    pub fn get(&self, key: &K) -> Result<V, StoreError> {
        match self.inner.get(key) {
            Some(value) => Ok(value.clone()),
            None => Err(StoreError::KeyNotFound),
        }
    }

    pub fn get_ref<'a>(&'a self, key: &K) -> Result<&'a V, StoreError> {
        match self.inner.get(key) {
            Some(value) => Ok(value),
            None => Err(StoreError::KeyNotFound),
        }
    }

    pub fn remove(&mut self, key: &K) -> Result<V, StoreError> {
        match self.inner.remove(key) {
            Some(value) => Ok(value),
            None => Err(StoreError::KeyNotFound),
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.inner.contains_key(key)
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a V)> {
        self.inner.iter()
    }
}