mod error;
mod store;
mod traits;

pub use crate::error::{StoreError, ValidationError};
pub use crate::store::Store;
pub use crate::traits::Serializable;

mod tests {
    pub mod basic;
    pub mod custom_types;
    pub mod lifetimes;
    
}