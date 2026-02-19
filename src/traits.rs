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

impl Serializable for f32 {
    fn to_string_repr(&self) -> String {
        self.to_string()
    }

    fn size(&self) -> usize {
        mem::size_of::<f32>()
    }
}

impl Serializable for usize {
    fn to_string_repr(&self) -> String {
        self.to_string()
    }

    fn size(&self) -> usize {
        mem::size_of::<usize>()
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

impl<T> Serializable for Vec<T>
where
    T: Serializable + Debug + Clone,
{
    fn validate(&self) -> Result<(), ValidationError> {
        for item in self {
            item.validate()?;
        }
        Ok(())
    }

    fn to_string_repr(&self) -> String {
        let mut out = String::from("[");
        let mut first = true;
        for item in self {
            if !first {
                out.push_str(", ");
            }
            out.push_str(&item.to_string_repr());
            first = false;
        }
        out.push(']');
        out
    }

    fn size(&self) -> usize {
        self.iter().map(|item| item.size()).sum()
    }
}

impl<T> Serializable for Option<T>
where
    T: Serializable + Debug + Clone,
{
    fn validate(&self) -> Result<(), ValidationError> {
        match self {
            Some(val) => val.validate(),
            None => Ok(()),
        }
    }

    fn to_string_repr(&self) -> String {
        match self {
            Some(val) => val.to_string_repr(),
            None => "None".to_string(),
        }
    }

    fn size(&self) -> usize {
        match self {
            Some(val) => val.size(),
            None => 0,
        }
    }
}