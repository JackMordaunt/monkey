mod lexer;
mod repl;

use std::io;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();
    // let mut line = String::new();
    // loop {
    //     line.clear();
    //     print!("{} ", PROMPT); stdout.flush()?;
    //     stdin.read_line(&mut line)
    //         .map_err(|err| format!("reading line: {}", err))?;
    //     for token in Lexer::new(line.chars()) {
    //         println!("{:?}", token);
    //     }
    // }
    repl::start(&mut stdin, &mut stdout)
        .map_err(|err| format!("repl: {}", err))?;
    Ok(())
}
