use percent_encoding::{percent_decode_str};

pub fn decode(input: &str) {
    let decoded = percent_decode_str(input).decode_utf8().expect("decode error");
    println!("{}", decoded);
}
