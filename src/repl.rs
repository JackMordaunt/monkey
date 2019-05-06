use crate::lexer::Lexer;
use crate::parser::Parser;

use std::io::prelude::*;
use std::error::Error;
use colored::*;

const PROMPT: &'static str = ">>";

pub fn start<R, W>(r: &mut R, w: &mut W) 
    where R: BufRead, W: Write,
{
    let mut line = String::new();
    loop {
        line.clear();
        if let Err(err) = input(r, w, &mut line) {
            println!("{}: {}", "input".red(), err);
        };
        match Parser::new(Lexer::new(line.chars())).parse() {
            Ok(program) => println!("{}", program),
            Err(err) => println!("\n{} \n{}", "error".red(), err),
        };
    }
}

// Display prompt and read line of input.
fn input<R, W>(r: &mut R, w: &mut W, line_buffer: &mut String) -> Result<(), Box<dyn Error>>
    where R: BufRead, W: Write,
{
    write!(w, "{} ", PROMPT)?; w.flush()?;
    r.read_line(line_buffer)?;
    Ok(())
}