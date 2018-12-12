use std::io::{self, BufRead, Write};
use std::collections::HashMap;

use excel::CellType;
use excel::Command;
use excel::display_error;

fn main() {
    let storage: HashMap<(char, i32), CellType> = HashMap::new();
    let stdin = io::stdin();

    loop {
        print!(">>> ");
        io::stdout().flush().ok().expect("Could not flush stdout");

        let mut buf = String::new();

        stdin
            .lock()
            .read_line(&mut buf)
            .expect("Could not read from stdin");

        if buf.len() == 0 {
            println!();
            break;
        }

        match buf.parse() {
            Ok(command) => {
                if let Command::Exit = command {
                    break;
                }
            },
            Err(error) => println!("Error: {}", display_error(error)),
        }
    }
}
