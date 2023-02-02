use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn encode(input: &str) {
    let encoded = utf8_percent_encode(input, FRAGMENT).to_string();
    println!("{}", encoded);
}
