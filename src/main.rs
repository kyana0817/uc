use clap;
use std::{io};
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS, percent_decode_str};
use clap::{Parser, ValueEnum};


const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

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


fn encode(input: &str) {
    let encoded = utf8_percent_encode(input, FRAGMENT).to_string();
    println!("{}", encoded);
}

fn decode(input: &str) {
    let decoded = percent_decode_str(input).decode_utf8().expect("decode error");
    println!("{}", decoded);
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
