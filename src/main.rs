use std::{env, io};
use text_colorizer::*;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

struct Argument {
    mode: String,
    input: Option<String>,
}

fn main() {
    let args: Argument = parce_args();
    let encoded = encode(&args.input
        .expect("error not input")
    );

    println!("{}", args.mode);
    println!("encode: {}", encoded);
}

fn parce_args() -> Argument {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 2 {
        eprintln!("{} wrong number of arguments: expected 4, get {}", "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Argument {
        mode: args[0].clone(),
        input: Some(args[1].clone()),
    }
}

fn encode(input: &str) -> String {
    utf8_percent_encode(input, FRAGMENT).to_string()
}

fn io_pipe() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read pipe");
    
        input = input.trim().to_string();
    
        if input == "" {
            break;
        }
        println!("Pipe output: {}", input);
    }
}
