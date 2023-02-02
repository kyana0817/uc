use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn encode(input: &str) -> String {
    return utf8_percent_encode(input, FRAGMENT).to_string();
}


pub fn decode(input: &str) -> String {
    return percent_decode_str(input).decode_utf8().expect("decode error").to_string();
}
