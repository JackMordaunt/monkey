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
        write!(f, "[\n")?;
        for (ii, err) in self.0.iter().enumerate() {
            write!(f, "\t{}", err)?;
            if ii < self.0.len() -1 {
                write!(f, ",")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "]")
    }
}

impl Error for MultiError {}