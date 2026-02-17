use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

use crate::error::StoreError;
use crate::traits::Serializable;


#[derive(Debug, Clone)]
pub struct ConcurrentStore<K, V>
where
    K: Eq + Hash + Clone + Send + Sync,
    V: Serializable + Clone + Send + Sync,
{
    inner: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> ConcurrentStore<K, V>
where
    K: Eq + Hash + Clone + Send + Sync,
    V: Serializable + Clone + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }


    pub fn set(&self, key: K, value: V) -> Result<Option<V>, StoreError> {
        value.validate().map_err(StoreError::ValidationFailed)?;
        let mut guard = self.inner.write().map_err(StoreError::from)?;
        Ok(guard.insert(key, value))
    }

    pub fn get(&self, key: &K) -> Result<V, StoreError> {
        let guard = self.inner.read().map_err(StoreError::from)?;
        match guard.get(key) {
            Some(val) => Ok(val.clone()),
            None => Err(StoreError::KeyNotFound),
        }
    }


    pub fn get_ref(&self, key: &K) -> Result<V, StoreError> {
        self.get(key)
    }

    pub fn remove(&self, key: &K) -> Result<V, StoreError> {
        let mut guard = self.inner.write().map_err(StoreError::from)?;
        match guard.remove(key) {
            Some(val) => Ok(val),
            None => Err(StoreError::KeyNotFound),
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        match self.inner.read() {
            Ok(guard) => guard.contains_key(key),
            Err(_) => false,
        }
    }


    pub fn iter(&self) -> Result<Vec<(K, V)>, StoreError> {
        let guard = self.inner.read().map_err(StoreError::from)?;
        Ok(guard.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
    }
}