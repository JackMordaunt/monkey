use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct MultiError(Vec<Box<dyn Error>>);

impl MultiError {
    pub fn new() -> MultiError {
        MultiError(Vec::<Box<dyn Error>>::new())
    }
    pub fn push(&mut self, err: Box<dyn Error>) {
        self.0.push(err)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Display for MultiError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for err in &self.0 {
            write!(f, "  -> {}\n", err)?;
        }
        Ok(())
    }
}

impl Error for MultiError {}