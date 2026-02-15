use mini_redis::{Store, StoreError}; 
 
fn main() -> Result<(), StoreError> { 
    let mut store = Store::<String, i32>::new(); 
     
    store.set("counter".to_string(), 42)?; 
    let counter = store.get(&"counter".to_string())?; 
    println!("Counter:  {}", counter);
    Ok(()) 
}