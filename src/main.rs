#![allow(dead_code)]

mod token;
mod lexer;
mod ast;
mod parser;
mod repl;
mod util;

use whoami;
use colored::*;
use std::io;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();
    println!("Hello {}! This is the Monkey programming language.", whoami::username());
    println!("Feel free to type in commands.");
    if let Err(err) = repl::start(&mut stdin, &mut stdout) {
        println!("\n{} \n{}", "error".red(), err);
        std::process::exit(0);
    }
}
