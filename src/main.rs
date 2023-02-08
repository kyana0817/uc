pub mod converts;
pub mod utils;

pub mod tests;

use crate::converts::url::{encode, decode};
use crate::utils::io_pipe;
use clap;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {    
    #[clap(default_value_t = Mode::Encode, value_enum)]
    mode: Mode,
    input: Option<String>
}

#[derive(Clone, ValueEnum)]
pub enum Mode {
    Encode,
    Decode
}


fn main() {
    let cli = Cli::parse();

    match cli.mode {
        Mode::Encode => {
            match cli.input {
                Some(line) => println!("{}", encode(&line)),
                None => io_pipe(Box::new(encode)),
            }
        },
        Mode::Decode => {
            match cli.input {
                Some(line) => println!("{}", decode(&line)),
                None => io_pipe(Box::new(decode)),
            }
        }
    };
}
