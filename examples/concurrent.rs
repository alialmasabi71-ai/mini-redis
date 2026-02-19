use std::sync::Arc;
use std::thread;
use mini_redis::{ConcurrentStore, StoreError};

fn main() -> Result<(), StoreError> {
    let store = Arc::new(ConcurrentStore::<u32, u32>::new());
    let mut handles = Vec::new();
    for i in 0u32..20 {
        let s = Arc::clone(&store);
        handles.push(thread::spawn(move || {
            s.set(i, i).unwrap();
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    for i in 0..20 {
        let v = store.get(&i)?;
        println!("key {} has value {}", i, v);
    }
    Ok(())
}