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
        handles.push(thread::spawn(move || {
            b.wait();
            s.set(i, i).unwrap();
            assert_eq!(s.get(&i).unwrap(), i);
    
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    for i in 0..N {
        assert_eq!(store.get(&i)?, i);
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
        Err(StoreError::KeyNotFound) => (),
        Err(e) => panic!("unexpected error: {:?}", e),
    });
    assert_eq!(handle1.join().unwrap(), 1);
    handle2.join().unwrap();
    Ok(())
}