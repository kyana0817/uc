use clap;
use std::{io};
use clap::{Parser, ValueEnum};

use crate::url_convert::encode::encode;
use crate::url_convert::decode::decode;
pub mod url_convert;


#[derive(Parser)]
struct Cli {    
    #[clap(default_value_t = Mode::Encode, value_enum)]
    mode: Mode,
    input: Option<String>
}

#[derive(Clone, ValueEnum)]
enum Mode {
    Encode,
    Decode
}


fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::Encode => {
            match cli.input {
                Some(line) => encode(&line),
                None => io_pipe(Box::new(encode)),
            }
        },
        Mode::Decode => {
            match cli.input {
                Some(line) => decode(&line),
                None => io_pipe(Box::new(decode)),
            }
        }
    };
}

fn io_pipe(f: Box<dyn Fn(&str)>) {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read pipe");
        
        if input == "" {
            break;
        }

        input = input.trim().to_string();
            
        f(&input);
    }
}
