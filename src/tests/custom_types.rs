use crate::{Store, Serializable, ValidationError, StoreError};

#[derive(Debug, Clone, PartialEq, Eq)]
struct User {
    id: u64,
    name: String,
    email: String,
}

impl Serializable for User {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.name.is_empty() {
            Err(ValidationError::EmptyValue)
        } else {
            Ok(())
        }
    }
    fn to_string_repr(&self) -> String {
        format!("User {{ id: {}, name: {}, email: {} }}",self.id , self.name, self.email)
    }
}

#[test]
fn test_pass() -> Result<(), StoreError> {
    let mut store = Store::<u64, User>::new();
    let user = User {
        id: 1,
        name: "Ali".into(),
        email: "ali@example.com".into(),
    };
    store.set(user.id, user.clone())?;
    assert_eq!(store.get(&1)?, user);
    Ok(())
}

#[test]
fn test_validation_failed() {
    let mut store = Store::<u64, User>::new();
    let user2 = User {
        id: 2,
        name: "ali".into(),
        email: "".into(),
    };
    match store.set(user2.id, user2) {
        Err(StoreError::ValidationFailed(ValidationError::EmptyValue)) => (),
        res => panic!("expected validation error, got {:?}", res),
    }
}