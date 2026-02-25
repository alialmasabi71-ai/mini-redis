#[allow(warnings)]

use crate::{ConcurrentStore, StoreError};
use std::sync::{Arc, Barrier};
use std::thread;

#[test]
fn test_concurrent_set_get() -> Result<(), StoreError> {
    const N: usize = 10;
    let store = Arc::new(ConcurrentStore::<usize, usize>::new());
    let barrier = Arc::new(Barrier::new(N));
    let mut handles = Vec::with_capacity(N);
    for i in 0..N {
        let s = store.clone();
        let b = barrier.clone();
        let value = i+1;
        handles.push(thread::spawn(move || {
            b.wait();
            s.set(i, value).unwrap();
            assert_eq!(s.get(&i).unwrap(), value);
    
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    for i in 0..N {
        assert_eq!(store.get(&i)?, i+1);
        let v = store.get(&i);
        println!("key: {i} value: {:?}", v);
    }
    Ok(())
}

#[test]
fn test_concurrent_remove() -> Result<(), StoreError> {
    let store = Arc::new(ConcurrentStore::<u32, u32>::new());
    store.set(1, 1)?;
    let s1 = store.clone();
    let s2 = store.clone();
    let handle1 = thread::spawn(move || s1.remove(&1).unwrap());
    let handle2 = thread::spawn(move || match s2.get(&1) {
        Ok(_) => panic!("expected key to be removed"),
        Err(StoreError::KeyNotFound) => {println!("Key not found")},
        Err(e) => panic!("unexpected error: {:?}", e),
    });
    assert_eq!(handle1.join().unwrap(), 1);
    handle2.join().unwrap();
    Ok(())
}