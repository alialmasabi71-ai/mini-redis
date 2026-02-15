use mini_redis::{Store, Serializable, ValidationError, StoreError};

#[derive(Debug, Clone)] 
struct User { 
    id: u64, 
    name: String,
    email: String, 
} 
impl Serializable for User { 
fn validate(&self) -> Result<(), ValidationError> { 
if self.name.is_empty() { 
return Err(ValidationError::EmptyValue); 
} 
Ok(()) 
} 
fn to_string_repr(&self) -> String {
        format!("User {{ id: {}, name: {}, email: {} }}", self.id, self.name, self.email)
    }
}

fn main() -> Result<(), StoreError> {
    let mut store = Store::<u64, User>::new();
    let alice = User {
        id:1,
        name: "ali".into(),
        email: "ali@example,com".into(),
    };
    store.set(alice.id, alice.clone())?;
    let fetch = store.get(&1)?;
    println!("retrived: {}", fetch.to_string_repr());
    Ok(())
}