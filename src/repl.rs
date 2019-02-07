use crate::lexer::Lexer;

use std::io::prelude::*;
use std::error::Error;

const PROMPT: &'static str = ">>";

pub fn start<R, W>(r: &mut R, w: &mut W) -> Result<(), Box<dyn Error>> 
    where R: BufRead, W: Write,
{
    let mut line = String::new();
    loop {
        line.clear();
        write!(w, "{} ", PROMPT)?; w.flush()?;
        r.read_line(&mut line)
            .map_err(|err| format!("reading line: {}", err))?;
        for token in Lexer::new(line.chars()) {
            println!("{:?}", token);
        }
    }
}