//// Diff contains the index and left/right values that are different.
type Diff<'a, 'b, T> = Vec<(usize, Option<&'a T>, Option<&'b T>)>;

// Compute the difference between two slices.
pub fn diff<'a, 'b, T>(left: &'a [T], right: &'b [T]) -> Diff<'a, 'b, T>
    where T: PartialEq
{
    let mut diff = vec![];
    let min = std::cmp::min(left.len(), right.len());
    for ii in 0..min {
        if &left[ii] != &right[ii] {
            diff.push((ii, Some(&left[ii]), Some(&right[ii])));
        }
    }
    if left.len() > right.len() {
        for ii in 0..left.len()-right.len() {
            diff.push((min+ii, Some(&left[min+ii]), None))
        }
    }
    if left.len() < right.len() {
        for ii in 0..right.len()-left.len() {
            diff.push((min+ii, None, Some(&right[min+ii])))
        }
    }
    return diff;
}

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
            write!(f, "\t{}\n", err)?;
            if ii < self.0.len() -1 {
                write!(f, ",")?;
            }
        }
        write!(f, "]")
    }
}

impl Error for MultiError {}