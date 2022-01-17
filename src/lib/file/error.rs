use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum MyError {
    NotFile,
}

impl Error for MyError {}
impl Display for MyError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "invalid first item to double")
    }
}
