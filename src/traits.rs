use crate::error::ValidationError;
use std::fmt::Debug;
use std::mem;


pub trait Serializable: Debug + Clone {

    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }

    fn to_string_repr(&self) -> String {
        format!("{:?}", self)
    }

    fn size(&self) -> usize {
        self.to_string_repr().len()
    }
}

//-----------------------------------------------------------------------------

impl Serializable for String {
    fn to_string_repr(&self) -> String {
        self.clone()
    }

    fn size(&self) -> usize {
        self.len()
    }
}

impl Serializable for i32 {
    fn to_string_repr(&self) -> String {
        self.to_string()
    }

    fn size(&self) -> usize {
        mem::size_of::<i32>()
    }
}

impl Serializable for u32 {
    fn to_string_repr(&self) -> String {
        self.to_string()
    }

    fn size(&self) -> usize {
        mem::size_of::<u32>()
    }
}

impl Serializable for i64 {
    fn to_string_repr(&self) -> String {
        self.to_string()
    }

    fn size(&self) -> usize {
        mem::size_of::<i64>()
    }
}

impl Serializable for u64 {
    fn to_string_repr(&self) -> String {
        self.to_string()
    }

    fn size(&self) -> usize {
        mem::size_of::<u64>()
    }
}

impl Serializable for f64 {
    fn to_string_repr(&self) -> String {
        self.to_string()
    }

    fn size(&self) -> usize {
        mem::size_of::<f64>()
    }
}

impl Serializable for bool {
    fn to_string_repr(&self) -> String {
        self.to_string()
    }

    fn size(&self) -> usize {
        mem::size_of::<bool>()
    }
}