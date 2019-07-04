use std::str;
use std::str::Utf8Error;

pub fn to_str<'a> (v: &'a Vec<u8>) -> Result<&'a str, Utf8Error> {
    str::from_utf8(&v)
}

pub fn to_string(v: Vec<u8>) -> String {
    let s: String = v.iter().map(|c| {
            let x = format!("{:x}", c);
            x
            }).collect();

    s
}

pub fn to_array<'a> (v: &'a Vec<u8>) -> &'a [u8] {
    v.as_slice()
}

