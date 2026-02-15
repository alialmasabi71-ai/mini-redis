use crate::{Store, StoreError, Serializable, ValidationError};
use proptest::prelude::*;

#[test]
fn test_set_get_remove_basic() -> Result<(), StoreError> {
    let mut store = Store::<String, i32>::new();

    assert_eq!(store.set("a".to_string(), 1)?, None);

    assert_eq!(store.set("a".to_string(), 2)?, Some(1));

    assert_eq!(store.get(&"a".to_string())?, 2);

    assert!(store.contains_key(&"a".to_string()));
    
    assert_eq!(store.remove(&"a".to_string())?, 2);

    match store.get(&"a".to_string()) {
        Err(StoreError::KeyNotFound) => (),
        res => panic!("expected KeyNotFound, got {:?}", res),
    }
    Ok(())
}

#[test]
fn test_get_ref() -> Result<(), StoreError> {
    let mut store = Store::<String, i32>::new();
    store.set("x".into(), 5)?;
    let r = store.get_ref(&"x".to_string())?;
    assert_eq!(*r, 5);
    // mutate the store
    store.set("x".into(), 6)?;
    assert_eq!(store.get_ref(&"x".to_string())?.clone(), 6);
    Ok(())
}

#[derive(Debug, Clone)]
struct NotEmpty(String);

impl Serializable for NotEmpty {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.0.is_empty() {
            Err(ValidationError::EmptyValue)
        } else {
            Ok(())
        }
    }
    fn to_string_repr(&self) -> String {
        self.0.clone()
    }
}

#[test]
fn test_validation_failed() {
    let mut store = Store::<String, NotEmpty>::new();
    match store.set("k".to_string(), NotEmpty("".to_string())) {
        Err(StoreError::ValidationFailed(ValidationError::EmptyValue)) => (),
        res => panic!("expected ValidationFailed(EmptyValue), got {:?}", res),
    }
}

#[test]
fn test_iter() -> Result<(), StoreError> {
    let mut store = Store::<String, i32>::new();
    store.set("x".into(), 1)?;
    store.set("y".into(), 2)?;

    let mut pairs: Vec<(String, i32)> = store
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    pairs.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(pairs, vec![("x".to_string(), 1), ("y".to_string(), 2)]);
    Ok(())
}

proptest! {
    //inserting then retrieving a key yields the same value.
    #[test]
    fn set_get(key in "[a-zA-Z0-9]{1,8}", value in any::<i32>()) {
        let mut store = Store::<String, i32>::new();
        store.set(key.clone(), value).unwrap();
        prop_assert_eq!(store.get(&key).unwrap(), value);
    }

    //removing a key then getting it, results in KeyNotFound.
    #[test]
    fn remove_get(key in "[a-zA-Z0-9]{1,8}", value in any::<i32>()) {
        let mut store = Store::<String, i32>::new();
        store.set(key.clone(), value).unwrap();
        store.remove(&key).unwrap();

        match store.get(&key) {
            Err(StoreError::KeyNotFound) => prop_assert!(true),
            res => prop_assert!(false, "expected KeyNotFound, got {:?}", res),
        }
    }
}