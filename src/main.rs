use std::{env, io};
use text_colorizer::*;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

struct Argument {
    input: Option<String>,
}

fn main() {
    let args: Argument = parce_args();

    match args.input {
        Some(input) => encode(&input),
        None => io_pipe(),
    }
}

fn parce_args() -> Argument {
    let args = env::args().nth(1);

    Argument {
        input: args,
    }
}

fn encode(input: &str) {
    let encoded = utf8_percent_encode(input, FRAGMENT).to_string();
    println!("{}", encoded);
}

fn io_pipe() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read pipe");
        
        if input == "" {
            break;
        }

        input = input.trim().to_string();
            
        encode(&input);
    }
}
