use std::str;
use std::str::Utf8Error;

pub fn to_str<'a> (v: &'a Vec<u8>) -> Result<&'a str, Utf8Error> {
    str::from_utf8(&v)
}

pub fn to_string<'a> (v: &'a Vec<u8>) -> String {
    v.iter().map(|c| { *c as char }).collect::<String>()
}

pub fn to_array<'a> (v: &'a Vec<u8>) -> &'a [u8] {
    v.as_slice()
}

